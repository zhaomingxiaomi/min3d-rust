mod math;
mod rasterizer;
use iced::{
    slider, Alignment, Column, Container, Element, Length, Sandbox, Settings,
    Slider, Text, Image, image::Handle,
};

use math::matrix::Matrix;
use math::vector::{Vector, Color};

use rasterizer::rasterizer::{Rasterizer, get_model_matrix, get_presp_projection_matrix, get_view_matrix, draw_trangle};
use rasterizer::triangle::Triangle;

pub fn main() -> iced::Result {
    Example::run(Settings::default())
}

struct Example {
    radius: f32,
    slider: slider::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    RadiusChanged(f32),
}

impl Sandbox for Example {
    type Message = Message;

    fn new() -> Self {
        Example {
            radius: 50.0,
            slider: slider::State::new(),
        }
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
		let mut image = Vec::new();
		for i in 0..256 {
			for j in 0..256 {
                //if i < 128 && j < 128 {
                    image.push(100u8);
                    image.push(100u8);
                    image.push(100u8);
                    image.push(255u8);
                //}
			}
		}
        let mut triangle = Triangle::new();
        triangle.set_colors(vec![
            Color {r: 1.0, g:0.0, b:0.0}, 
            Color {r: 0.0, g:1.0, b:0.0},
            Color {r: 0.0, g:0.0, b:1.0},
        ]);

        triangle.set_vertexs(vec![
            Vector { x: 2.0, y:-2.0, z: -2.0, w: 1.0 },
            Vector { x: 0.0, y: 2.0, z: -2.0, w: 1.0 },
            Vector { x: -3.0, y: -2.0, z: -2.0, w: 1.0 }
        ]);

        let mut rasterizer = Rasterizer::new();
        rasterizer.set_model(get_model_matrix((self.radius as f32 - 50.0) * 60.0 / 50.0));
        rasterizer.set_view(get_view_matrix(
            Vector::new(0.0, 0.0, 5.0, 1.0),
            Vector::new(0.0, 0.0, 0.0, 1.0),
            Vector::new(0.0, 1.0, 0.0, 1.0)
        ));

        rasterizer.set_projection(get_presp_projection_matrix(60.0, 1.0, -0.1, -50.0));
        rasterizer.compute_mvp();
        draw_trangle(&rasterizer, &mut image, 256, 256, triangle);

        let handle = Handle::from_pixels(256, 256, image);
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
}
