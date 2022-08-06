use std::ops::{Add, Sub};

use super::utils::interpolation;

pub type Vector2f = Vector<2, f32>;
pub type Vector3f = Vector<3, f32>;
pub type Vector4f = Vector<4, f32>;

pub type Color3f = Vector<3, f32>;
pub type Point3f = Vector<3, f32>;
pub type Point2f = Vector<2, f32>;

#[derive(Clone, Debug)]
pub struct Vector<const N: usize, T> {
    pub v: Vec<T>
}

impl<const N: usize, T> Vector<N, T> where T: Default + Clone {
    pub fn new() -> Vector<N, T> {
        Vector { v: vec![T::default(); N] }
    }
}

impl<T> Vector<2, T> where T: Default + Clone {
    pub fn new_2(x: T, y: T) -> Vector<2, T> {
        Vector { v: vec![x, y] }
    }
}

impl<T> Vector<3, T> where T: Default + Clone {
    pub fn new_3(x: T, y: T, z: T) -> Vector<3, T> {
        Vector { v: vec![x, y, z] }
    }
}

impl<T> Vector<4, T> where T: Default + Clone {
    pub fn new_4(x: T, y: T, z: T, w: T) -> Vector<4, T> {
        Vector { v: vec![x, y, z, w] }
    }
}

impl<const N: usize> Vector<N, f32> {
    pub fn length(&self) -> f32 {
        let mut r = 0.0;
        for v in &self.v {
            r += v*v;
        }

        r.sqrt()
    }

    pub fn normlize(&mut self) {
        let l = self.length();
        for i in 0..self.v.len() {
            self.v[i] = self.v[i] / l;
        }
    }

    pub fn dot_product(&self, v2: &Vector<N, f32>) -> f32 {
        let mut r = 0.0;
        for i in 0..self.v.len() {
            r += self.v[i] * v2.v[i];
        }

        r
    }

    pub fn sub(&self, other: &Vector<N, f32>) -> Vector<N, f32> {
        let mut r = Vec::new();
        for i in 0..N {
            r.push(self.v[i].clone() - other.v[i].clone());
        }

        Vector {
            v: r
        }
    }

    pub fn add(&self, rhs: &Vector<N, f32>) -> Vector<N, f32> {
        let mut r = Vec::new();
        for i in 0..N {
            r.push(self.v[i].clone() - rhs.v[i].clone());
        }

        Vector {
            v: r
        }
    }
}

impl<const N: usize, T> Vector<N, T> where T: Clone {
    pub fn x(&self) -> T {
        return self.v[0].clone();
    }

    pub fn u(&self) -> T {
        return self.v[0].clone();
    }

    pub fn r(&self) -> T {
        return self.v[0].clone();
    }

    pub fn y(&self) -> T {
        return self.v[1].clone();
    }

    pub fn v(&self) -> T {
        return self.v[1].clone();
    }

    pub fn g(&self) -> T {
        return self.v[1].clone();
    }

    pub fn z(&self) -> T {
        return self.v[2].clone();
    }

    pub fn b(&self) -> T {
        return self.v[2].clone();
    }

    pub fn w(&self) -> T {
        return self.v[3].clone();
    }
}

impl Vector<4, f32> {
    pub fn cross_product(&self, v1: &Vector<4, f32>) -> Vector<4, f32> {
        Vector {
            v: vec![
                self.v[1] * v1.v[2] - self.v[2] * v1.v[1],
                self.v[2] * v1.v[0] - self.v[0] * v1.v[2],
                self.v[0] * v1.v[1] - self.v[1] * v1.v[0],
                1.0
            ]
        }
    }

    pub fn divide_w(&mut self) {
        self.v[0] /= self.v[3];
        self.v[1] /= self.v[3];
        self.v[2] /= self.v[3];
        self.v[3] = 1.0;
    }

    pub fn reset_z(&mut self, n: f32, f: f32) {
        let f1 = (n + f) / 2.0;
        let f2 = (n - f) / 2.0;

        self.v[2] = f2 * self.v[2] + f1;
    }
}

pub fn vector4f_interpolation(v1: &Vector4f, v2: &Vector4f, t: f32) -> Vector4f {
    Vector4f::new_4(
        interpolation(v1.x(), v2.x(), t),
        interpolation(v1.y(), v2.y(), t),
        interpolation(v1.z(), v2.z(), t),
        interpolation(v1.w(), v2.w(), t)
    )
}