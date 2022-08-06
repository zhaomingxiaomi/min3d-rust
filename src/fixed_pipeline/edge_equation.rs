use crate::common::light::compute_light;
use crate::common::texture::Texture;
use crate::common::triangle::{RenderType, Triangle};
use crate::math::utils::{interpolate_vector2f, interpolate_vector3f, interpolate_vector4f};
use crate::math::vector::Vector3f;
use crate::math::{utils::clamp, vector::Vector4f};

use super::rasterizer::Rasterizer;

pub fn draw_trangle_edge_equation(
    image: &mut Vec<u8>,
    rasterizer: &Rasterizer,
    zbuf: &mut Vec<f32>,
    width: i32,
    height: i32,
    triangle: &Triangle,
    textures: &Vec<Texture>
) {
    let mut l = std::f32::MAX;
    let mut r = std::f32::MIN;
    let mut t = std::f32::MIN;
    let mut b = std::f32::MAX;


    for i in 0..triangle.vertexs.len() {
        l = clamp(triangle.vertexs[i].v.x().min(l), 0.0, (width - 1) as f32);
        r = clamp(triangle.vertexs[i].v.x().max(r), 0.0, (width - 1) as f32);
        t = clamp(triangle.vertexs[i].v.y().max(t), 0.0, (height - 1) as f32);
        b = clamp(triangle.vertexs[i].v.y().min(b), 0.0, (height - 1) as f32);
    }

    //let (p1, p2, p3) = retriangle(triangle);
    let p1 = &triangle.vertexs[0].v;
    let p2 = &triangle.vertexs[1].v;
    let p3 = &triangle.vertexs[2].v;


    for i in l.round() as i32..=r.round() as i32 {
        for j in b.round() as i32..=t.round() as i32 {
            match triangle.render {
                RenderType::COLOR => {
                    //msaa
                    let pos = vec![(0.25, 0.25), (0.25, 0.75), (0.75, 0.25), (0.75, 0.75)];

                    let mut count = 0.0;
                    let mut min_depth = 100.0;
                    for p in pos {
                        if inside_triangle(i as f32 + p.0, j as f32 + p.1, &p1, &p2, &p3) {
                            let (alpha, beta, gamma) =
                                compute_barycentric_2d(i as f32 + p.0, j as f32 + p.1, triangle);
                            let z = alpha * triangle.vertexs[0].v.z()
                                + beta * triangle.vertexs[1].v.z()
                                + gamma * triangle.vertexs[2].v.z();
                            min_depth = z.min(min_depth);
                            count += 1.0;
                        }
                    }

                    if count > 0.0 && min_depth >= zbuf[(width * j + i) as usize] {
                        zbuf[(width * j + i) as usize] = min_depth;
                        let a = &triangle.vertexs[0].color;
                        let b = &triangle.vertexs[1].color;
                        let c = &triangle.vertexs[2].color;
                        let (alpha, beta, gamma) =
                            compute_barycentric_2d(i as f32 + 0.5, j as f32 + 0.5, triangle);

                        let origin_b = image[((width * j + i) * 4) as usize] as f32 / 255.0;
                        let origin_g = image[((width * j + i) * 4 + 1) as usize] as f32 / 255.0;
                        let origin_r = image[((width * j + i) * 4 + 2) as usize] as f32 / 255.0;

                        let cur_r = clamp(
                            (((a.r() * alpha + b.r() * beta + gamma * c.r()) * count
                                + origin_r * (4.0 - count))
                                * 255.0
                                / 4.0) as i32,
                            0,
                            255,
                        );
                        let cur_g = clamp(
                            (((a.g() * alpha + b.g() * beta + gamma * c.g()) * count
                                + origin_g * (4.0 - count))
                                * 255.0
                                / 4.0) as i32,
                            0,
                            255,
                        );
                        let cur_b = clamp(
                            (((a.b() * alpha + b.b() * beta + gamma * c.b()) * count
                                + origin_b * (4.0 - count))
                                * 255.0
                                / 4.0) as i32,
                            0,
                            255,
                        );

                        image[((width * j + i) * 4) as usize] = cur_b as u8;
                        image[((width * j + i) * 4 + 1) as usize] = cur_g as u8;
                        image[((width * j + i) * 4 + 2) as usize] = cur_r as u8;
                        image[((width * j + i) * 4 + 3) as usize] = 255;
                    }
                }

                RenderType::TEXTURE => {
                    if inside_triangle(i as f32 + 0.5, j as f32 + 0.5, &p1, &p2, &p3) {
                        //println!("{:?}, {:?}", i, j);

                        let (alpha, beta, gamma) =
                            compute_barycentric_2d(i as f32 + 0.5, j as f32 + 0.5, triangle);
        
                        let z = alpha * triangle.vertexs[0].v.z() + beta * triangle.vertexs[1].v.z() + gamma * triangle.vertexs[2].v.z();
                        if z < zbuf[(width * j + i) as usize] {
                            continue;
                        }
        
                        zbuf[(width * j + i) as usize] = z;

                        let uv = interpolate_vector2f(&triangle.vertexs[0].tex_coords,
                            &triangle.vertexs[1].tex_coords,
                            &triangle.vertexs[2].tex_coords, alpha, beta, gamma);

                        let mv = interpolate_vector4f(&triangle.vertexs[0].tv,
                            &triangle.vertexs[1].tv,
                            &triangle.vertexs[2].tv, alpha, beta, gamma);

                        let n = interpolate_vector3f(&triangle.vertexs[0].normal,
                            &triangle.vertexs[1].normal,
                            &triangle.vertexs[2].normal, alpha, beta, gamma);

                        let (mut r, mut g, mut b) = textures[0].get_color(uv.u(), 1.0-uv.v());

                        if rasterizer.get_lights().len() > 0 {
                            let result = compute_light(
                                &Vector3f::new_3(mv.x(), mv.y(), mv.z()), 
                                &n, 
                                rasterizer.get_lights(),
                                &Vector3f::new_3(0.005, 0.005, 0.005),
                                &Vector3f::new_3(r as f32  / 255.0, g as f32 / 255.0, b as f32 / 255.0),
                                &Vector3f::new_3(0.7937, 0.7937, 0.7937),
                                rasterizer.get_eye_pos());

                            r = (result.r() * 255.0) as u8;
                            g = (result.g() * 255.0) as u8;
                            b = (result.b() * 255.0) as u8;
                        }

                        image[((width * j + i) * 4) as usize] = b;
                        image[((width * j + i) * 4 + 1) as usize] = g;
                        image[((width * j + i) * 4 + 2) as usize] = r;
                        image[((width * j + i) * 4 + 3) as usize] = 255;

                    }
                }
            }
        }
    }
}

fn retriangle(triangle: &Triangle) -> (Vector4f, Vector4f, Vector4f) {
    let p0 = triangle.vertexs[0].v.clone();
    let p1 = triangle.vertexs[1].v.clone();
    let p2 = triangle.vertexs[2].v.clone();
    let (min, mid, max) = {
        if p0.y() > p1.y() {
            if p0.y() < p2.y() {
                (p1, p0, p2)
            } else if p1.y() > p2.y() {
                (p2, p1, p0)
            } else {
                (p1, p2, p0)
            }
        } else {
            if p1.y() < p2.y() {
                (p0, p1, p2)
            } else if p0.y() > p2.y() {
                (p2, p0, p1)
            } else {
                (p0, p2, p1)
            }
        }
    };

    if mid.x() > max.x() {
        (min, mid, max)
    } else {
        (min, max, mid)
    }
}

fn inside_triangle(x: f32, y: f32, a: &Vector4f, b: &Vector4f, c: &Vector4f) -> bool {
    let v1 = (b.x() - a.x()) * (y - a.y()) - (b.y() - a.y()) * (x - a.x());
    let v2 = (c.x() - b.x()) * (y - b.y()) - (c.y() - b.y()) * (x - b.x());
    let v3 = (a.x() - c.x()) * (y - c.y()) - (a.y() - c.y()) * (x - c.x());
    return v1 > 0.0 && v2 > 0.0 && v3 > 0.0;
}

fn compute_barycentric_2d(x: f32, y: f32, triangle: &Triangle) -> (f32, f32, f32) {
    let a = &triangle.vertexs[0].v;
    let b = &triangle.vertexs[1].v;
    let c = &triangle.vertexs[2].v;
    let alpha = ((x - b.x()) * (b.y() - c.y()) + (y - b.y()) * (c.x() - b.x()))
        / ((a.x() - b.x()) * (b.y() - c.y()) + (a.y() - b.y()) * (c.x() - b.x()));
    let beta = ((x - c.x()) * (c.y() - a.y()) + (y - c.y()) * (a.x() - c.x()))
        / ((b.x() - c.x()) * (c.y() - a.y()) + (b.y() - c.y()) * (a.x() - c.x()));
    let gamma = 1.0 - alpha - beta;
    (alpha, beta, gamma)
}
