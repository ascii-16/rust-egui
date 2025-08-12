mod app;

use app::MyApp;
use rust_egui::units::length::LengthUnit;

pub fn main() -> iced::Result {
    // App expects generic type `U: Unit` so we default it to LengthUnit
    iced::run("My Iced App", MyApp::<LengthUnit>::update, MyApp::view)
}