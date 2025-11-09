use crate::renderer::{Vertex, Color};
use rand::Rng;

#[derive(Clone, Copy)]
pub enum SaucerSize {
    Large,
    Small,
}

pub struct Saucer {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub size: SaucerSize,
    pub alive: bool,
    pub shoot_cooldown: f32,
    pub id: usize,
    pub color: Color,
}

impl Saucer {
    pub fn new(size: SaucerSize, id: usize, color: Color) -> Self {
        let mut rng = rand::thread_rng();

        // Spawn from left or right edge
        let from_left = rng.gen_bool(0.5);
        let x = if from_left { -1.2 } else { 1.2 };
        let y = rng.gen_range(-0.8..0.8);

        // Move horizontally across screen
        let speed = match size {
            SaucerSize::Large => 0.3,
            SaucerSize::Small => 0.5,
        };
        let vx = if from_left { speed } else { -speed };

        // Small random vertical movement
        let vy = rng.gen_range(-0.1..0.1);

        Self {
            x,
            y,
            vx,
            vy,
            size,
            alive: true,
            shoot_cooldown: 1.0,
            id,
            color,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.x += self.vx * delta_time;
        self.y += self.vy * delta_time;

        // Random direction changes
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.02) {
            self.vy = rng.gen_range(-0.1..0.1);
        }

        // Update shoot cooldown
        if self.shoot_cooldown > 0.0 {
            self.shoot_cooldown -= delta_time;
        }

        // Remove if off screen
        if self.x.abs() > 1.5 || self.y.abs() > 1.5 {
            self.alive = false;
        }
    }

    pub fn can_shoot(&self) -> bool {
        self.shoot_cooldown <= 0.0
    }

    pub fn shoot(&mut self) {
        self.shoot_cooldown = match self.size {
            SaucerSize::Large => 2.0,  // Slower shooting
            SaucerSize::Small => 1.0,  // Faster shooting
        };
    }

    pub fn get_size_value(&self) -> f32 {
        match self.size {
            SaucerSize::Large => 0.07,   // Increased by 40% (0.05 * 1.4 = 0.07)
            SaucerSize::Small => 0.042,  // Increased by 40% (0.03 * 1.4 = 0.042)
        }
    }

    pub fn get_collision_radius(&self) -> f32 {
        self.get_size_value() * 1.5
    }

    pub fn get_vertices(&self) -> Vec<Vertex> {
        let size = self.get_size_value();
        let color = self.color.to_array();

        let mut vertices = Vec::new();

        // Classic flying saucer shape
        // Top dome
        let dome_points = [(-size * 0.6, 0.0),
            (-size * 0.4, size * 0.4),
            (0.0, size * 0.5),
            (size * 0.4, size * 0.4),
            (size * 0.6, 0.0)];

        // Bottom section
        let bottom_points = [(-size * 0.6, 0.0),
            (-size, -size * 0.3),
            (size, -size * 0.3),
            (size * 0.6, 0.0)];

        // Center line (widest part)
        let center_line = [(-size, 0.0),
            (size, 0.0)];

        // Draw dome
        for i in 0..dome_points.len() - 1 {
            vertices.push(Vertex {
                position: [self.x + dome_points[i].0, self.y + dome_points[i].1],
                color,
            });
            vertices.push(Vertex {
                position: [self.x + dome_points[i + 1].0, self.y + dome_points[i + 1].1],
                color,
            });
        }

        // Draw bottom
        for i in 0..bottom_points.len() - 1 {
            vertices.push(Vertex {
                position: [self.x + bottom_points[i].0, self.y + bottom_points[i].1],
                color,
            });
            vertices.push(Vertex {
                position: [self.x + bottom_points[i + 1].0, self.y + bottom_points[i + 1].1],
                color,
            });
        }

        // Draw center line
        vertices.push(Vertex {
            position: [self.x + center_line[0].0, self.y + center_line[0].1],
            color,
        });
        vertices.push(Vertex {
            position: [self.x + center_line[1].0, self.y + center_line[1].1],
            color,
        });

        // Add small details (windows/lights)
        let detail_size = size * 0.15;
        for offset in [-0.3, 0.0, 0.3] {
            vertices.push(Vertex {
                position: [self.x + size * offset - detail_size, self.y + size * 0.2],
                color,
            });
            vertices.push(Vertex {
                position: [self.x + size * offset + detail_size, self.y + size * 0.2],
                color,
            });
        }

        vertices
    }
}
