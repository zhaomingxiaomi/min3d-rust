use std::ops::{Mul, Add, Sub};

type Vector2f = Vector<2, f32>;
type Vector3f = Vector<3, f32>;
type Vector4f = Vector<4, f32>;

pub struct Vector<const N: usize, T>{
    pub v: Vec<T>
}

impl<const N: usize, T> Vector<N, T> where T: Default + Clone {
    fn new() -> Vector<N, T> {
        Vector { v: vec![T::default(); N] }
    }
}

impl<const N: usize, T> Add for Vector<N, T> where T: Default + Clone + Add<Output = T> {
    type Output = Vector<N, T>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut r = Vec::new();
        for i in 0..N {
            r.push(self.v[i].clone() + rhs.v[i].clone());
        }

        Vector {
            v: r
        }
    }
}

impl<const N: usize, T> Sub for Vector<N, T> where T: Default + Clone + Sub<Output = T> {
    type Output = Vector<N, T>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut r = Vec::new();
        for i in 0..N {
            r.push(self.v[i].clone() - rhs.v[i].clone());
        }

        Vector {
            v: r
        }
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

    pub fn dot_product(&mut self, v2: &Vector<N, f32>) -> f32 {
        let mut r = 0.0;
        for i in 0..self.v.len() {
            r += self.v[i] * v2.v[i];
        }

        r
    }
}

impl<const N: usize, T> Vector<N, T> where T: Clone {
    fn x(&self) -> T {
        return self.v[0].clone();
    }

    fn u(&self) -> T {
        return self.v[0].clone();
    }

    fn r(&self) -> T {
        return self.v[0].clone();
    }

    fn y(&self) -> T {
        return self.v[1].clone();
    }

    fn v(&self) -> T {
        return self.v[1].clone();
    }

    fn g(&self) -> T {
        return self.v[1].clone();
    }

    fn z(&self) -> T {
        return self.v[2].clone();
    }

    fn b(&self) -> T {
        return self.v[2].clone();
    }

    fn w(&self) -> T {
        return self.v[3].clone();
    }
}

impl Vector<4, f32> {
    fn cross_product(&self, v1: &Vector<4, f32>) -> Vector<4, f32> {
        Vector {
            v: vec![
                self.v[1] * v1.v[2] - self.v[2] * v1.v[1],
                self.v[2] * v1.v[0] - self.v[0] * v1.v[2],
                self.v[0] * v1.v[1] - self.v[1] * v1.v[0],
                1.0
            ]
        }
    }
}