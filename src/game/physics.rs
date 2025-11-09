pub fn wrap_position(x: &mut f32, y: &mut f32) {
    if *x > 1.2 {
        *x = -1.2;
    } else if *x < -1.2 {
        *x = 1.2;
    }

    if *y > 1.2 {
        *y = -1.2;
    } else if *y < -1.2 {
        *y = 1.2;
    }
}

pub fn rotate_point(x: f32, y: f32, angle: f32) -> (f32, f32) {
    let cos = angle.cos();
    let sin = angle.sin();
    (x * cos - y * sin, x * sin + y * cos)
}
