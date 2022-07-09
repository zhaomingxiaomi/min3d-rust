use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;

use super::vector::Vector;


pub struct Matrix {
    pub m: Vec<Vec<f32>>
}

impl Matrix {
    pub fn new() -> Matrix {
        Matrix {
            m: vec![vec![0.0; 4]; 4]
        }
    }

    pub fn identity() -> Matrix {
        Matrix {
            m: vec![vec![1.0, 0.0, 0.0, 0.0], 
                    vec![0.0, 1.0, 0.0, 0.0],
                    vec![0.0, 0.0, 1.0, 0.0],
                    vec![0.0, 0.0, 0.0, 0.0],
                    ]
        }
    }

    pub fn apply(&self, v: &Vector) -> Vector {
        Vector {
            x: v.x * self.m[0][0] + v.y * self.m[0][1] + v.z * self.m[0][2] + v.w * self.m[0][3],
            y: v.x * self.m[1][0] + v.y * self.m[1][1] + v.z * self.m[1][2] + v.w * self.m[1][3],
            z: v.x * self.m[2][0] + v.y * self.m[2][1] + v.z * self.m[2][2] + v.w * self.m[2][3],
            w: v.x * self.m[3][0] + v.y * self.m[3][1] + v.z * self.m[3][2] + v.w * self.m[3][3]
        }
    }
}

impl Add for Matrix {
    type Output = Matrix;
    fn add(self, rhs: Self) -> Self::Output {
        let add_r = self.m.iter().enumerate().fold(vec![], |mut a, b| {
            a.push(b.1.iter().enumerate().fold(vec![], |mut c, d| {
                c.push(rhs.m[b.0][d.0] + d.1);
                c
            }));
            a
        });

        Matrix {
            m: add_r
        }
    }
}

impl Sub for Matrix {
    type Output = Matrix;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut sub_r = Vec::new();
        for i in 0..self.m.len() {
            let mut cur = Vec::new();
            for j in 0..self.m.len() {
                cur.push(self.m[i][j] - rhs.m[i][j]);
            }
            sub_r.push(cur);
        }
        Matrix {
            m: sub_r
        }
    }
}

impl Mul for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut mul_r = Vec::new();
        for i in 0..self.m.len() {
            let mut cur = Vec::new();
            for j in 0..self.m.len() {
                cur.push(self.m[i][0] * rhs.m[0][j]
                        + self.m[i][1] * rhs.m[1][j]
                        + self.m[i][2] * rhs.m[2][j]
                        + self.m[i][3] * rhs.m[3][j]);
            }
            mul_r.push(cur);
        }

        Matrix {
            m: mul_r
        }
    }
}