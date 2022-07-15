use crate::math::{vector::{Point3f, Point2f, Vector, vector_interpolation, Color}, utils::interpolation};

#[derive(Clone)]
pub struct Triangle {
    pub vertexs: Vec<Vertex>
}

#[derive(Clone, Copy)]
pub struct Vertex {
    pub v: Vector,
    pub color: Color,
    pub tex_coords: Point2f,
    pub normal: Point3f,
    pub ws: f32
}

pub fn vertex_interp(v1: &Vertex, v2: &Vertex, t: f32) -> Vertex {
    Vertex {
        v: vector_interpolation(&v1.v, &v2.v, t),
        color: Color { 
            r: interpolation(v1.color.r, v2.color.r, t), 
            g: interpolation(v1.color.g, v2.color.g, t), 
            b: interpolation(v1.color.b, v2.color.b, t), 
        },
        tex_coords: Point2f {
            x: interpolation(v1.tex_coords.x, v2.tex_coords.x, t),
            y: interpolation(v1.tex_coords.y, v2.tex_coords.y, t),
        },
        normal: Point3f {
            x: interpolation(v1.normal.x, v2.normal.x, t),
            y: interpolation(v1.normal.y, v2.normal.y, t),
            z: interpolation(v1.normal.y, v2.normal.y, t),
        },

        ws: interpolation(v1.ws, v2.ws, t),
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Self { v: Default::default(), ws: Default::default(), color: Default::default(), tex_coords: Default::default(), normal: Default::default() }
    }
}


impl Triangle {
    pub fn new() -> Triangle {
        Triangle {
            vertexs: vec![Vertex::default(), Vertex::default(), Vertex::default()]
        }
    }

    pub fn set_vertexs(&mut self, v: Vec<Vector>) {
        for i in 0..v.len() {
            self.vertexs[i].v = v[i];
            self.vertexs[i].ws = v[i].w;
        }
    }

    pub fn set_colors(&mut self, c: Vec<Color>) {
        for i in 0..c.len() {
            self.vertexs[i].color = c[i];
        }
    }

    pub fn set_tex_coords(&mut self, v: Vec<Point2f>) {
        for i in 0..v.len() {
            self.vertexs[i].tex_coords = v[i];
        }
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Self::new()
    }
}