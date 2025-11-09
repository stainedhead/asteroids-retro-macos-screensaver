use super::physics;
use crate::renderer::{Color, Vertex};

pub struct Ship {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub angle: f32,
    pub angular_velocity: f32,
    pub color: Color,
    pub id: usize,
    pub shoot_cooldown: f32,
    pub thrust_level: u8,    // 0 = no thrust, 1-3 = thrust levels
    pub energy: f32,         // Battery/energy level (0.0 to 1.0)
    pub burst_count: u8,     // Current shot in burst (0-2)
    pub burst_cooldown: f32, // Time between burst shots
    spawn_x: f32,
    spawn_y: f32,
}

impl Ship {
    pub fn new(x: f32, y: f32, color: Color, id: usize) -> Self {
        Self {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            angle: 0.0,
            angular_velocity: 0.0,
            color,
            id,
            shoot_cooldown: 0.0,
            thrust_level: 0,
            energy: 1.0, // Start with full battery
            burst_count: 0,
            burst_cooldown: 0.0,
            spawn_x: x,
            spawn_y: y,
        }
    }

    pub fn thrust(&mut self, delta_time: f32) {
        // Only thrust if we have energy
        if self.energy > 0.0 {
            // Set velocity directly in forward direction (not additive)
            // Ship nose points up (+Y) in local coords at angle 0
            // In standard trig, +Y is angle π/2, so we ADD π/2
            let thrust_speed = 0.6; // Fixed forward speed when thrusting
            let forward_angle = self.angle + std::f32::consts::FRAC_PI_2;
            self.vx = forward_angle.cos() * thrust_speed;
            self.vy = forward_angle.sin() * thrust_speed;

            // Drain energy when thrusting
            self.energy -= 0.3 * delta_time;
            if self.energy < 0.0 {
                self.energy = 0.0;
            }

            // Set thrust level based on velocity
            let speed = (self.vx * self.vx + self.vy * self.vy).sqrt();
            self.thrust_level = if speed > 0.7 {
                3
            } else if speed > 0.4 {
                2
            } else if speed > 0.1 {
                1
            } else {
                0
            };

            // Velocity limiting
            if speed > 1.0 {
                self.vx = (self.vx / speed) * 1.0;
                self.vy = (self.vy / speed) * 1.0;
            }
        } else {
            self.thrust_level = 0;
        }
    }

    pub fn rotate(&mut self, direction: f32, delta_time: f32) {
        self.angular_velocity = direction * 3.0;
        self.angle += self.angular_velocity * delta_time;
    }

    pub fn update(&mut self, delta_time: f32) {
        self.x += self.vx * delta_time;
        self.y += self.vy * delta_time;

        // Apply angular velocity to angle
        self.angle += self.angular_velocity * delta_time;

        // Strong friction to slow down quickly when not thrusting
        self.vx *= 0.95;
        self.vy *= 0.95;

        // Decay thrust level based on velocity
        let speed = (self.vx * self.vx + self.vy * self.vy).sqrt();
        self.thrust_level = if speed > 0.7 {
            3
        } else if speed > 0.4 {
            2
        } else if speed > 0.1 {
            1
        } else {
            0
        };

        // Update shoot cooldown
        if self.shoot_cooldown > 0.0 {
            self.shoot_cooldown -= delta_time;
        }

        // Update burst cooldown
        if self.burst_cooldown > 0.0 {
            self.burst_cooldown -= delta_time;
        }

        // Recharge energy when not shooting or thrusting (fast recharge)
        if self.shoot_cooldown <= 0.0 && self.thrust_level == 0 {
            self.energy += 1.5 * delta_time; // Fast recharge
            if self.energy > 1.0 {
                self.energy = 1.0;
            }
        }
    }

    pub fn can_shoot(&self) -> bool {
        self.shoot_cooldown <= 0.0 && self.energy >= 0.1
    }

    pub fn shoot(&mut self) {
        // Start a 3-shot burst
        self.burst_count = 0;
        self.shoot_cooldown = 1.0; // Cooldown between bursts
        self.burst_cooldown = 0.0;
    }

    pub fn update_burst(&mut self) -> bool {
        // Returns true if a bullet should be fired this frame
        if self.burst_count < 3 && self.burst_cooldown <= 0.0 && self.energy >= 0.1 {
            self.burst_count += 1;
            self.burst_cooldown = 0.08; // Fast shots in burst (80ms between shots)

            // Drain energy for each shot
            self.energy -= 0.1;
            if self.energy < 0.0 {
                self.energy = 0.0;
            }

            return true;
        }
        false
    }

    pub fn respawn(&mut self) {
        self.x = self.spawn_x;
        self.y = self.spawn_y;
        self.vx = 0.0;
        self.vy = 0.0;
        self.angle = 0.0;
        self.angular_velocity = 0.0;
        self.thrust_level = 0;
        self.energy = 1.0; // Restore full energy on respawn
        self.burst_count = 0;
        self.burst_cooldown = 0.0;
    }

    // Get the position of the ship's nose (for bullet spawning)
    pub fn get_nose_position(&self) -> (f32, f32) {
        let size = 0.024; // Reduced by 20% (0.03 * 0.8 = 0.024)
        let nose_offset = size * 2.0;
        // Ship nose is at +Y in local coords, which matches how rotate_point works
        let (nx, ny) = physics::rotate_point(0.0, nose_offset, self.angle);
        (self.x + nx, self.y + ny)
    }

    pub fn get_vertices(&self) -> Vec<Vertex> {
        let size = 0.024; // Reduced by 20% (0.03 * 0.8 = 0.024)
        let color = [self.color.r, self.color.g, self.color.b, self.color.a];

        // Redesigned ship with straight sides
        //       0 (nose/tip)
        //      /|\  (small diamond window)
        //     1 | 2
        //     |  -  |  (crossbar)
        //     |     |  (straight sides)
        //     3     4  (wing shoulders)
        //     ||   ||  (double overhang lines)
        //     5     6  (base)

        let wing_height = size * 1.5; // Total height
        let top_width = size * 0.8; // Width at shoulders
        let base_width = size * 0.4; // Width at base (narrower overhang)

        // Crossbar at 75% height
        let crossbar_y = -wing_height + (wing_height * 0.25);
        let shoulder_y = size * 0.5; // Where straight sides begin

        let points = [
            (0.0, size * 2.0),           // 0: Nose tip
            (-top_width, shoulder_y),    // 1: Left shoulder
            (top_width, shoulder_y),     // 2: Right shoulder
            (-top_width, crossbar_y),    // 3: Left at crossbar
            (top_width, crossbar_y),     // 4: Right at crossbar
            (-base_width, -wing_height), // 5: Left base (overhang)
            (base_width, -wing_height),  // 6: Right base (overhang)
        ];

        let mut vertices = Vec::new();

        // Draw the main ship outline
        let lines = vec![
            (0, 1), // Left side from nose to shoulder
            (1, 3), // Left straight side to crossbar
            (3, 5), // Left side continues to base overhang
            (0, 2), // Right side from nose to shoulder
            (2, 4), // Right straight side to crossbar
            (4, 6), // Right side continues to base overhang
            (3, 4), // Crossbar
        ];

        for (start, end) in lines {
            let (x1, y1) = points[start];
            let (x2, y2) = points[end];

            let (rx1, ry1) = physics::rotate_point(x1, y1, self.angle);
            let (rx2, ry2) = physics::rotate_point(x2, y2, self.angle);

            vertices.push(Vertex {
                position: [self.x + rx1, self.y + ry1],
                color,
            });
            vertices.push(Vertex {
                position: [self.x + rx2, self.y + ry2],
                color,
            });
        }

        // Add small diamond window in the nose (doesn't touch sides)
        let window_size = size * 0.3;
        let window_y = size * 1.2; // Inside the nose area
        let diamond_points = [
            (0.0, window_y + window_size),  // Top
            (-window_size * 0.5, window_y), // Left
            (0.0, window_y - window_size),  // Bottom
            (window_size * 0.5, window_y),  // Right
        ];

        let diamond_lines = vec![(0, 1), (1, 2), (2, 3), (3, 0)];
        for (start, end) in diamond_lines {
            let (x1, y1) = diamond_points[start];
            let (x2, y2) = diamond_points[end];

            let (rx1, ry1) = physics::rotate_point(x1, y1, self.angle);
            let (rx2, ry2) = physics::rotate_point(x2, y2, self.angle);

            vertices.push(Vertex {
                position: [self.x + rx1, self.y + ry1],
                color,
            });
            vertices.push(Vertex {
                position: [self.x + rx2, self.y + ry2],
                color,
            });
        }

        // Add double lines for the overhang at bottom
        let inner_width = base_width * 0.6;
        let overhang_lines = vec![
            // Outer overhang lines (already drawn as part of main outline)
            // Inner overhang lines (parallel to outer)
            ((-inner_width, crossbar_y), (-inner_width, -wing_height)), // Left inner
            ((inner_width, crossbar_y), (inner_width, -wing_height)),   // Right inner
        ];

        for ((x1, y1), (x2, y2)) in overhang_lines {
            let (rx1, ry1) = physics::rotate_point(x1, y1, self.angle);
            let (rx2, ry2) = physics::rotate_point(x2, y2, self.angle);

            vertices.push(Vertex {
                position: [self.x + rx1, self.y + ry1],
                color,
            });
            vertices.push(Vertex {
                position: [self.x + rx2, self.y + ry2],
                color,
            });
        }

        // Add thrust flame from the back if thrusting
        if self.thrust_level > 0 {
            let flame_size = match self.thrust_level {
                1 => size * 0.8,
                2 => size * 1.2,
                3 => size * 1.6,
                _ => 0.0,
            };

            // Triangle flame pointing away from the back center
            let back_y = -wing_height;
            let flame_points = [
                (-size * 0.3, back_y),      // Left edge at ship base
                (size * 0.3, back_y),       // Right edge at ship base
                (0.0, back_y - flame_size), // Point extending away from ship
            ];

            // Draw flame triangle (3 lines)
            let flame_lines = vec![
                (0, 1), // Base of triangle at ship
                (1, 2), // Right side to point
                (2, 0), // Left side back to base
            ];

            for (start, end) in flame_lines {
                let (x1, y1) = flame_points[start];
                let (x2, y2) = flame_points[end];

                let (rx1, ry1) = physics::rotate_point(x1, y1, self.angle);
                let (rx2, ry2) = physics::rotate_point(x2, y2, self.angle);

                vertices.push(Vertex {
                    position: [self.x + rx1, self.y + ry1],
                    color,
                });
                vertices.push(Vertex {
                    position: [self.x + rx2, self.y + ry2],
                    color,
                });
            }
        }

        vertices
    }
}
