use crate::math::{utils::clamp, vector::{Vector4f, Color3f, Point2f, Point3f}};

use super::triangle::{Vertex, vertex_interp, Triangle};

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

    //直接判断mid和max的x值，小的在左边
    if mid.v.x() < max.v.x() {
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
        v: Vector4f::new_4(
            (r.v.x() - l.v.x()) * w, 
            (r.v.y() - l.v.y()) * w, 
            (r.v.z() - l.v.z()) * w, 
            (r.v.w() - l.v.w()) * w),
        color: Color3f::new_3(
            (r.color.r() - l.color.r()) * w, 
            (r.color.g() - l.color.g()) * w, 
            (r.color.b() - l.color.b()) * w),

        tex_coords: Point2f::new(),
        normal: Point3f::new(),
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

fn trapezoid_draw_scanline(image: &mut Vec<u8>, width: i32, zbuf: &mut Vec<f32>, trap: &Trapezoid, scanline: &Scanline) {
    let start = trap.l.as_ref().unwrap();
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

fn trapezoid_draw(image: &mut Vec<u8>, 
    width: i32, 
    height: i32, zbuf: &mut Vec<f32>, trap: &mut Trapezoid) {
    let t = trap.t.floor() as i32;
    let b = trap.b.floor() as i32;

    for i in t..=b {
        if i >= 0 && i < height {
            trapezoid_interpation(trap, i as f32 + 0.5);
            let scanline = trapezoid_init_scanline(trap, i);
            trapezoid_draw_scanline(image, width, zbuf, trap, &scanline);
        }
    }
}

pub fn draw_trangle_edge_walking(image: &mut Vec<u8>, zbuf: &mut Vec<f32>, width: i32, height: i32, triangle: &Triangle) {
    let mut traps = trapezoid_init(&triangle.vertexs[0], &triangle.vertexs[1], &triangle.vertexs[2]);
    if traps.len() >= 1 {
        let trap = &mut traps[0];
        trapezoid_draw(image, width, height, zbuf, trap);
    }

    if traps.len() >= 2 {
        let trap = &mut traps[1];
        trapezoid_draw(image, width, height, zbuf, trap);
    }
}
