use slint::{Color, Image};

/// What an app looks like as a tile on the home screen.
#[derive(Clone, Copy)]
pub struct CardStyle {
    /// Gradient angle in degrees, CSS-style (0 = towards the top, 90 = towards the right).
    pub angle: f32,
    pub start: Color,
    pub end: Color,
    /// Where the gradient reaches `end`, from 0.0 to 1.0.
    pub end_position: f32,
    /// Color of the icon and title.
    pub foreground: Color,
}

impl CardStyle {
    pub fn solid(background: Color, foreground: Color) -> Self {
        Self {
            angle: 0.0,
            start: background,
            end: background,
            end_position: 1.0,
            foreground,
        }
    }

    pub fn gradient(angle: f32, start: Color, end: Color, foreground: Color) -> Self {
        Self {
            angle,
            start,
            end,
            end_position: 1.0,
            foreground,
        }
    }

    /// Reach `end` at `position` (0.0-1.0) instead of at the far edge.
    pub fn ending_at(mut self, position: f32) -> Self {
        self.end_position = position;
        self
    }
}

pub trait App {
    fn icon(&self) -> Image;

    fn title(&self) -> String;

    fn card_style(&self) -> CardStyle;
}

/// A color from `0xRRGGBB`.
pub fn rgb(hex: u32) -> Color {
    Color::from_rgb_u8((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

/// Loads an embedded SVG, e.g. `svg_icon(include_bytes!("../assets/icon.svg"))`.
pub fn svg_icon(data: &[u8]) -> Image {
    Image::load_from_svg_data(data).expect("invalid embedded SVG")
}
