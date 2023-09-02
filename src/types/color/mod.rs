pub type Direct2DColor = windows::Win32::Graphics::Direct2D::Common::D2D1_COLOR_F;

mod solid_color;

pub use solid_color::*;

mod gradient_color;

pub use gradient_color::*;

#[derive(Clone, Debug)]
pub enum Color {
    SolidColor(Direct2DColor),
    LinearGradient(GradientColor, f32),
    RadialGradient(GradientColor),
}

impl Default for Color {
    fn default() -> Self {
        Color::SolidColor(Direct2DColor {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        })
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Color::SolidColor(a), Color::SolidColor(b)) => a.eq(b),
            (Color::LinearGradient(a, angle1), Color::LinearGradient(b, angle2)) => a.eq(b) && *angle1 == *angle2,
            (Color::RadialGradient(a), Color::RadialGradient(b)) => a.eq(b),
            _ => false,
        }
    }
}

impl From<Direct2DColor> for Color {
    fn from(color: Direct2DColor) -> Self {
        Color::SolidColor(color)
    }
}

impl From<SolidColor> for Color {
    fn from(color: SolidColor) -> Self {
        Color::SolidColor(color.0)
    }
}


// from rgb
impl From<(u8, u8, u8)> for Color {
    fn from(color: (u8, u8, u8)) -> Self {
        let (r, g, b) = color;
        let color = Direct2DColor {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: 1.0,
        };
        Color::SolidColor(color)
    }
}

// from rgba
impl From<(u8, u8, u8, u8)> for Color {
    fn from(color: (u8, u8, u8, u8)) -> Self {
        let (r, g, b, a) = color;
        let color = Direct2DColor {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        };
        Color::SolidColor(color)
    }
}
