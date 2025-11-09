use crate::renderer::{Vertex, Color};
use rand::Rng;

pub struct Asteroid {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub size: f32,
    pub size_level: i32,
    pub alive: bool,
    pub rotation: f32,
    pub rotation_speed: f32,
    pub color: Color,
    vertices_offset: Vec<(f32, f32)>,
}

impl Asteroid {
    pub fn new(x: f32, y: f32, size_level: i32, color: Color) -> Self {
        let mut rng = rand::thread_rng();

        let size = match size_level {
            3 => 0.15,
            2 => 0.10,
            1 => 0.05,
            _ => 0.15,
        };

        let speed = 0.2;
        let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);

        // Generate random polygon shape
        let num_vertices = 8;
        let mut vertices_offset = Vec::new();
        for i in 0..num_vertices {
            let angle = (i as f32 / num_vertices as f32) * std::f32::consts::PI * 2.0;
            let offset = rng.gen_range(0.7..1.0);
            vertices_offset.push((
                angle.cos() * offset,
                angle.sin() * offset,
            ));
        }

        Self {
            x,
            y,
            vx: angle.cos() * speed,
            vy: angle.sin() * speed,
            size,
            size_level,
            alive: true,
            rotation: 0.0,
            rotation_speed: rng.gen_range(-1.0..1.0),
            color,
            vertices_offset,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.x += self.vx * delta_time;
        self.y += self.vy * delta_time;
        self.rotation += self.rotation_speed * delta_time;
    }

    pub fn get_vertices(&self) -> Vec<Vertex> {
        let color = self.color.to_array();
        let mut vertices = Vec::new();

        for i in 0..self.vertices_offset.len() {
            let (x1, y1) = self.vertices_offset[i];
            let (x2, y2) = self.vertices_offset[(i + 1) % self.vertices_offset.len()];

            let cos = self.rotation.cos();
            let sin = self.rotation.sin();

            let rx1 = x1 * cos - y1 * sin;
            let ry1 = x1 * sin + y1 * cos;
            let rx2 = x2 * cos - y2 * sin;
            let ry2 = x2 * sin + y2 * cos;

            vertices.push(Vertex {
                position: [self.x + rx1 * self.size, self.y + ry1 * self.size],
                color,
            });
            vertices.push(Vertex {
                position: [self.x + rx2 * self.size, self.y + ry2 * self.size],
                color,
            });
        }

        vertices
    }
}
