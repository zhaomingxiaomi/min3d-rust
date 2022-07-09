mod math;
mod rasterizer;
use iced::{
    slider, Alignment, Column, Container, Element, Length, Sandbox, Settings,
    Slider, Text, Image, image::Handle,
};

use math::matrix::Matrix;
use math::vector::Vector;
use rasterizer::rasterizer::Rasterizer;

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
		for _ in 0..256 {
			for _ in 0..256 {
				image.push(123u8);
				image.push(123u8);
				image.push(123u8);
				image.push(123u8);
			}
		}
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
