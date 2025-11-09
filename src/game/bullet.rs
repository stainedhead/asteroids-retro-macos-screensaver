use crate::renderer::{Vertex, Color};

pub struct Bullet {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub alive: bool,
    pub lifetime: f32,
    pub owner_id: usize,
    pub color: Color,
}

impl Bullet {
    pub fn new(x: f32, y: f32, angle: f32, owner_id: usize, color: Color) -> Self {
        let speed = 2.0;
        // Ship nose points up (+Y) in local coords at angle 0
        // In standard trig, +Y is angle π/2, so we ADD π/2
        let forward_angle = angle + std::f32::consts::FRAC_PI_2;
        Self {
            x,
            y,
            vx: forward_angle.cos() * speed,
            vy: forward_angle.sin() * speed,
            alive: true,
            lifetime: 2.0, // 2 second lifetime
            owner_id,
            color,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.x += self.vx * delta_time;
        self.y += self.vy * delta_time;

        self.lifetime -= delta_time;
        if self.lifetime <= 0.0 {
            self.alive = false;
        }
    }

    pub fn get_vertices(&self) -> Vec<Vertex> {
        let size = 0.008;
        let color = self.color.to_array();

        // Simple small square for bullet
        vec![
            Vertex {
                position: [self.x - size, self.y - size],
                color,
            },
            Vertex {
                position: [self.x + size, self.y - size],
                color,
            },
            Vertex {
                position: [self.x + size, self.y - size],
                color,
            },
            Vertex {
                position: [self.x + size, self.y + size],
                color,
            },
            Vertex {
                position: [self.x + size, self.y + size],
                color,
            },
            Vertex {
                position: [self.x - size, self.y + size],
                color,
            },
            Vertex {
                position: [self.x - size, self.y + size],
                color,
            },
            Vertex {
                position: [self.x - size, self.y - size],
                color,
            },
        ]
    }
}
