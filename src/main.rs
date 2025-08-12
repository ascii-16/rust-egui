mod app;

pub fn main() -> iced::Result {
    iced::run("My Iced App", app::MyApp::update, app::MyApp::view)
}