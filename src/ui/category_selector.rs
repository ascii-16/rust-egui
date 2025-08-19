use iced::widget::{column, pick_list, text};
use iced::{Alignment, Element, Length};

use crate::shared::category::Category;
use crate::state::message::Message;

pub fn category_selector(category: Category) -> Element<'static, Message> {
    column![
        text("Select category").size(18),
        pick_list(&Category::ALL[..], Some(category), Message::CategoryChanged)
            .width(Length::Fixed(120.0))
            .style(|theme, status| {
                let mut style = pick_list::default(theme, status);
                style.border.radius = 6.0.into();
                style
            })
    ]
    .spacing(15)
    .align_x(Alignment::Center)
    .into()
}
