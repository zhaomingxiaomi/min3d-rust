use crate::math::{utils::interpolation};
use crate::math::vector::{Vector4f, Color3f, Point2f, Point3f, vector4f_interpolation};

use super::texture::Texture;

#[derive(Clone)]
pub enum RenderType {
    COLOR,
    TEXTURE
}
#[derive(Clone)]
pub struct Triangle {
    pub vertexs: Vec<Vertex>,
    pub render: RenderType
}

#[derive(Clone)]
pub struct Vertex {
    pub origin_v: Vector4f,
    pub tv: Vector4f,
    pub v: Vector4f,
    pub color: Color3f,
    pub tex_coords: Point2f,
    pub normal: Point3f,
    pub rhw: f32
}

pub fn vertex_interp(v1: &Vertex, v2: &Vertex, t: f32) -> Vertex {
    Vertex {
        origin_v: vector4f_interpolation(&v1.origin_v, &v2.origin_v, t),
        v: vector4f_interpolation(&v1.v, &v2.v, t),
        color: Color3f::new_3(
            interpolation(v1.color.r(), v2.color.r(), t), 
            interpolation(v1.color.g(), v2.color.g(), t), 
            interpolation(v1.color.b(), v2.color.b(), t), 
        ),
        tv: vector4f_interpolation(&v1.tv, &v2.tv, t),
        tex_coords: Point2f::new_2(
            interpolation(v1.tex_coords.u(), v2.tex_coords.u(), t),
            interpolation(v1.tex_coords.v(), v2.tex_coords.v(), t),
        ),
        normal: Point3f::new_3(
            interpolation(v1.normal.x(), v2.normal.x(), t),
            interpolation(v1.normal.y(), v2.normal.y(), t),
            interpolation(v1.normal.z(), v2.normal.z(), t),
        ),
        rhw: interpolation(v1.rhw, v2.rhw, t)
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Self { origin_v: Vector4f::new(), tv: Vector4f::new(), v: Vector4f::new(), color: Color3f::new(), tex_coords: Point2f::new(), normal: Point3f::new(), rhw: 1.0 }
    }
}


impl<'a> Triangle {
    pub fn new() -> Triangle {
        Triangle {
            vertexs: vec![Vertex::default(), Vertex::default(), Vertex::default()],
            render: RenderType::COLOR,
        }
    }

    pub fn set_render_type(&mut self, t: RenderType) {
        self.render = t;
    }

    pub fn set_origin_vertexs(&mut self, v: Vec<Vector4f>) {
        for i in 0..v.len() {
            self.vertexs[i].origin_v = v[i].clone();
        }
    }

    pub fn set_tvetexs(&mut self, v: Vec<Vector4f>) {
        for i in 0..v.len() {
            self.vertexs[i].tv = v[i].clone();
        }
    }

    pub fn set_vertexs(&mut self, mut v: Vec<Vector4f>) {
        for i in 0..v.len() {
            self.vertexs[i].rhw =  1.0 / v[i].w();
            v[i].divide_w();
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

    pub fn set_normal(&mut self, c: Vec<Point3f>) {
        for i in 0..c.len() {
            self.vertexs[i].normal = c[i].clone();
        }
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Self::new()
    }
}