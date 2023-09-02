use std::sync::Arc;
use typed_builder::TypedBuilder;
use windows::Win32::Graphics::DirectWrite::{DWRITE_FONT_STRETCH, DWRITE_FONT_STRETCH_CONDENSED, DWRITE_FONT_STRETCH_EXPANDED, DWRITE_FONT_STRETCH_EXTRA_CONDENSED, DWRITE_FONT_STRETCH_EXTRA_EXPANDED, DWRITE_FONT_STRETCH_MEDIUM, DWRITE_FONT_STRETCH_NORMAL, DWRITE_FONT_STRETCH_SEMI_CONDENSED, DWRITE_FONT_STRETCH_SEMI_EXPANDED, DWRITE_FONT_STRETCH_ULTRA_CONDENSED, DWRITE_FONT_STRETCH_ULTRA_EXPANDED, DWRITE_FONT_STYLE, DWRITE_FONT_STYLE_ITALIC, DWRITE_FONT_STYLE_NORMAL, DWRITE_FONT_STYLE_OBLIQUE, DWRITE_FONT_WEIGHT, DWRITE_FONT_WEIGHT_BLACK, DWRITE_FONT_WEIGHT_BOLD, DWRITE_FONT_WEIGHT_DEMI_BOLD, DWRITE_FONT_WEIGHT_EXTRA_BLACK, DWRITE_FONT_WEIGHT_EXTRA_BOLD, DWRITE_FONT_WEIGHT_EXTRA_LIGHT, DWRITE_FONT_WEIGHT_HEAVY, DWRITE_FONT_WEIGHT_LIGHT, DWRITE_FONT_WEIGHT_MEDIUM, DWRITE_FONT_WEIGHT_NORMAL, DWRITE_FONT_WEIGHT_REGULAR, DWRITE_FONT_WEIGHT_SEMI_BOLD, DWRITE_FONT_WEIGHT_SEMI_LIGHT, DWRITE_FONT_WEIGHT_THIN, DWRITE_FONT_WEIGHT_ULTRA_BLACK, DWRITE_FONT_WEIGHT_ULTRA_BOLD, DWRITE_FONT_WEIGHT_ULTRA_LIGHT};
use crate::{Color, Direct2DPoint};

#[derive(Debug, Clone, PartialEq, TypedBuilder)]
pub struct TextProperty {
    #[builder(setter(into))]
    pub text: Arc<str>,
    #[builder(setter(into))]
    pub font_size: f32,
    #[builder(default, setter(into))]
    pub font_weight: FontWeight,
    #[builder(default, setter(into))]
    pub position: Direct2DPoint,
    #[builder(default, setter(into))]
    pub font_family: Option<Arc<str>>,
    #[builder(default, setter(into))]
    pub color: Color,
    #[builder(default, setter(into))]
    pub font_style: FontStyle,
    #[builder(default, setter(into))]
    pub font_stretch: FontStretch,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FontWeight {
    Thin,
    ExtraLight,
    UltraLight,
    Light,
    SemiLight,
    Normal,
    Regular,
    Medium,
    DemiBold,
    SemiBold,
    Bold,
    ExtraBold,
    UltraBold,
    Black,
    Heavy,
    ExtraBlack,
    UltraBlack,
    Custom(i32),
}

impl Default for FontWeight {
    fn default() -> Self {
        FontWeight::Normal
    }
}

impl From<i32> for FontWeight {
    fn from(value: i32) -> Self {
        FontWeight::Custom(value)
    }
}

impl Into<DWRITE_FONT_WEIGHT> for FontWeight {
    fn into(self) -> DWRITE_FONT_WEIGHT {
        match self {
            FontWeight::Thin => DWRITE_FONT_WEIGHT_THIN,
            FontWeight::ExtraLight => DWRITE_FONT_WEIGHT_EXTRA_LIGHT,
            FontWeight::UltraLight => DWRITE_FONT_WEIGHT_ULTRA_LIGHT,
            FontWeight::Light => DWRITE_FONT_WEIGHT_LIGHT,
            FontWeight::SemiLight => DWRITE_FONT_WEIGHT_SEMI_LIGHT,
            FontWeight::Normal => DWRITE_FONT_WEIGHT_NORMAL,
            FontWeight::Regular => DWRITE_FONT_WEIGHT_REGULAR,
            FontWeight::Medium => DWRITE_FONT_WEIGHT_MEDIUM,
            FontWeight::DemiBold => DWRITE_FONT_WEIGHT_DEMI_BOLD,
            FontWeight::SemiBold => DWRITE_FONT_WEIGHT_SEMI_BOLD,
            FontWeight::Bold => DWRITE_FONT_WEIGHT_BOLD,
            FontWeight::ExtraBold => DWRITE_FONT_WEIGHT_EXTRA_BOLD,
            FontWeight::UltraBold => DWRITE_FONT_WEIGHT_ULTRA_BOLD,
            FontWeight::Black => DWRITE_FONT_WEIGHT_BLACK,
            FontWeight::Heavy => DWRITE_FONT_WEIGHT_HEAVY,
            FontWeight::ExtraBlack => DWRITE_FONT_WEIGHT_EXTRA_BLACK,
            FontWeight::UltraBlack => DWRITE_FONT_WEIGHT_ULTRA_BLACK,
            FontWeight::Custom(weight) => DWRITE_FONT_WEIGHT(weight),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FontStyle {
    Normal,
    Oblique,
    Italic,
}

impl Default for FontStyle {
    fn default() -> Self {
        FontStyle::Normal
    }
}

impl Into<DWRITE_FONT_STYLE> for FontStyle {
    fn into(self) -> DWRITE_FONT_STYLE {
        match self {
            FontStyle::Normal => DWRITE_FONT_STYLE_NORMAL,
            FontStyle::Oblique => DWRITE_FONT_STYLE_OBLIQUE,
            FontStyle::Italic => DWRITE_FONT_STYLE_ITALIC,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FontStretch {
    // Undefined,
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    Normal,
    Medium,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
    Custom(i32),
}

impl Default for FontStretch {
    fn default() -> Self {
        FontStretch::Normal
    }
}

impl From<i32> for FontStretch {
    fn from(value: i32) -> Self {
        FontStretch::Custom(value)
    }
}

impl Into<DWRITE_FONT_STRETCH> for FontStretch {
    fn into(self) -> DWRITE_FONT_STRETCH {
        match self {
            FontStretch::UltraCondensed => DWRITE_FONT_STRETCH_ULTRA_CONDENSED,
            FontStretch::ExtraCondensed => DWRITE_FONT_STRETCH_EXTRA_CONDENSED,
            FontStretch::Condensed => DWRITE_FONT_STRETCH_CONDENSED,
            FontStretch::SemiCondensed => DWRITE_FONT_STRETCH_SEMI_CONDENSED,
            FontStretch::Normal => DWRITE_FONT_STRETCH_NORMAL,
            FontStretch::Medium => DWRITE_FONT_STRETCH_MEDIUM,
            FontStretch::SemiExpanded => DWRITE_FONT_STRETCH_SEMI_EXPANDED,
            FontStretch::Expanded => DWRITE_FONT_STRETCH_EXPANDED,
            FontStretch::ExtraExpanded => DWRITE_FONT_STRETCH_EXTRA_EXPANDED,
            FontStretch::UltraExpanded => DWRITE_FONT_STRETCH_ULTRA_EXPANDED,
            FontStretch::Custom(stretch) => DWRITE_FONT_STRETCH(stretch),
        }
    }
}