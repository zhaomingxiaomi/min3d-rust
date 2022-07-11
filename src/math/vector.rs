use std::ops::{Add, Sub};

use super::matrix::Matrix;

pub struct Point2f {
    pub x: f32,
    pub y: f32
}

pub struct Point3f {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}



impl Vector {
    fn new(x: f32, y: f32, z: f32, w: f32) -> Vector {
        Vector {
            x, y, z, w
        }
    }

    pub fn default() -> Vector {
        Vector {
            x: 0.0, y: 0.0, z: 0.0, w: 0.0, 
        }
    }

    pub fn length(&self) -> f32 {
        return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
    }

    pub fn normlize(&mut self) {
        let l = self.length();
        self.x = self.x / l;
        self.y = self.y / l;
        self.z = self.z / l;
        self.w = self.w / l;
    }

    pub fn cross_product(&self, v: &Vector) -> Vector {
        Vector {
            x: self.y * v.z - self.z * v.x,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.z * v.x,
            w: 1.0
        }
    }

    pub fn dot_product(&self, v: &Vector) -> f32 {
        return self.x * v.x + self.y * v.y + self.z * v.z;
    }

    pub fn apply(&self, m: &Matrix) -> Vector {
        Vector {
            x: self.x * m.m[0][0] + self.y * m.m[1][0] + self.z * m.m[2][0] + self.w * m.m[3][0],
            y: self.x * m.m[0][0] + self.y * m.m[1][1] + self.z * m.m[2][1] + self.w * m.m[3][1],
            z: self.x * m.m[0][0] + self.y * m.m[1][2] + self.z * m.m[2][2] + self.w * m.m[3][2],
            w: self.x * m.m[0][0] + self.y * m.m[1][3] + self.z * m.m[3][3] + self.w * m.m[3][3]
        }
    }

    pub fn add(&self, v: &Vector) -> Vector {
        Vector {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
            w: 1.0
        }
    }

    pub fn sub(&self, rhs: &Self) -> Vector {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: 1.0
        }
    }

}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: 1.0
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: 1.0
        }
    }
}