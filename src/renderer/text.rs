use super::{Vertex, Color};

// Simple text label renderer for common letters
pub fn render_label(text: &str, x: f32, y: f32, size: f32, color: Color) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    let char_width = size * 0.8;

    for (i, ch) in text.chars().enumerate() {
        let char_x = x + (i as f32) * char_width;
        vertices.extend(render_char(ch, char_x, y, size * 0.6, color));
    }

    vertices
}

fn render_char(ch: char, x: f32, y: f32, size: f32, color: Color) -> Vec<Vertex> {
    let color = [color.r, color.g, color.b, color.a];
    let mut vertices = Vec::new();

    // Define points for character
    let p = [
        (x, y + size),              // 0: top-left
        (x + size * 0.6, y + size), // 1: top-right
        (x + size * 0.6, y + size * 0.5), // 2: mid-right
        (x + size * 0.6, y),        // 3: bottom-right
        (x, y),                     // 4: bottom-left
        (x, y + size * 0.5),        // 5: mid-left
        (x + size * 0.3, y + size * 0.5), // 6: center
    ];

    let segments: Vec<(usize, usize)> = match ch {
        'D' => vec![(0,4), (0,1), (1,3), (3,4)],
        'I' => vec![(0,1), (6,2), (4,3)],
        'R' => vec![(0,4), (0,1), (1,2), (2,5), (5,0), (5,3)],
        'T' => vec![(0,1), (6,4)],
        'H' => vec![(0,4), (1,3), (5,2)],
        'U' => vec![(0,4), (4,3), (3,1)],
        'S' => vec![(1,0), (0,5), (5,2), (2,3), (3,4)],
        'B' => vec![(0,4), (0,1), (1,2), (2,5), (5,0), (5,3), (3,4)],
        'A' => vec![(4,0), (0,1), (1,3), (5,2)],
        'E' => vec![(1,0), (0,4), (4,3), (5,2)],
        'Y' => vec![(0,6), (1,6), (6,4)],
        'C' => vec![(1,0), (0,4), (4,3)],
        'P' => vec![(4,0), (0,1), (1,2), (2,5), (5,0)],
        'O' => vec![(0,1), (1,3), (3,4), (4,0), (0,5), (5,2), (2,3)],
        'W' => vec![(0,4), (4,6), (6,3), (3,1)],
        _ => vec![],
    };

    for (start, end) in segments {
        vertices.push(Vertex {
            position: [p[start].0, p[start].1],
            color,
        });
        vertices.push(Vertex {
            position: [p[end].0, p[end].1],
            color,
        });
    }

    vertices
}

// Simple vector-based digit renderer (0-9)
pub fn render_digit(digit: u32, x: f32, y: f32, size: f32, color: Color) -> Vec<Vertex> {
    let color = [color.r, color.g, color.b, color.a];
    let mut vertices = Vec::new();

    // Define segments for 7-segment style digits
    let segments = match digit {
        0 => vec![(0,1), (1,2), (2,3), (3,4), (4,5), (5,0)], // Rectangle minus bottom
        1 => vec![(1,2), (2,3)], // Right side
        2 => vec![(0,1), (1,2), (2,7), (7,4), (4,5)], // S shape
        3 => vec![(0,1), (1,2), (2,3), (3,4), (2,7)],
        4 => vec![(0,7), (7,2), (1,2), (2,3)],
        5 => vec![(1,0), (0,7), (7,2), (2,3), (3,4)],
        6 => vec![(1,0), (0,5), (5,4), (4,3), (3,2), (2,7)],
        7 => vec![(0,1), (1,2), (2,3)],
        8 => vec![(0,1), (1,2), (2,3), (3,4), (4,5), (5,0), (0,7), (7,2)],
        9 => vec![(4,3), (3,2), (2,1), (1,0), (0,5), (5,2)],
        _ => vec![],
    };

    // Define 7-segment display points
    // 0---1
    // |   |
    // 5-7-2
    // |   |
    // 4---3
    let points = [
        (x, y + size),           // 0: top-left
        (x + size * 0.6, y + size), // 1: top-right
        (x + size * 0.6, y + size * 0.5), // 2: mid-right
        (x + size * 0.6, y),     // 3: bottom-right
        (x, y),                  // 4: bottom-left
        (x, y + size * 0.5),     // 5: mid-left
        (x + size * 0.3, y + size * 0.5), // 6: center
        (x + size * 0.3, y + size * 0.5), // 7: center (duplicate for middle bar)
    ];

    for (start, end) in segments {
        vertices.push(Vertex {
            position: [points[start].0, points[start].1],
            color,
        });
        vertices.push(Vertex {
            position: [points[end].0, points[end].1],
            color,
        });
    }

    vertices
}

pub fn render_number(number: u32, x: f32, y: f32, size: f32, color: Color) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    let digits: Vec<u32> = number.to_string().chars().filter_map(|c| c.to_digit(10)).collect();

    let spacing = size * 0.8;

    for (i, digit) in digits.iter().enumerate() {
        let digit_x = x + (i as f32) * spacing;
        vertices.extend(render_digit(*digit, digit_x, y, size, color));
    }

    vertices
}
