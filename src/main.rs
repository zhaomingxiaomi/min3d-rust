mod math;
mod common;
mod fixed_pipeline;

use std::fs::{File, self};
use std::io::{BufReader, BufRead};
use std::path::Path;
use rayon::current_num_threads;
use rayon::iter::ParallelIterator;
use rayon::iter::IntoParallelRefMutIterator;

use iced::futures::executor::{ThreadPool, ThreadPoolBuilder};
use iced::{
    slider, Alignment, Column, Container, Element, Length, Sandbox, Settings,
    Slider, Text, Image, image::Handle,
};

use math::vector::{Vector4f, Vector2f, Vector3f};
use fixed_pipeline::rasterizer::{Rasterizer, get_model_matrix, get_presp_projection_matrix, get_view_matrix, draw_trangle, get_ortho_projection_matrix, draw_trangle_map, RenderResult};
use common::triangle::Triangle;
use common::texture::Texture;
use common::light::Light;

pub fn main() -> iced::Result {
    SoftRender::run(Settings::default())
}

struct SoftRender {
    radius: f32,
    slider: slider::State,
    texture: Vec<Texture>,
    t: Vec<Triangle>
}

#[derive(Debug, Clone, Copy)]
enum Message {
    RadiusChanged(f32),
}

impl Sandbox for SoftRender {
    type Message = Message;

    fn new() -> SoftRender {
        let f = File::open(Path::new("./objdata")).unwrap();
        let reader = BufReader::new(f);
        let lines = reader.lines();
        let mut idx = 0;
        let texture = Texture::new(0, "./spot_texture.png");

        let mut e = SoftRender {
            radius: 50.0,
            slider: slider::State::new(),
            texture: Vec::new(),
            t: Vec::new()
        };

        e.texture.push(texture);
                
        let mut vetexs = Vec::new();
        let mut texcoords = Vec::new();
        let mut normals = Vec::new();
        lines.for_each(|line| {
            if let Ok(line) = line {
                if idx % 3 == 0 {
                    let all: Vec<&str> = line.split(",").collect();
                    //翻转模型z值
                    vetexs.push(Vector4f::new_4(all[0].parse::<f32>().unwrap(), all[1].parse::<f32>().unwrap(), -1.0 * all[2].parse::<f32>().unwrap(), 1.0));
                } else if idx % 3 == 1 {
                    let all: Vec<&str> = line.split(",").collect();
                    texcoords.push(Vector2f::new_2(all[0].parse::<f32>().unwrap(), all[1].parse::<f32>().unwrap()));
                } else {
                    let all: Vec<&str> = line.split(",").collect();
                    normals.push(Vector3f::new_3(all[0].parse::<f32>().unwrap(), all[1].parse::<f32>().unwrap(),all[2].parse::<f32>().unwrap()));
                }

                idx += 1;
            }
        });

        for _ in 0..vetexs.len() / 3 {
            let mut t = Triangle::new();
            let mut v = Vec::new();
            let mut p = Vec::new();
            let mut n = Vec::new();
            v.push(vetexs.remove(0));
            v.push(vetexs.remove(0));
            v.push(vetexs.remove(0));

            p.push(texcoords.remove(0));
            p.push(texcoords.remove(0));
            p.push(texcoords.remove(0));

            n.push(normals.remove(0));
            n.push(normals.remove(0));
            n.push(normals.remove(0));



            t.set_origin_vertexs(v);
            t.set_tex_coords(p);
            t.set_normal(n);
            t.set_render_type(common::triangle::RenderType::TEXTURE);
            e.t.push(t);
        }

        e
    }

    fn title(&self) -> String {
        String::from("mini3d-rs")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::RadiusChanged(radius) => {
                self.radius = radius;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
		let mut image = vec![0 as u8; 512*512*4];
        // let mut triangle1 = Triangle::new();
        // triangle1.set_colors(vec![
        //     Color3f::new_3(1.0, 0.0, 0.0), 
        //     Color3f::new_3(1.0, 0.0, 0.0),
        //     Color3f::new_3(1.0, 0.0, 0.0),
        // ]);

        // triangle1.set_vertexs(vec![
        //     Vector4f::new_4(2.0, -2.0, -0.1, 1.0),
        //     Vector4f::new_4(0.0, 2.0, -20.0, 1.0),
        //     Vector4f::new_4(-3.0, -2.0, -5.0, 1.0)
        // ]);

        // let mut triangle2 = Triangle::new();
        // triangle2.set_colors(vec![
        //     Color3f::new_3(0.0, 1.0, 0.0), 
        //     Color3f::new_3(0.0, 1.0, 0.0),
        //     Color3f::new_3(0.0, 1.0, 0.0),
        // ]);

        // triangle2.set_vertexs(vec![
        //     Vector4f::new_4(1.0, -2.0, -5.0, 1.0),
        //     Vector4f::new_4(0.0, 4.0, -10.0, 1.0),
        //     Vector4f::new_4(-3.0, -2.0, -1.0, 1.0)
        // ]);

        let mut rasterizer = Rasterizer::new();
        rasterizer.set_model(get_model_matrix((self.radius as f32 - 50.0) * 180.0 / 50.0));
        //rasterizer.set_model(get_model_matrix(0.0));

        rasterizer.set_view(get_view_matrix(
            Vector4f::new_4(0.0, 0.0, 2.0, 1.0),
            Vector4f::new_4(0.0, 0.0, 0.0, 1.0),
            Vector4f::new_4(0.0, 1.0, 0.0, 1.0)
        ));

        rasterizer.set_projection(get_presp_projection_matrix(60.0, 1.0, -0.1, -50.0));
        rasterizer.compute_mvp();
        rasterizer.set_eye_pos(Vector3f::new_3(0.0, 0.0, 2.0));
        rasterizer.set_lights(vec![
            Light::new(            
                Vector3f::new_3(20.0, 20.0, -20.0),
                Vector3f::new_3(500.0, 500.0, 500.0)
            ),
            Light::new(            
                Vector3f::new_3(20.0, -20.0, 0.0),
                Vector3f::new_3(500.0, 500.0, 500.0)
            ),
            ]
        );

        let mut zbuf: Vec<f32> = vec![-51.0; 512*512];
        let res: Vec<Vec<RenderResult>> = self.t.par_iter_mut()
        .map(|x| {
            draw_trangle_map(&rasterizer, 512, 512, x, &self.texture)
        })
        .collect();

        for tmp in res {
            for r in tmp {
                if r.z > zbuf[r.idx as usize] {
                    image[r.idx as usize * 4] = r.b;
                    image[r.idx as usize * 4 + 1] = r.g;
                    image[r.idx as usize * 4 + 2] = r.r;
                    image[r.idx as usize * 4 + 3] = 255;
                    zbuf[r.idx as usize] = r.z;
                }
            }
        }
        // for t in self.t.iter_mut() {
        //     draw_trangle(&rasterizer, &mut image, &mut zbuf,-0.1, -50.0, 512, 512, t, &self.texture);
        // }
        //draw_trangle(&rasterizer, &mut image, &mut zbuf,-0.1, -50.0, 256, 256, triangle1);
        //draw_trangle(&rasterizer, &mut image, &mut zbuf , -0.1, -50.0, 256, 256, triangle2);

        // let mut imgbuf = image::ImageBuffer::new(512, 512);

        // // Iterate over the coordinates and pixels of the image
        // for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        //     *pixel = image::Rgb([image[((512*y+x)*4+2) as usize], image[((512*y+x)*4+1) as usize], image[((512*y+x)*4) as usize]]);
        // }
        // imgbuf.save("b.png").unwrap();

        let handle = Handle::from_pixels(512, 512, image);
        let content = Column::new()
            .padding(20)
            .spacing(20)
            .max_width(500)
            .align_items(Alignment::Center)
            .push(Text::new(format!("Radius: {:.2}", self.radius)))
			.push(Image::new(handle).width(Length::Fill).height(Length::Fill))
            .push(
                Slider::new(
                    &mut self.slider,
                    1.0..=100.0,
                    self.radius,
                    Message::RadiusChanged,
                )
                .step(0.01),
            );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn background_color(&self) -> iced::Color {
        iced::Color::WHITE
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }

    fn should_exit(&self) -> bool {
        false
    }

    fn run(settings: Settings<()>) -> Result<(), iced::Error>
    where
        Self: 'static + Sized,
    {
        <Self as iced::Application>::run(settings)
    }
}
