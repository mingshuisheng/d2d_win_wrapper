use windows::core::IntoParam;
use windows::core::Param;
use windows::Win32::Graphics::Direct2D::{ID2D1BitmapBrush, ID2D1Brush, ID2D1LinearGradientBrush, ID2D1RadialGradientBrush, ID2D1SolidColorBrush};

pub enum Brush {
    Bitmap(ID2D1BitmapBrush),
    SolidColor(ID2D1SolidColorBrush),
    LinearGradient(ID2D1LinearGradientBrush),
    RadialGradient(ID2D1RadialGradientBrush),
}

impl From<ID2D1BitmapBrush> for Brush {
    fn from(brush: ID2D1BitmapBrush) -> Self {
        Self::Bitmap(brush)
    }
}

impl From<ID2D1SolidColorBrush> for Brush {
    fn from(brush: ID2D1SolidColorBrush) -> Self {
        Self::SolidColor(brush)
    }
}

impl From<ID2D1LinearGradientBrush> for Brush {
    fn from(brush: ID2D1LinearGradientBrush) -> Self {
        Self::LinearGradient(brush)
    }
}

impl From<ID2D1RadialGradientBrush> for Brush {
    fn from(brush: ID2D1RadialGradientBrush) -> Self {
        Self::RadialGradient(brush)
    }
}

impl IntoParam<ID2D1Brush> for &Brush {
    fn into_param(self) -> Param<ID2D1Brush> {
        match self {
            Brush::Bitmap(brush) => brush.into_param(),
            Brush::SolidColor(brush) => brush.into_param(),
            Brush::LinearGradient(brush) => brush.into_param(),
            Brush::RadialGradient(brush) => brush.into_param(),
        }
    }
}