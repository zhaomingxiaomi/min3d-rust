use crate::math::vector::{Vector, Point3f, Point2f};

struct Triangle {
    v: Vec<Vector>,
    color: Vec<Point3f>,
    tex_coords: Vec<Point2f>,
    normal: Vec<Point3f>
}

impl Triangle {
    pub fn new() -> Triangle {
        Triangle {
            v: Vec::new(),
            color: Vec::new(),
            tex_coords: Vec::new(),
            normal: Vec::new()
        }
    }
}