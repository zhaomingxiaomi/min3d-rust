use image::{RgbaImage, Pixel};

use crate::math::utils::clamp;

pub struct Texture {
    id: i32,
    image: RgbaImage,
    width: u32,
    height: u32
}

impl Texture {
    pub fn new(id: i32, path: &str)-> Texture {
        let r = image::open(path).unwrap().to_rgba8();
        Texture { 
            id: id,
            width: r.width(), 
            height: r.height(),
            image: r, 
        }
    }

    pub fn get_color(&self, u: f32, v: f32) -> (u8, u8, u8) {
        let x = clamp((u * self.width as f32).round() as u32, 0, self.width-1);
        let y = clamp((v * self.height as f32).round() as u32, 0, self.height-1);

        let r = self.image.get_pixel(x, y).to_rgba();
        (r.0[0], r.0[1], r.0[2])
    }
}