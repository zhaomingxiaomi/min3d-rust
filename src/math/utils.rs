pub fn interpolation(x1: f32, x2: f32, t: f32) -> f32 {
    x1 + (x2 - x1) * t
}

pub fn clamp(v: i32, min: i32, max: i32) -> i32 {
    if v<min {min} else if v>max {max} else {v}
}