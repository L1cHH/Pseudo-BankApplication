use iced::{Background, Border, Color, Shadow, Theme, Vector};
use iced::border::Radius;
use iced::widget::{container};
use iced::widget::container::Appearance;

pub struct ContainerStyle;

impl container::StyleSheet for ContainerStyle {
    type Style = Theme;
    fn appearance(&self, style: &Self::Style) -> Appearance {
        Appearance {
            text_color: Some(Color::BLACK),//Color::from_rgba8(0, 51, 90, 0.8)),
            background: Some(Background::Color(Color::from_rgba8(33, 120, 186, 0.8))),
            border: Border {
                color : Color::new(0.0, 0.0, 0.0, 1.0),
                width : 1.0,
                radius: Radius::from(6)
            },
            shadow: Shadow {
                color: Color::BLACK,
                offset: Vector::new(0.0, 2.0),
                blur_radius: 5.0
            }
        }
    }
}