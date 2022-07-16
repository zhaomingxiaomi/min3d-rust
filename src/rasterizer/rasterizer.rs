use crate::math::{matrix::Matrix, vector::{Vector, Point3f, Color, Point2f}, utils::clamp};
use super::triangle::{Triangle, Vertex, vertex_interp};

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
        if p0.v.y > p1.v.y {
            if p0.v.y < p2.v.y {
                (p1, p0, p2)
            } else if p1.v.y > p2.v.y {
                (p2, p1, p0)
            } else {
                (p1, p2, p0)
            }
        } else {
            if p1.v.y < p2.v.y {
                (p0, p1, p2)
            } else if p0.v.y > p2.v.y {
                (p2, p0, p1)
            } else {
                (p0, p2, p1)
            }
        }
    };

    if min.v.y == mid.v.y && min.v.y == max.v.y {
        return traps;
    } else if min.v.x == mid.v.x && min.v.x == max.v.x {
        return traps;
    }

    if min.v.y == mid.v.y {
        let mut t: Trapezoid = Trapezoid::default();
        t.t = min.v.y;
        t.b = max.v.y;
        if min.v.x < mid.v.x {
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

    if max.v.y == mid.v.y {
        let mut t: Trapezoid = Trapezoid::default();
        t.t = min.v.y;
        t.b = max.v.y;
        if max.v.x < mid.v.x {
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

    t1.t = min.v.y;
    t1.b = mid.v.y;
    t2.t = mid.v.y;
    t2.b = max.v.y;

    //直接判断mid和max的x值，小的在左边
    if mid.v.x < max.v.x {
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

    let s1 = trap.l2.unwrap().v.y - trap.l1.unwrap().v.y;
    let s2 = trap.r2.unwrap().v.y - trap.r1.unwrap().v.y;
    let y1 = clamp((y - trap.l1.unwrap().v.y) / s1, 0.0, 1.0);
    let y2 = clamp((y - trap.r1.unwrap().v.y) / s2, 0.0, 1.0);


    trap.l = Some(vertex_interp(&trap.l1.unwrap(), &trap.l2.unwrap(), y1));
    trap.r = Some(vertex_interp(&trap.r1.unwrap(), &trap.r2.unwrap(), y2));
}

fn trapezoid_get_step(trap: &Trapezoid) -> Vertex {
    let r = trap.r.as_ref().unwrap();
    let l = trap.l.as_ref().unwrap();
    let w = 1.0 / (r.v.x - l.v.x);
    Vertex {
        v: Vector { 
            x: (r.v.x - l.v.x) * w, 
            y: (r.v.y - l.v.y) * w, 
            z: (r.v.z - l.v.z) * w, 
            w: (r.v.w - l.v.w) * w},
        color: Color {
            r: (r.color.r - l.color.r) * w, 
            g: (r.color.g - l.color.g) * w, 
            b: (r.color.b - l.color.b) * w, 
        },

        ws: (r.ws - l.ws) / w,
        tex_coords: Point2f::default(),
        normal: Point3f::default(),
    }
}

fn trapezoid_init_scanline(trap: &Trapezoid, y: i32) -> Scanline {
    let w = trap.r.as_ref().unwrap().v.x.round() as i32 - trap.l.as_ref().unwrap().v.x.round() as i32;
    let x = trap.l.as_ref().unwrap().v.x.round() as i32;

    Scanline { 
        step: trapezoid_get_step(trap),
        x: x,
        y: y,
        w: w
    }
}

fn trapezoid_draw_scanline(image: &mut Vec<u8>, width: i32, trap: &Trapezoid, scanline: &Scanline) {
    let start = trap.l.as_ref().unwrap();
    let mut r = start.color.r;
    let mut g = start.color.g;
    let mut b = start.color.b;
    let mut w = start.ws;


    for i in 0..scanline.w {
        if scanline.x + i < 0 || scanline.x + i >= width {continue;}
        let cur_r = clamp((255.0 * r / w) as i32, 0, 255);
        let cur_g = clamp((255.0 * g / w) as i32, 0, 255);
        let cur_b = clamp((255.0 * b / w) as i32, 0, 255);

        image[((width * scanline.y + scanline.x + i) * 4) as usize] = cur_b as u8;
        image[((width * scanline.y + scanline.x + i) * 4 + 1) as usize] = cur_g as u8;
        image[((width * scanline.y + scanline.x + i) * 4 + 2) as usize] = cur_r as u8;
        image[((width * scanline.y + scanline.x + i) * 4 + 3) as usize] = 255;

        r += scanline.step.color.r;
        g += scanline.step.color.g;
        b += scanline.step.color.b;
        w += scanline.step.ws;
    }
}

fn trapezoid_draw(image: &mut Vec<u8>, 
    width: i32, 
    height: i32, trap: &mut Trapezoid) {
    let t = trap.t.floor() as i32;
    let b = trap.b.floor() as i32;

    for i in t..=b {
        if i >= 0 && i < height {
            trapezoid_interpation(trap, i as f32 + 0.5);
            let scanline = trapezoid_init_scanline(trap, i);
            trapezoid_draw_scanline(image, width, trap, &scanline);
        }
    }
}

pub struct Rasterizer {
    model: Matrix,
    view: Matrix,
    projection: Matrix,
    mvp: Matrix,
    
    //view port
    //width: i32,
    //height: i32,

    //image
    //buf: Option<Vec<u8>>
}

impl Rasterizer {
    pub fn new() -> Rasterizer {
        Rasterizer {
            model: Matrix::identity(),
            view: Matrix::identity(),
            projection: Matrix::identity(),
            mvp: Matrix::identity(),
        }
    }

    pub fn set_view(&mut self, m: Matrix) {
        self.view = m;

    }

    pub fn set_model(&mut self, m: Matrix) {
        self.model = m;
    }

    pub fn set_projection(&mut self, m: Matrix) {
        self.projection = m;
    }

    pub fn compute_mvp(&mut self) {
        self.mvp = self.projection.mul(&self.view).mul(&self.model);
    }
}

pub fn draw_trangle(rasterizer: &Rasterizer, 
    image: &mut Vec<u8>, 
    width: i32, 
    height: i32, 
    mut triangle: Triangle) {
    let t1 = rasterizer.mvp.apply(&triangle.vertexs[0].v);
    let t2 = rasterizer.mvp.apply(&triangle.vertexs[1].v);
    let t3 = rasterizer.mvp.apply(&triangle.vertexs[2].v);

    let view_port = get_view_port(width as f32, height as f32);
    let mut p1 = view_port.apply(&t1);
    let mut p2 = view_port.apply(&t2);
    let mut p3 = view_port.apply(&t3);

    triangle.set_vertexs(vec![p1.divide_w(), p2.divide_w(), p3.divide_w()]);
    let mut traps = trapezoid_init(&triangle.vertexs[0], &triangle.vertexs[1], &triangle.vertexs[2]);
    if traps.len() >= 1 {
        let trap = &mut traps[0];
        trapezoid_draw(image, width, height, trap);
    }

    if traps.len() >= 2 {
        let trap = &mut traps[1];
        trapezoid_draw(image, width, height, trap);
    }
}

pub fn get_view_matrix(eye: Vector, at: Vector, mut up: Vector) -> Matrix {
    let mut g = at.sub(&eye);
    g.normlize();
    up.normlize();
    let mut x = g.cross_product(&up);
    x.normlize();
    Matrix { 
        m: vec![
            vec![x.x, x.y, x.z, -eye.x],
            vec![up.x, up.y, up.z, -eye.y],
            vec![-g.x, -g.y, -g.z, -eye.z],
            vec![0.0, 0.0, 0.0, 1.0]]
    }
}

pub fn get_view_port(width: f32, height: f32) -> Matrix {
    Matrix { 
        m: vec![
            vec![width/2.0, 0.0, 0.0, width/2.0],
            vec![0.0, -height/2.0, 0.0, height/2.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]
    }
}

pub fn get_model_matrix(angel: f32) -> Matrix {
    let r = std::f32::consts::PI * angel / 180.0;
    Matrix {
        m: vec![
            vec![r.cos(), 0.0, r.sin(), 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![-r.sin(), 0.0, r.cos(), 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]
    }
}

pub fn get_ortho_projection_matrix(l: f32, r: f32, t: f32, b: f32, n: f32, f: f32) -> Matrix {
    Matrix { 
        m: vec![
            vec![2.0/(r - l), 0.0, 0.0, 0.0],
            vec![0.0, 2.0/(t - b), 0.0, 0.0],
            vec![0.0, 0.0, 2.0/(n - f), 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]
    } * Matrix {
        m: vec![
            vec![1.0, 0.0, 0.0, -(l+r)/2.0],
            vec![0.0, 1.0, 0.0, -(t+b)/2.0],
            vec![0.0, 0.0, 1.0, -(n+f)/2.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]
    }
}

pub fn get_presp_projection_matrix(eye_fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Matrix {
    let angle = eye_fov * std::f32::consts::PI / 180.0;
    let height = near * angle.tan();
    //let width = height * aspect_ratio;

    let t = near.abs() * (angle/2.0).tan();
    let r = t * aspect_ratio;
    let l = -r;
    let b = -t;

    get_ortho_projection_matrix(l, r, t, b, near, far) * Matrix {
        m: vec![
            vec![near, 0.0, 0.0, 0.0],
            vec![0.0, near, 0.0, 0.0],
            vec![0.0, 0.0, near+far, -near*far],
            vec![0.0, 0.0, 1.0, 0.0],
        ]
    }
}
