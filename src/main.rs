mod math;
mod rasterizer;
use iced::{
    slider, Alignment, Column, Container, Element, Length, Sandbox, Settings,
    Slider, Text, Image, image::Handle,
};

use math::vector::{Vector4f, Color3f};
use rasterizer::rasterizer::{Rasterizer, get_model_matrix, get_presp_projection_matrix, get_view_matrix, draw_trangle, get_ortho_projection_matrix};
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
        let mut triangle1 = Triangle::new();
        triangle1.set_colors(vec![
            Color3f::new_3(1.0, 0.0, 0.0), 
            Color3f::new_3(1.0, 0.0, 0.0),
            Color3f::new_3(1.0, 0.0, 0.0),
        ]);

        triangle1.set_vertexs(vec![
            Vector4f::new_4(2.0, -2.0, -1.0, 1.0),
            Vector4f::new_4(0.0, 2.0, -20.0, 1.0),
            Vector4f::new_4(-3.0, -2.0, -5.0, 1.0)
        ]);

        let mut triangle2 = Triangle::new();
        triangle2.set_colors(vec![
            Color3f::new_3(0.0, 1.0, 0.0), 
            Color3f::new_3(0.0, 1.0, 0.0),
            Color3f::new_3(0.0, 1.0, 0.0),
        ]);

        triangle2.set_vertexs(vec![
            Vector4f::new_4(1.0, -2.0, -5.0, 1.0),
            Vector4f::new_4(0.0, 4.0, -10.0, 1.0),
            Vector4f::new_4(-3.0, -2.0, -1.0, 1.0)
        ]);

        let mut rasterizer = Rasterizer::new();
        rasterizer.set_model(get_model_matrix((self.radius as f32 - 50.0) * 60.0 / 50.0));
        rasterizer.set_view(get_view_matrix(
            Vector4f::new_4(0.0, 0.0, 3.0, 1.0),
            Vector4f::new_4(0.0, 0.0, 0.0, 1.0),
            Vector4f::new_4(0.0, 1.0, 0.0, 1.0)
        ));

        rasterizer.set_projection(get_presp_projection_matrix(60.0, 1.0, -0.1, -50.0));
        rasterizer.compute_mvp();

        let mut zbuf: Vec<f32> = vec![-51.0; 256*256];

        draw_trangle(&rasterizer, &mut image, &mut zbuf,-0.1, -50.0, 256, 256, triangle1);
        draw_trangle(&rasterizer, &mut image, &mut zbuf , -0.1, -50.0, 256, 256, triangle2);

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
