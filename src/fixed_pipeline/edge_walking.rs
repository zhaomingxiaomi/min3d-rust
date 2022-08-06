use crate::{math::{utils::clamp, vector::{Vector4f, Color3f, Point2f, Point3f, Vector3f}}, common::{triangle::{RenderType, self}, texture::Texture, light::compute_light}};

use crate::common::triangle::{Vertex, vertex_interp, Triangle};

use super::rasterizer::{self, Rasterizer};

struct Trapezoid<'a > {
    t: f32,
    b: f32,

    l: Option<Vertex>,
    r: Option<Vertex>,
    l1: Option<&'a Vertex>,
    l2: Option<&'a Vertex>,
    r1: Option<&'a Vertex>,
    r2: Option<&'a Vertex>
}


struct Scanline {
    step: Vertex,
    x: i32,
    y: i32,
    w: i32
}


impl<'a> Default for Trapezoid<'a> {
    fn default() -> Self {
        Self { 
            t: Default::default(), 
            b: Default::default(), 
            l1: Default::default(), 
            l2: Default::default(), 
            r1: Default::default(), 
            r2: Default::default(), 
            l: Default::default(),
            r: Default::default()
         }
    }
}

fn trapezoid_init<'a>(p0: &'a Vertex, p1: &'a Vertex, p2: &'a Vertex) -> Vec<Trapezoid<'a>> {
    let mut traps = Vec::new();
    let (min, mid, max) = {
        if p0.v.y() > p1.v.y() {
            if p0.v.y() < p2.v.y() {
                (p1, p0, p2)
            } else if p1.v.y() > p2.v.y() {
                (p2, p1, p0)
            } else {
                (p1, p2, p0)
            }
        } else {
            if p1.v.y() < p2.v.y() {
                (p0, p1, p2)
            } else if p0.v.y() > p2.v.y() {
                (p2, p0, p1)
            } else {
                (p0, p2, p1)
            }
        }
    };

    if min.v.y() == mid.v.y() && min.v.y() == max.v.y() {
        return traps;
    } else if min.v.x() == mid.v.x() && min.v.x() == max.v.x() {
        return traps;
    }

    if min.v.y() == mid.v.y() {
        let mut t: Trapezoid = Trapezoid::default();
        t.t = min.v.y();
        t.b = max.v.y();
        if min.v.x() < mid.v.x() {
            t.l1 = Some(min);
            t.l2 = Some(max);

            t.r1 = Some(mid);
            t.r2 = Some(max);

        } else {
            t.l1 = Some(mid);
            t.l2 = Some(max);

            t.r1 = Some(min);
            t.r2 = Some(max);
        }

        //这里应该不需要判断t.t和t.b大小，应该已经确认t.t<t.b了
        traps.push(t);
        return traps;
    }

    if max.v.y() == mid.v.y() {
        let mut t: Trapezoid = Trapezoid::default();
        t.t = min.v.y();
        t.b = max.v.y();
        if max.v.x() < mid.v.x() {
            t.l1 = Some(min);
            t.l2 = Some(max);

            t.r1 = Some(min);
            t.r2 = Some(mid);

        } else {
            t.l1 = Some(min);
            t.l2 = Some(mid);

            t.r1 = Some(min);
            t.r2 = Some(max);
        }

        //这里应该不需要判断t.t和t.b大小，应该已经确认t.t<t.b了
        traps.push(t);
        return traps;
    }

    let mut t1 = Trapezoid::default();
    let mut t2 = Trapezoid::default();

    t1.t = min.v.y();
    t1.b = mid.v.y();
    t2.t = mid.v.y();
    t2.b = max.v.y();

    //wrong: 直接判断mid和max的x值，小的在左边
    //需要判断和max和min连线上，y值与min.y相同的点之间x的大小
    let f = (mid.v.y() - max.v.y()) / (min.v.y() - max.v.y());
    let x = max.v.x() + f * (min.v.x() - max.v.x());
    if mid.v.x() < x {
        t1.l1 = Some(min);
        t1.l2 = Some(mid);
        t1.r1 = Some(min);
        t1.r2 = Some(max);

        t2.l1 = Some(mid);
        t2.l2 = Some(max);
        t2.r1 = Some(min);
        t2.r2 = Some(max);
    } else {
        t1.l1 = Some(min);
        t1.l2 = Some(max);
        t1.r1 = Some(min);
        t1.r2 = Some(mid);

        t2.l1 = Some(min);
        t2.l2 = Some(max);
        t2.r1 = Some(mid);
        t2.r2 = Some(max);
    }

    traps.push(t1);
    traps.push(t2);

    return traps;
}

fn trapezoid_interpation(trap: &mut Trapezoid, y: f32) {

    let s1 = trap.l2.unwrap().v.y() - trap.l1.unwrap().v.y();
    let s2 = trap.r2.unwrap().v.y() - trap.r1.unwrap().v.y();
    let y1 = clamp((y - trap.l1.unwrap().v.y()) / s1, 0.0, 1.0);
    let y2 = clamp((y - trap.r1.unwrap().v.y()) / s2, 0.0, 1.0);

    trap.l = Some(vertex_interp(&trap.l1.unwrap(), &trap.l2.unwrap(), y1));
    trap.r = Some(vertex_interp(&trap.r1.unwrap(), &trap.r2.unwrap(), y2));
}

fn trapezoid_get_step(trap: &Trapezoid) -> Vertex {
    let r = trap.r.as_ref().unwrap();
    let l = trap.l.as_ref().unwrap();
    let w = 1.0 / (r.v.x() - l.v.x());
    Vertex {
        origin_v: Vector4f::new_4(
            (r.origin_v.x() - l.origin_v.x()) * w, 
            (r.origin_v.y() - l.origin_v.y()) * w, 
            (r.origin_v.z() - l.origin_v.z()) * w, 
            (r.origin_v.w() - l.origin_v.w()) * w),
        tv: Vector4f::new_4(
            (r.tv.x() - l.tv.x()) * w, 
            (r.tv.y() - l.tv.y()) * w, 
            (r.tv.z() - l.tv.z()) * w, 
            (r.tv.w() - l.tv.w()) * w),
        v: Vector4f::new_4(
            (r.v.x() - l.v.x()) * w, 
            (r.v.y() - l.v.y()) * w, 
            (r.v.z() - l.v.z()) * w, 
            (r.v.w() - l.v.w()) * w),
        color: Color3f::new_3(
            (r.color.r() - l.color.r()) * w, 
            (r.color.g() - l.color.g()) * w, 
            (r.color.b() - l.color.b()) * w),

        tex_coords: Point2f::new_2(
            (r.tex_coords.u() - l.tex_coords.u()) * w, 
            (r.tex_coords.v() - l.tex_coords.v()) * w, 
        ),
        normal: Point3f::new_3(
            (r.normal.x() - l.normal.x()) * w, 
            (r.normal.y() - l.normal.y()) * w, 
            (r.normal.z() - l.normal.z()) * w, 
        ),
        rhw: (r.rhw - l.rhw) * w, 
    }
}

fn trapezoid_init_scanline(trap: &Trapezoid, y: i32) -> Scanline {
    let w = trap.r.as_ref().unwrap().v.x().round() as i32 - trap.l.as_ref().unwrap().v.x().round() as i32;
    let x = trap.l.as_ref().unwrap().v.x().round() as i32;

    Scanline { 
        step: trapezoid_get_step(trap),
        x: x,
        y: y,
        w: w
    }
}

fn trapezoid_draw_scanline(image: &mut Vec<u8>, rasterizer: &Rasterizer, render: &RenderType, width: i32, zbuf: &mut Vec<f32>, trap: &Trapezoid, scanline: &Scanline, textures: &Vec<Texture>, triangle: &Triangle) {
    let start = trap.l.as_ref().unwrap();
    match render {
        RenderType::COLOR => {
            let mut r = start.color.r();
            let mut g = start.color.g();
            let mut b = start.color.b();
            let mut z = start.v.z();
        
            
            for i in 0..scanline.w {
                if scanline.x + i < 0 || scanline.x + i >= width {continue;}
                let index = width * scanline.y + scanline.x + i;
                if z >= zbuf[index as usize] {
                    zbuf[index as usize] = z;
                    let cur_r = clamp((255.0 * r) as i32, 0, 255);
                    let cur_g = clamp((255.0 * g) as i32, 0, 255);
                    let cur_b = clamp((255.0 * b) as i32, 0, 255);

            
                    image[(index * 4) as usize] = cur_b as u8;
                    image[(index * 4 + 1) as usize] = cur_g as u8;
                    image[(index * 4 + 2) as usize] = cur_r as u8;
                    image[(index * 4 + 3) as usize] = 255;
                }
        
                r += scanline.step.color.r();
                g += scanline.step.color.g();
                b += scanline.step.color.b();
                z += scanline.step.v.z();
            }
        }
        RenderType::TEXTURE => {
            let mut u = start.tex_coords.u();
            let mut v = start.tex_coords.v();
            let mut normal_x = start.normal.x();
            let mut normal_y = start.normal.y();
            let mut normal_z = start.normal.z();

            let mut x = start.tv.x();
            let mut y = start.tv.y();
            let mut z = start.tv.z();
            for i in 0..scanline.w {
                if scanline.x + i < 0 || scanline.x + i >= width {
                    continue;
                }
                let index = width * scanline.y + scanline.x + i;
                if z >= zbuf[index as usize] {
                    zbuf[index as usize] = z;
                    {
                        let (mut r, mut g, mut b) = textures[0].get_color(u, 1.0-v);

                        if rasterizer.get_lights().len() > 0 {
                            let result = compute_light(
                                &Vector3f::new_3(x, y, z), 
                                &Vector3f::new_3(normal_x, normal_y, normal_z), 
                                rasterizer.get_lights(),
                                &Vector3f::new_3(0.005, 0.005, 0.005),
                                &Vector3f::new_3(r as f32  / 255.0, g as f32 / 255.0, b as f32 / 255.0),
                                &Vector3f::new_3(0.7937, 0.7937, 0.7937),
                                rasterizer.get_eye_pos());

                            r = (result.r() * 255.0) as u8;
                            g = (result.g() * 255.0) as u8;
                            b = (result.b() * 255.0) as u8;
                        }


                        image[(index * 4) as usize] = b;
                        image[(index * 4 + 1) as usize] = g;
                        image[(index * 4 + 2) as usize] = r;
                        image[(index * 4 + 3) as usize] = 255;
                    }
                }
        
                u += scanline.step.tex_coords.u();
                v += scanline.step.tex_coords.v();
                x += scanline.step.tv.x();
                y += scanline.step.tv.y();
                z += scanline.step.tv.z();
                normal_x += scanline.step.normal.x();
                normal_y += scanline.step.normal.y();
                normal_z += scanline.step.normal.z();
                z += scanline.step.v.z();
            }
        }
    }

}

fn trapezoid_draw(image: &mut Vec<u8>, 
    rasterizer: &Rasterizer,
    rendertype: &RenderType,
    width: i32, 
    height: i32, 
    zbuf: &mut Vec<f32>, 
    trap: &mut Trapezoid, 
    textures: &Vec<Texture>,
    triangle: &Triangle) {
    let t = trap.t.floor() as i32;
    let b = trap.b.floor() as i32;

    for i in t..=b {
        if i >= 0 && i < height {
            trapezoid_interpation(trap, i as f32 + 0.5);
            let scanline = trapezoid_init_scanline(trap, i);
            if scanline.w < 0 {
                //panic!("wrong situtation!");
                println!("Wrong situtation!, w = {}", scanline.w);
                // let mut traps = trapezoid_init(&triangle.vertexs[0], &triangle.vertexs[1], &triangle.vertexs[2]);
                // println!("{:?}", traps[0].l1.unwrap().v);
                // println!("{:?}", traps[0].l2.unwrap().v);
                // println!("{:?}", traps[0].r1.unwrap().v);
                // println!("{:?}", traps[0].r2.unwrap().v);
            
                // println!("{:?}", traps[0].l1.unwrap().v);
                // println!("{:?}", traps[0].l2.unwrap().v);
                // println!("{:?}", traps[0].r1.unwrap().v);
                // println!("{:?}", traps[0].r2.unwrap().v);

            }
            trapezoid_draw_scanline(image, rasterizer, rendertype, width, zbuf, trap, &scanline, textures, triangle);
        }
    }
}

pub fn draw_trangle_edge_walking(image: &mut Vec<u8>, 
    rasterizer: &Rasterizer,
    zbuf: &mut Vec<f32>, 
    width: i32, height: i32, 
    triangle: &Triangle, textures: &Vec<Texture>) {
    let mut traps = trapezoid_init(&triangle.vertexs[0], &triangle.vertexs[1], &triangle.vertexs[2]);
    if traps.len() >= 1 {
        let trap = &mut traps[0];
        trapezoid_draw(image, rasterizer, &triangle.render, width, height, zbuf, trap, textures, triangle);
    }

    if traps.len() >= 2 {
        let trap = &mut traps[1];
        trapezoid_draw(image, rasterizer, &triangle.render, width, height, zbuf, trap, textures, triangle);
    }
}
