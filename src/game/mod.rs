mod physics;
mod ship;
mod asteroid;
mod bullet;
mod ai;
mod saucer;

use crate::renderer::{Vertex, Color};
pub use ship::Ship;
pub use asteroid::Asteroid;
pub use bullet::Bullet;
pub use saucer::{Saucer, SaucerSize};

pub struct GameState {
    pub player_ship: Ship,
    pub saucers: Vec<Saucer>,
    pub asteroids: Vec<Asteroid>,
    pub bullets: Vec<Bullet>,
    pub width: f32,
    pub height: f32,
    pub score: u32,
    time_since_asteroid_spawn: f32,
    time_since_saucer_spawn: f32,
    saucer_id_counter: usize,
    // Death loop protection
    deaths_in_short_time: u32,
    time_since_last_death: f32,
    max_asteroids: usize,
    // Configurable colors
    pub game_color: Color,        // Color for game objects (ship, asteroids, bullets, saucers)
    pub hud_color: Color,          // Color for HUD/instrument cluster
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    pub fn new() -> Self {
        // Default to arcade green for game, grey for HUD
        let game_color = Color::ARCADE_GREEN;
        let hud_color = Color::GREY;

        // One player ship in the center, using game color
        let player_ship = Ship::new(
            0.0, 0.0,
            game_color,
            0,
        );

        Self {
            player_ship,
            saucers: Vec::new(),
            asteroids: Vec::new(),
            bullets: Vec::new(),
            width: 1920.0,
            height: 1080.0,
            score: 0,
            time_since_asteroid_spawn: 0.0,
            time_since_saucer_spawn: 0.0,
            saucer_id_counter: 1,
            deaths_in_short_time: 0,
            time_since_last_death: 10.0,  // Start high so first death doesn't trigger
            max_asteroids: 12,  // Reasonable limit for playability
            game_color,
            hud_color,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        // Update death timer
        self.time_since_last_death += delta_time;

        // Spawn asteroids periodically (but respect max limit)
        self.time_since_asteroid_spawn += delta_time;
        if self.time_since_asteroid_spawn > 3.0 && self.asteroids.len() < self.max_asteroids {
            self.spawn_asteroid();
            self.time_since_asteroid_spawn = 0.0;
        }

        // Spawn saucers periodically
        self.time_since_saucer_spawn += delta_time;
        if self.time_since_saucer_spawn > 10.0 && self.saucers.len() < 2 {
            self.spawn_saucer();
            self.time_since_saucer_spawn = 0.0;
        }

        // Update player ship with simple AI behavior for screensaver mode
        let targets: Vec<_> = self.asteroids
            .iter()
            .map(|a| (a.x, a.y))
            .chain(self.saucers.iter().map(|s| (s.x, s.y)))
            .collect();

        let ship_state_before = self.player_ship.can_shoot();
        ai::update_ship_ai(&mut self.player_ship, &targets, delta_time);

        // Player shoots - check if burst was initiated
        if ship_state_before && self.player_ship.shoot_cooldown > 0.0 {
            // Burst initiated, shoot will be handled by update_burst
        }

        // Handle burst firing - check each frame if a bullet should be fired
        if self.player_ship.update_burst() {
            self.bullets.push(ai::create_bullet_from_ship(&self.player_ship, self.game_color));
        }

        // Update saucers with AI
        let mut new_saucer_bullets = Vec::new();
        for saucer in &mut self.saucers {
            saucer.update(delta_time);

            // Saucers shoot at player or randomly
            use rand::Rng;
            let mut rng = rand::thread_rng();

            if saucer.can_shoot() {
                let shoot_chance = match saucer.size {
                    SaucerSize::Large => 0.3,  // Less accurate, shoots more randomly
                    SaucerSize::Small => 0.7,  // More accurate
                };

                if rng.gen_bool(shoot_chance) {
                    // Aim at player
                    let dx = self.player_ship.x - saucer.x;
                    let dy = self.player_ship.y - saucer.y;
                    let angle = dy.atan2(dx);

                    new_saucer_bullets.push(Bullet::new(saucer.x, saucer.y, angle, saucer.id, self.game_color));
                    saucer.shoot();
                } else if rng.gen_bool(0.5) {
                    // Random shot
                    let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
                    new_saucer_bullets.push(Bullet::new(saucer.x, saucer.y, angle, saucer.id, self.game_color));
                    saucer.shoot();
                }
            }
        }
        self.bullets.extend(new_saucer_bullets);

        // Remove dead saucers
        self.saucers.retain(|s| s.alive);

        // Update bullets
        for bullet in &mut self.bullets {
            bullet.update(delta_time);
        }
        self.bullets.retain(|b| b.alive);

        // Update asteroids
        for asteroid in &mut self.asteroids {
            asteroid.update(delta_time);
        }

        // Check collisions
        self.check_collisions();

        // Wrap player and bullets
        physics::wrap_position(&mut self.player_ship.x, &mut self.player_ship.y);
        for bullet in &mut self.bullets {
            physics::wrap_position(&mut bullet.x, &mut bullet.y);
        }
        for asteroid in &mut self.asteroids {
            physics::wrap_position(&mut asteroid.x, &mut asteroid.y);
        }
    }

    fn spawn_saucer(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let size = if rng.gen_bool(0.7) {
            SaucerSize::Large
        } else {
            SaucerSize::Small
        };

        self.saucers.push(Saucer::new(size, self.saucer_id_counter, self.game_color));
        self.saucer_id_counter += 1;
    }

    fn handle_player_death(&mut self) {
        // Track death timing
        if self.time_since_last_death < 3.0 {
            // Death within 3 seconds of last death
            self.deaths_in_short_time += 1;
        } else {
            // Reset counter if it's been a while
            self.deaths_in_short_time = 1;
        }

        self.time_since_last_death = 0.0;

        // If dying too frequently (3+ deaths in quick succession), clear the field
        if self.deaths_in_short_time >= 3 {
            self.clear_asteroid_field();
            self.deaths_in_short_time = 0;
        }

        self.player_ship.respawn();
    }

    fn clear_asteroid_field(&mut self) {
        // Clear all asteroids to give player breathing room
        self.asteroids.clear();

        // Also clear bullets to reduce chaos
        self.bullets.clear();

        // Give a brief pause before spawning new asteroids
        self.time_since_asteroid_spawn = -5.0;  // 5 second grace period
    }

    fn spawn_asteroid(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let x = rng.gen_range(-1.0..1.0);
        let y = if rng.gen_bool(0.5) { -1.2 } else { 1.2 };

        self.asteroids.push(Asteroid::new(x, y, 3, self.game_color));
    }

    fn check_collisions(&mut self) {
        // Bullet-Asteroid collisions
        let mut new_asteroids = Vec::new();
        let mut score_add = 0;

        for bullet in &mut self.bullets {
            for asteroid in &mut self.asteroids {
                if !bullet.alive || !asteroid.alive {
                    continue;
                }

                let dx = bullet.x - asteroid.x;
                let dy = bullet.y - asteroid.y;
                let dist_sq = dx * dx + dy * dy;

                if dist_sq < asteroid.size * asteroid.size {
                    bullet.alive = false;
                    asteroid.alive = false;

                    // Award points
                    score_add += match asteroid.size_level {
                        3 => 20,
                        2 => 50,
                        1 => 100,
                        _ => 20,
                    };

                    // Split asteroid if large enough
                    if asteroid.size_level > 1 {
                        new_asteroids.push(Asteroid::new(
                            asteroid.x + 0.05,
                            asteroid.y,
                            asteroid.size_level - 1,
                            self.game_color,
                        ));
                        new_asteroids.push(Asteroid::new(
                            asteroid.x - 0.05,
                            asteroid.y,
                            asteroid.size_level - 1,
                            self.game_color,
                        ));
                    }
                }
            }
        }

        self.asteroids.retain(|a| a.alive);
        self.asteroids.extend(new_asteroids);

        // Player-Asteroid collisions
        let mut player_died = false;
        for asteroid in &self.asteroids {
            if !asteroid.alive {
                continue;
            }

            let dx = self.player_ship.x - asteroid.x;
            let dy = self.player_ship.y - asteroid.y;
            let dist_sq = dx * dx + dy * dy;

            if dist_sq < asteroid.size * asteroid.size {
                player_died = true;
                break;
            }
        }

        if player_died {
            self.handle_player_death();
            return;  // Skip remaining collision checks this frame
        }

        // Bullet-Saucer collisions
        for bullet in &mut self.bullets {
            for saucer in &mut self.saucers {
                if !bullet.alive || !saucer.alive || bullet.owner_id == saucer.id {
                    continue;
                }

                let dx = bullet.x - saucer.x;
                let dy = bullet.y - saucer.y;
                let dist_sq = dx * dx + dy * dy;
                let collision_radius = saucer.get_collision_radius();

                if dist_sq < collision_radius * collision_radius {
                    bullet.alive = false;
                    saucer.alive = false;

                    // Award points for saucer
                    score_add += match saucer.size {
                        SaucerSize::Large => 200,
                        SaucerSize::Small => 1000,
                    };
                }
            }
        }

        // Player-Saucer collisions
        let mut player_hit_saucer = false;
        for saucer in &mut self.saucers {
            if !saucer.alive {
                continue;
            }

            let dx = self.player_ship.x - saucer.x;
            let dy = self.player_ship.y - saucer.y;
            let dist_sq = dx * dx + dy * dy;
            let collision_radius = saucer.get_collision_radius();

            if dist_sq < collision_radius * collision_radius {
                saucer.alive = false;
                player_hit_saucer = true;
                break;
            }
        }

        if player_hit_saucer {
            self.handle_player_death();
            return;
        }

        // Bullet-Player collisions (from saucers)
        for bullet in &mut self.bullets {
            if !bullet.alive || bullet.owner_id == self.player_ship.id {
                continue;
            }

            let dx = bullet.x - self.player_ship.x;
            let dy = bullet.y - self.player_ship.y;
            let dist_sq = dx * dx + dy * dy;

            if dist_sq < 0.01 {
                bullet.alive = false;
                self.handle_player_death();
                return;
            }
        }

        self.score += score_add;
    }

    pub fn get_vertices(&self) -> Vec<Vertex> {
        let mut vertices = Vec::new();

        // Render score in top-left corner
        use crate::renderer::{render_number, render_label};
        vertices.extend(render_number(self.score, -0.95, 0.9, 0.06, self.hud_color));

        // Calculate aspect ratio for circular compass
        let aspect_ratio = self.width / self.height;

        // ===== STACKED INDICATORS IN TOP-RIGHT =====
        let base_x = 0.55;
        let label_size = 0.025;
        let value_size = 0.03;
        let spacing = 0.12;  // Vertical spacing between indicators
        let indicator_width = 0.12;  // All visual indicators same width
        let color = self.hud_color.to_array();

        // === 1. DIRECTION INDICATOR ===
        let dir_y = 0.90;

        // Label (left-aligned at base_x)
        vertices.extend(render_label("DIRECT", base_x, dir_y, label_size, self.hud_color));

        // Direction indicator position
        let cursor_x = base_x + 0.18 + indicator_width / 2.0;  // Center of indicator area
        let cursor_y = dir_y - 0.0125;  // Vertically centered with baseline
        let cursor_radius = 0.025;
        let cursor_y_scale = aspect_ratio;  // Scale Y to maintain aspect ratio

        // Screen-shaped box frame (rectangular outline matching screen aspect)
        let box_half_width = indicator_width / 2.0;
        let box_half_height = box_half_width / aspect_ratio;

        // Draw rectangular box frame
        let box_left = cursor_x - box_half_width;
        let box_right = cursor_x + box_half_width;
        let box_top = cursor_y + box_half_height;
        let box_bottom = cursor_y - box_half_height;

        vertices.push(Vertex { position: [box_left, box_top], color });
        vertices.push(Vertex { position: [box_right, box_top], color });
        vertices.push(Vertex { position: [box_right, box_top], color });
        vertices.push(Vertex { position: [box_right, box_bottom], color });
        vertices.push(Vertex { position: [box_right, box_bottom], color });
        vertices.push(Vertex { position: [box_left, box_bottom], color });
        vertices.push(Vertex { position: [box_left, box_bottom], color });
        vertices.push(Vertex { position: [box_left, box_top], color });

        // Circle inside the box (using 16 segments for smooth appearance)
        let circle_segments = 16;
        let circle_radius = cursor_radius * 0.8;  // Slightly smaller than cursor range
        for i in 0..circle_segments {
            let angle1 = (i as f32 / circle_segments as f32) * 2.0 * std::f32::consts::PI;
            let angle2 = ((i + 1) as f32 / circle_segments as f32) * 2.0 * std::f32::consts::PI;

            let x1 = cursor_x + circle_radius * angle1.cos();
            let y1 = cursor_y + (circle_radius / cursor_y_scale) * angle1.sin();
            let x2 = cursor_x + circle_radius * angle2.cos();
            let y2 = cursor_y + (circle_radius / cursor_y_scale) * angle2.sin();

            vertices.push(Vertex { position: [x1, y1], color });
            vertices.push(Vertex { position: [x2, y2], color });
        }

        // Center point (small dot) - draw as small cross
        let dot_size = 0.002;
        vertices.push(Vertex { position: [cursor_x - dot_size, cursor_y], color });
        vertices.push(Vertex { position: [cursor_x + dot_size, cursor_y], color });
        vertices.push(Vertex { position: [cursor_x, cursor_y - dot_size / cursor_y_scale], color });
        vertices.push(Vertex { position: [cursor_x, cursor_y + dot_size / cursor_y_scale], color });

        // Triangular cursor that rotates around center point
        let cursor_angle = self.player_ship.angle + std::f32::consts::FRAC_PI_2;

        // Cursor tip (points in heading direction)
        let tip_x = cursor_x + cursor_radius * cursor_angle.cos();
        let tip_y = cursor_y + (cursor_radius / cursor_y_scale) * cursor_angle.sin();

        // Cursor base (two points forming triangle base)
        let base_angle1 = cursor_angle + std::f32::consts::PI * 0.85;
        let base_angle2 = cursor_angle - std::f32::consts::PI * 0.85;
        let base_radius = cursor_radius * 0.4;

        let base1_x = cursor_x + base_radius * base_angle1.cos();
        let base1_y = cursor_y + (base_radius / cursor_y_scale) * base_angle1.sin();
        let base2_x = cursor_x + base_radius * base_angle2.cos();
        let base2_y = cursor_y + (base_radius / cursor_y_scale) * base_angle2.sin();

        // Draw triangle (tip to base1, base1 to base2, base2 to tip)
        vertices.push(Vertex { position: [tip_x, tip_y], color });
        vertices.push(Vertex { position: [base1_x, base1_y], color });
        vertices.push(Vertex { position: [base1_x, base1_y], color });
        vertices.push(Vertex { position: [base2_x, base2_y], color });
        vertices.push(Vertex { position: [base2_x, base2_y], color });
        vertices.push(Vertex { position: [tip_x, tip_y], color });

        // Numeric direction value (right-aligned at base_x + 0.35)
        let mut degrees = self.player_ship.angle.to_degrees();
        while degrees < 0.0 { degrees += 360.0; }
        degrees = degrees % 360.0;
        vertices.extend(render_number(degrees as u32, base_x + 0.35, dir_y, value_size, self.hud_color));

        // === 2. THRUST INDICATOR ===
        let thrust_y = dir_y - spacing;

        // Label (left-aligned at base_x)
        vertices.extend(render_label("THRUST", base_x, thrust_y, label_size, self.hud_color));

        // Thrust bar (width = indicator_width = 0.12)
        let thrust_bar_x = base_x + 0.18;
        let thrust_bar_width = indicator_width;
        let thrust_bar_height = 0.025;

        // Outline
        vertices.push(Vertex { position: [thrust_bar_x, thrust_y], color });
        vertices.push(Vertex { position: [thrust_bar_x + thrust_bar_width, thrust_y], color });
        vertices.push(Vertex { position: [thrust_bar_x + thrust_bar_width, thrust_y], color });
        vertices.push(Vertex { position: [thrust_bar_x + thrust_bar_width, thrust_y - thrust_bar_height], color });
        vertices.push(Vertex { position: [thrust_bar_x + thrust_bar_width, thrust_y - thrust_bar_height], color });
        vertices.push(Vertex { position: [thrust_bar_x, thrust_y - thrust_bar_height], color });
        vertices.push(Vertex { position: [thrust_bar_x, thrust_y - thrust_bar_height], color });
        vertices.push(Vertex { position: [thrust_bar_x, thrust_y], color });

        // Fill based on speed
        let speed = (self.player_ship.vx * self.player_ship.vx + self.player_ship.vy * self.player_ship.vy).sqrt();
        let thrust_fill = (thrust_bar_width - 0.005) * (speed / 1.0).min(1.0);
        if thrust_fill > 0.0 {
            for i in 0..3 {
                let y = thrust_y - 0.005 - (thrust_bar_height - 0.01) * (i as f32 / 2.0);
                vertices.push(Vertex { position: [thrust_bar_x + 0.003, y], color });
                vertices.push(Vertex { position: [thrust_bar_x + 0.003 + thrust_fill, y], color });
            }
        }

        // Numeric value (0-9, right-aligned at base_x + 0.35)
        let thrust_value = ((speed * 10.0).min(9.0)) as u32;
        vertices.extend(render_number(thrust_value, base_x + 0.35, thrust_y, value_size, self.hud_color));

        // === 3. POWER INDICATOR ===
        let power_y = thrust_y - spacing;

        // Label (left-aligned at base_x)
        vertices.extend(render_label("POWER", base_x, power_y, label_size, self.hud_color));

        // Power bar with tip (width = indicator_width = 0.12)
        let power_bar_x = base_x + 0.18;
        let power_bar_width = indicator_width;
        let power_bar_height = 0.025;

        // Outline
        vertices.push(Vertex { position: [power_bar_x, power_y], color });
        vertices.push(Vertex { position: [power_bar_x + power_bar_width, power_y], color });
        vertices.push(Vertex { position: [power_bar_x + power_bar_width, power_y], color });
        vertices.push(Vertex { position: [power_bar_x + power_bar_width, power_y - power_bar_height], color });
        vertices.push(Vertex { position: [power_bar_x + power_bar_width, power_y - power_bar_height], color });
        vertices.push(Vertex { position: [power_bar_x, power_y - power_bar_height], color });
        vertices.push(Vertex { position: [power_bar_x, power_y - power_bar_height], color });
        vertices.push(Vertex { position: [power_bar_x, power_y], color });

        // Battery tip (arrow pointing right)
        let tip_width = 0.008;
        let tip_height = power_bar_height * 0.4;
        let tip_y_center = power_y - power_bar_height / 2.0;
        vertices.push(Vertex { position: [power_bar_x + power_bar_width, tip_y_center + tip_height], color });
        vertices.push(Vertex { position: [power_bar_x + power_bar_width + tip_width, tip_y_center + tip_height], color });
        vertices.push(Vertex { position: [power_bar_x + power_bar_width + tip_width, tip_y_center + tip_height], color });
        vertices.push(Vertex { position: [power_bar_x + power_bar_width + tip_width, tip_y_center - tip_height], color });
        vertices.push(Vertex { position: [power_bar_x + power_bar_width + tip_width, tip_y_center - tip_height], color });
        vertices.push(Vertex { position: [power_bar_x + power_bar_width, tip_y_center - tip_height], color });

        // Fill based on energy
        let power_fill = (power_bar_width - 0.005) * self.player_ship.energy;
        if power_fill > 0.0 {
            for i in 0..4 {
                let y = power_y - 0.005 - (power_bar_height - 0.01) * (i as f32 / 3.0);
                vertices.push(Vertex { position: [power_bar_x + 0.003, y], color });
                vertices.push(Vertex { position: [power_bar_x + 0.003 + power_fill, y], color });
            }
        }

        // Numeric value (percentage, right-aligned at base_x + 0.35)
        let power_pct = (self.player_ship.energy * 100.0) as u32;
        vertices.extend(render_number(power_pct, base_x + 0.35, power_y, value_size, self.hud_color));

        // Render player ship
        vertices.extend(self.player_ship.get_vertices());

        // Render saucers
        for saucer in &self.saucers {
            if saucer.alive {
                vertices.extend(saucer.get_vertices());
            }
        }

        // Render bullets
        for bullet in &self.bullets {
            if bullet.alive {
                vertices.extend(bullet.get_vertices());
            }
        }

        // Render asteroids
        for asteroid in &self.asteroids {
            if asteroid.alive {
                vertices.extend(asteroid.get_vertices());
            }
        }

        vertices
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }
}
