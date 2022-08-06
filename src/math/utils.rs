use super::vector::{Vector2f, Vector3f, Vector4f};

pub fn interpolation(x1: f32, x2: f32, t: f32) -> f32 {
    x1 + (x2 - x1) * t
}

pub fn interpolate_vector2f(a: &Vector2f, b: &Vector2f, c: &Vector2f, alpha: f32, beta: f32, gamma: f32) -> Vector2f{
    Vector2f::new_2(alpha * a.x() + beta * b.x() + gamma * c.x(), alpha * a.y() + beta * b.y() + gamma * c.y())
}


pub fn interpolate_vector3f(a: &Vector3f, b: &Vector3f, c: &Vector3f, alpha: f32, beta: f32, gamma: f32) -> Vector3f{
    Vector3f::new_3(alpha * a.x() + beta * b.x() + gamma * c.x(), alpha * a.y() + beta * b.y() + gamma * c.y(), alpha * a.z() + beta * b.z() + gamma * c.z())
}

pub fn interpolate_vector4f(a: &Vector4f, b: &Vector4f, c: &Vector4f, alpha: f32, beta: f32, gamma: f32) -> Vector4f{
    Vector4f::new_4(
        alpha * a.x() + beta * b.x() + gamma * c.x(), 
        alpha * a.y() + beta * b.y() + gamma * c.y(), 
        alpha * a.z() + beta * b.z() + gamma * c.z(),
        alpha * a.w() + beta * b.w() + gamma * c.w())
}

pub fn clamp<T: std::cmp::PartialOrd> (v: T, min: T, max: T) -> T {
    if v<min {min} else if v>max {max} else {v}
}