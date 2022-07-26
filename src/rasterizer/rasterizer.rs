use crate::math::{matrix::Matrix, vector::{Vector}};
use super::{triangle::{Triangle}, edge_walking::draw_trangle_edge_walking, edge_equation::draw_trangle_edge_equation};

pub struct Rasterizer {
    model: Matrix,
    view: Matrix,
    projection: Matrix,
    mvp: Matrix,
    
    //view port
    //width: i32,
    //height: i32,

    //image
    //buf: Option<Vec<u8>>
}

impl Rasterizer {
    pub fn new() -> Rasterizer {
        Rasterizer {
            model: Matrix::identity(),
            view: Matrix::identity(),
            projection: Matrix::identity(),
            mvp: Matrix::identity(),
        }
    }

    pub fn set_view(&mut self, m: Matrix) {
        self.view = m;

    }

    pub fn set_model(&mut self, m: Matrix) {
        self.model = m;
    }

    pub fn set_projection(&mut self, m: Matrix) {
        self.projection = m;
    }

    pub fn compute_mvp(&mut self) {
        self.mvp = self.projection.mul(&self.view).mul(&self.model);
    }
}

pub fn draw_trangle(rasterizer: &Rasterizer, 
    image: &mut Vec<u8>, 
    zbuf: &mut Vec<f32>,
    near: f32,
    far: f32,
    width: i32, 
    height: i32, 
    mut triangle: Triangle) {
    let t1 = rasterizer.mvp.apply(&triangle.vertexs[0].v);
    let t2 = rasterizer.mvp.apply(&triangle.vertexs[1].v);
    let t3 = rasterizer.mvp.apply(&triangle.vertexs[2].v);

    let view_port = get_view_port(width as f32, height as f32);
    let mut p1 = view_port.apply(&t1);
    let mut p2 = view_port.apply(&t2);
    let mut p3 = view_port.apply(&t3);

    p1.divide_w(); p2.divide_w(); p3.divide_w();
    p1.reset_z(near, far);
    p2.reset_z(near, far);
    p3.reset_z(near, far);

    triangle.set_vertexs(vec![p1, p2, p3]);
    //draw_trangle_edge_walking(image, zbuf, width, height, &triangle);
    draw_trangle_edge_equation(image, zbuf, width, height, &triangle);
}

pub fn get_view_matrix(eye: Vector, at: Vector, mut up: Vector) -> Matrix {
    let mut g = at.sub(&eye);
    g.normlize();
    up.normlize();
    let mut x = g.cross_product(&up);
    x.normlize();
    Matrix { 
        m: vec![
            vec![x.x, x.y, x.z, -eye.x],
            vec![up.x, up.y, up.z, -eye.y],
            vec![-g.x, -g.y, -g.z, -eye.z],
            vec![0.0, 0.0, 0.0, 1.0]]
    }
}

pub fn get_view_port(width: f32, height: f32) -> Matrix {
    Matrix { 
        m: vec![
            vec![width/2.0, 0.0, 0.0, width/2.0],
            vec![0.0, -height/2.0, 0.0, height/2.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]
    }
}

pub fn get_model_matrix(angel: f32) -> Matrix {
    let r = std::f32::consts::PI * angel / 180.0;
    Matrix {
        m: vec![
            vec![r.cos(), 0.0, r.sin(), 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![-r.sin(), 0.0, r.cos(), 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]
    }
}

pub fn get_ortho_projection_matrix(l: f32, r: f32, t: f32, b: f32, n: f32, f: f32) -> Matrix {
    Matrix { 
        m: vec![
            vec![2.0/(r - l), 0.0, 0.0, 0.0],
            vec![0.0, 2.0/(t - b), 0.0, 0.0],
            vec![0.0, 0.0, 2.0/(n - f), 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]
    } * Matrix {
        m: vec![
            vec![1.0, 0.0, 0.0, -(l+r)/2.0],
            vec![0.0, 1.0, 0.0, -(t+b)/2.0],
            vec![0.0, 0.0, 1.0, -(n+f)/2.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]
    }
}

pub fn get_presp_projection_matrix(eye_fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Matrix {
    let angle = eye_fov * std::f32::consts::PI / 180.0;
    //let height = near * angle.tan();
    //let width = height * aspect_ratio;

    let t = near.abs() * (angle/2.0).tan();
    let r = t * aspect_ratio;
    let l = -r;
    let b = -t;

    get_ortho_projection_matrix(l, r, t, b, near, far) * Matrix {
        m: vec![
            vec![near, 0.0, 0.0, 0.0],
            vec![0.0, near, 0.0, 0.0],
            vec![0.0, 0.0, near+far, -near*far],
            vec![0.0, 0.0, 1.0, 0.0],
        ]
    }

    // let v = Vector { x: 1.0, y:-2.0, z: -3.0, w: 1.0 };
    // let view = get_view_matrix(
    //     Vector::new(0.0, 0.0, 5.0, 1.0),
    //     Vector::new(0.0, 0.0, 0.0, 1.0),
    //     Vector::new(0.0, 1.0, 0.0, 1.0)
    // );

    // let r1 = view.apply(&v);
    
    // let prop = Matrix {
    //         m: vec![
    //             vec![near.abs(), 0.0, 0.0, 0.0],
    //             vec![0.0, near.abs(), 0.0, 0.0],
    //             vec![0.0, 0.0, near.abs()+far.abs(), -near.abs()*far.abs()],
    //             vec![0.0, 0.0, 1.0, 0.0],
    //         ]
    //     };
    // let r2 = prop.apply(&r1);

    //get_ortho_projection_matrix(l, r, t, b, near, far)
}
