use iced::widget::{button, column, text, text_input};
use iced::{Alignment, Element, Task};

pub fn main() -> iced::Result {
    iced::run("My Iced App", MyApp::update, MyApp::view)
}

#[derive(Default)]
struct MyApp {
    name: String,
    age: u32,
}

#[derive(Debug, Clone)]
enum Message {
    NameChanged(String),
    IncrementAge,
}

impl MyApp {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::NameChanged(name) => {
                self.name = name;
            }
            Message::IncrementAge => {
                self.age += 1;
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        column![
            text("My Iced Application").size(24),
            text_input("Your name", &self.name)
                .on_input(Message::NameChanged),
            text(format!("Age: {}", self.age)),
            button("Click each year")
                .on_press(Message::IncrementAge),
            text(format!("Hello '{}', age {}", self.name, self.age)),
        ]
        .spacing(10)
        .align_x(Alignment::Center)
        .into()
    }
}