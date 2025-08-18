mod app;

use app::MyApp;

pub fn main() -> iced::Result {
    iced::run("My Iced App", MyApp::update, MyApp::view)
}