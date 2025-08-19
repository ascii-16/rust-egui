mod app;

use app::MyApp;

pub fn main() -> iced::Result {
    iced::run("Iced Converter", MyApp::update, MyApp::view)
}