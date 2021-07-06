#![windows_subsystem = "windows"]
use iced::{
    button, slider, window::Icon, Align, Button, Column, Element, Length, ProgressBar, Row,
    Sandbox, Settings, Slider, Text,
};

pub fn main() -> iced::Result {
    let data = include_bytes!("icon.png");
    let img = image::load_from_memory_with_format(data, image::ImageFormat::Png)
        .unwrap()
        .to_rgba8();
    let icon = Icon::from_rgba(img.to_vec(), img.height(), img.width()).unwrap();
    // let settings = Settings::default();
    Counter::run(Settings {
        window: iced::window::Settings {
            size: (300, 300),
            min_size: Some((300, 300)),
            icon: Some(icon),
            transparent: (true),
            // decorations: false,
            ..Default::default()
        },
        exit_on_close_request: true,
        antialiasing: true,
        ..Default::default()
    })
}

#[derive(Default)]
struct Counter {
    value: i32,
    progress_bar_slider: slider::State,
    increment_button: button::State,
    decrement_button: button::State,
    reset_button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
    Reset,
    SliderChanged(f32),
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
            Message::Reset => {
                self.value = 0;
            }
            Message::SliderChanged(val) => {
                self.value = val as _;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(10)
            .spacing(10)
            .align_items(Align::Center)
            .push(
                Row::new()
                    .spacing(10)
                    .push(
                        Column::new()
                            .width(Length::FillPortion(2))
                            .align_items(Align::Start)
                            .push(
                                Button::new(&mut self.increment_button, Text::new("Increment"))
                                    .on_press(Message::IncrementPressed),
                            ),
                    )
                    .push(
                        Column::new()
                            .width(Length::FillPortion(1))
                            .align_items(Align::Center)
                            .push(
                                Text::new(self.value.to_string())
                                    .size(30)
                                    .vertical_alignment(iced::VerticalAlignment::Top),
                            ),
                    )
                    .push(
                        Column::new()
                            .width(Length::FillPortion(2))
                            .align_items(Align::End)
                            .push(
                                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                                    .on_press(Message::DecrementPressed),
                            ),
                    ),
            )
            .push(ProgressBar::new(0.0..=100.0, self.value as _))
            .push(
                Slider::new(
                    &mut self.progress_bar_slider,
                    0.0..=100.0,
                    self.value as _,
                    Message::SliderChanged,
                )
                .step(1.0),
            )
            .push(
                Column::new()
                    .width(Length::Fill)
                    .align_items(Align::End)
                    .push(
                        Button::new(&mut self.reset_button, Text::new("Reset"))
                            .on_press(Message::Reset),
                    ),
            )
            .into()
    }
}
