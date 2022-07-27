use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::process::Output;
use crate::math::vector1::Vector;


type Mat4x4f = Matrix<4, 4, f32>;

pub struct Matrix<const M: usize, const N: usize, T>{
    pub m: Vec<Vec<T>>
}

impl<const M: usize, const N: usize, T> Matrix<M, N, T> where T: Default + Clone {
    pub fn new() -> Matrix<M, N, T> {
        Matrix {
            m: vec![vec![T::default(); N]; M]
        }
    }
}

impl<const M: usize, const N: usize, T> Matrix<M, N, T> where T: Default + Clone + Mul<Output = T> + Add<Output = T>{
    pub fn apply(&self, v: &Vector<N, T>) -> Vector<M, T> {
        let mut r = Vec::new();
        for i in 0..M {
            r.push({
                let mut cur = T::default();
                for j in 0..N {
                    cur = cur + self.m[i][j].clone() * v.v[j].clone();
                }

                cur
            });
        }

        Vector { v: r }
    }
}

impl<const M: usize, const N: usize> Matrix<M, N, f32> {
    pub fn identity() -> Matrix<M, N, f32> {
        let mut index = 0;
        let mut m = vec![vec![f32::default(); N]; M];
        for i in 0..M {
            m[i][index] += 1.0;
            index += 1;
        }
        Matrix {
            m: m
        }
    }
}

impl<const M: usize, const N: usize, T> Add for Matrix<M, N, T> where T: Default + Add<Output = T> + Clone {
    type Output = Matrix<M, N, T>;
    fn add(self, rhs: Self) -> Self::Output {
        let mut m = vec![vec![T::default(); N]; M];
        for i in 0..M {
            for j in 0..N {
                m[i][j] = self.m[i][j].clone() + rhs.m[i][j].clone();
            }
            
        }
        Matrix {
            m: m
        }
    }
}

impl<const M: usize, const N: usize, T> Sub for Matrix<M, N, T> where T: Default + Sub<Output = T> + Clone {
    type Output = Matrix<M, N, T>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut m = vec![vec![T::default(); N]; M];
        for i in 0..M {
            for j in 0..N {
                m[i][j] = self.m[i][j].clone() - rhs.m[i][j].clone();
            }
            
        }
        Matrix {
            m: m
        }
    }
}

impl<const N: usize, T> Mul for Matrix<N, N, T> where T:Default + Clone + Mul<Output = T> + Add<Output = T> {
    type Output = Matrix<N, N, T>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut m = vec![vec![T::default(); N]; N];
        for i in 0..N {
            for j in 0..N {
                m[i][j] = {
                    let mut r = T::default();
                    for k in 0..N {
                        r = r + self.m[i][k].clone() * rhs.m[k][j].clone();
                    }
                    r
                }
            }
            
        }
        Matrix {
            m: m
        }
    }
}

