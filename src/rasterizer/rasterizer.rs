use crate::math::{matrix::Matrix, vector::Vector};

pub struct Rasterizer {
    model: Matrix,
    view: Matrix,
    projection: Matrix,
    mvp: Matrix
}

enum ProjectionType {
    Orthographic,
    Prespective
}

impl Rasterizer {
    fn new() -> Rasterizer {
        Rasterizer {
            model: Matrix::identity(),
            view: Matrix::identity(),
            projection: Matrix::identity(),
            mvp: Matrix::identity()
        }
    }

    fn set_view(&mut self, m: Matrix) {
        self.view = m;
    }

    fn get_view(&self) -> &Matrix {
        &self.view
    }

    fn set_model(&mut self, m: Matrix) {
        self.model = m;
    }

    fn get_model(&self) -> &Matrix {
        &self.model
    }

    fn set_projection(&mut self, m: Matrix) {
        self.projection = m;
    }

    fn get_projection(&self) -> &Matrix {
        &self.projection
    }

    fn compute_mvp(&mut self) {
        self.mvp = self.projection.mul(&self.view).mul(&self.model);
    }
}


fn get_view_matrix(eye: Vector, at: Vector, mut up: Vector) -> Matrix {
    let mut g = at.sub(&eye);
    g.normlize();
    up.normlize();
    let x = g.cross_product(&up);
    Matrix { 
        m: vec![
            vec![x.x, x.y, x.z, -eye.x],
            vec![up.x, up.y, up.z, -eye.y],
            vec![-g.x, -g.y, -g.z, -eye.z],
            vec![0.0, 0.0, 0.0, 1.0]]
    }
}

fn get_model_matrix() -> Matrix {
    Matrix::identity()
}

fn get_ortho_projection_matrix(l: f32, r: f32, t: f32, b: f32, n: f32, f: f32) -> Matrix {
    Matrix {
        m: M
    }
}

fn get_projection_matrix(t: ProjectionType) -> Matrix {
    match t {
        ProjectionType::Orthographic => {
            Matrix {
                m: vec![
                    vec![]
                ]
            }
        },

        ProjectionType::Prespective => {
            Matrix {
                m: todo!(),
            }
        }
    }
}
