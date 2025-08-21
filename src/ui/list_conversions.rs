use iced::widget::{Space, column, container, row, scrollable, text};
use iced::{Alignment, Background, Border, Element, Length, Theme};

use crate::shared::constant::DARK_GRAY;
use crate::{state::message::Message, store::store::Conversion};

pub fn list_conversions(conversions: &[Conversion]) -> Element<'static, Message> {
    if conversions.is_empty() {
        return empty_state().into();
    }

    let cards_per_row = 3;

    let rows = conversions.chunks(cards_per_row).enumerate().fold(
        column!().spacing(16),
        |col, (row_idx, chunk)| {
            let row = chunk
                .iter()
                .enumerate()
                .fold(row!().spacing(16), |r, (i, conv)| {
                    let _index = row_idx * cards_per_row + i;
                    r.push(conversion_card(conv))
                });

            col.push(row)
        },
    );

    scrollable(container(rows).padding(16).width(Length::Fill)).into()
}

fn conversion_card(conv: &Conversion) -> Element<'static, Message> {
    let title = text(format!("{} → {}", conv.from_unit, conv.to_unit)).size(18);

    let subtitle = text(format!("Category: {}", conv.category)).size(14);

    let main_value = text(format!("{:.4} {}", conv.value, conv.to_unit)).size(16);

    container(
        column![
            title,
            subtitle,
            Space::with_height(Length::Fixed(6.0)),
            main_value,
            Space::with_height(Length::Fixed(12.0)),
        ]
        .spacing(6)
        .align_x(Alignment::Start),
    )
    .padding(16)
    .width(Length::FillPortion(1))
    .style(card_style)
    .into()
}

fn card_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(DARK_GRAY)),
        border: Border {
            radius: 12.0.into(),
            width: 1.0,
            color: DARK_GRAY,
        },
        text_color: None,
        ..Default::default()
    }
}

fn empty_state() -> Element<'static, Message> {
    container(
        column![text("No conversions yet").size(20),]
            .spacing(8)
            .align_x(Alignment::Center),
    )
    .padding(40)
    .width(Length::Fill)
    .style(|_theme| container::Style {
        background: Some(Background::Color(DARK_GRAY)),
        border: Border {
            radius: 12.0.into(),
            width: 1.0,
            color: DARK_GRAY,
        },
        ..Default::default()
    })
    .into()
}
