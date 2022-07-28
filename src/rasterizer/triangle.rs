use crate::math::{utils::interpolation};
use crate::math::vector::{Vector4f, Color3f, Point2f, Point3f, vector4f_interpolation};

#[derive(Clone)]
pub struct Triangle {
    pub vertexs: Vec<Vertex>
}

#[derive(Clone)]
pub struct Vertex {
    pub v: Vector4f,
    pub color: Color3f,
    pub tex_coords: Point2f,
    pub normal: Point3f,
}

pub fn vertex_interp(v1: &Vertex, v2: &Vertex, t: f32) -> Vertex {
    Vertex {
        v: vector4f_interpolation(&v1.v, &v2.v, t),
        color: Color3f::new_3(
            interpolation(v1.color.r(), v2.color.r(), t), 
            interpolation(v1.color.g(), v2.color.g(), t), 
            interpolation(v1.color.b(), v2.color.b(), t), 
        ),
        tex_coords: Point2f::new_2(
            interpolation(v1.tex_coords.x(), v2.tex_coords.x(), t),
            interpolation(v1.tex_coords.y(), v2.tex_coords.y(), t),
        ),
        normal: Point3f::new_3(
            interpolation(v1.normal.x(), v2.normal.x(), t),
            interpolation(v1.normal.y(), v2.normal.y(), t),
            interpolation(v1.normal.y(), v2.normal.y(), t),
        ),
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Self { v: Vector4f::new(), color: Color3f::new(), tex_coords: Point2f::new(), normal: Point3f::new() }
    }
}


impl Triangle {
    pub fn new() -> Triangle {
        Triangle {
            vertexs: vec![Vertex::default(), Vertex::default(), Vertex::default()]
        }
    }

    pub fn set_vertexs(&mut self, v: Vec<Vector4f>) {
        for i in 0..v.len() {
            self.vertexs[i].v = v[i].clone();
        }
    }

    pub fn set_colors(&mut self, c: Vec<Color3f>) {
        for i in 0..c.len() {
            self.vertexs[i].color = c[i].clone();
        }
    }

    pub fn set_tex_coords(&mut self, v: Vec<Point2f>) {
        for i in 0..v.len() {
            self.vertexs[i].tex_coords = v[i].clone();
        }
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Self::new()
    }
}