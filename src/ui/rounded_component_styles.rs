use iced::Theme;
use iced::widget::{button, pick_list, text_input};

pub fn rounded_pick_list_style(theme: &Theme, status: pick_list::Status) -> pick_list::Style {
    let mut style = pick_list::default(theme, status);
    style.border.radius = 6.0.into();
    style
}

pub fn rounded_button_style(
    theme: &Theme,
    status: button::Status,
    style_fn: Option<fn(&Theme, button::Status) -> button::Style>,
) -> button::Style {
    // Defaults to button::primary if no style_fn is passed
    let mut style = match style_fn {
        Some(f) => f(theme, status),
        None => button::primary(theme, status),
    };

    style.border.radius = 6.0.into();
    style
}

pub fn rounded_input_style(theme: &Theme, status: text_input::Status) -> text_input::Style {
    let mut style = text_input::default(theme, status);
    style.border.radius = 6.0.into();
    style
}
