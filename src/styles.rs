use iced::{widget::container::Style, Border, Theme};

pub fn bordered_box(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

    Style {
        background: Some(palette.background.weak.color.into()),
        border: Border {
            width: 1.0,
            radius: 0.0.into(),
            color: palette.background.strong.color,
        },
        ..Style::default()
    }
}

pub fn selected_bordered_box(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

    Style {
        background: Some(palette.background.weak.color.into()),
        border: Border {
            width: 3.0,
            radius: 0.0.into(),
            color: palette.success.strong.color,
        },
        ..Style::default()
    }
}
