use super::math::vector::Vector;
use super::math::matrix::Matrix;

pub struct Rasterizer {
    model: Matrix,
    view: Matrix,
    projection: Matrix,
    mvp: Matrix
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

    fn set_view(&self, m: Matrix) {
        self.view = m;
    }

    fn get_view(&self) -> &Matrix {
        self.view
    }

    fn set_model(&self, m: Matrix) {
        view = m;
    }

    fn get_model(&self) -> &Matrix {
        self.model
    }

    fn set_projection(&self, m: Matrix) {
        self.projection = m;
    }

    fn get_projection(&self) -> &Matrix {
        self.projection
    }

    fn compute_mvp(&mut self) {
        self.mvp = self.model * self.view * self.model;
    }
}


fn model_set_lookat(eye: Vector, at: Vector, up: Vector) -> Matrix {
    let g = at - eye;
}
