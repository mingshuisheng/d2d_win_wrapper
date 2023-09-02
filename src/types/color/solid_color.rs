use std::ops::Deref;
use crate::Direct2DColor;

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct SolidColor(pub(crate) Direct2DColor);

impl SolidColor {
    pub fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self(
            Direct2DColor {
                r,
                g,
                b,
                a,
            }
        )
    }

    pub fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self(
            Direct2DColor {
                r,
                g,
                b,
                a: 1.0,
            }
        )
    }
}

impl PartialEq  for SolidColor {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl From<SolidColor> for Direct2DColor {
    fn from(color: SolidColor) -> Self {
        color.0
    }
}

impl Deref for SolidColor {
    type Target = Direct2DColor;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}