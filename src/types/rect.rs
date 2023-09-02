use std::ops::Deref;
use crate::{Direct2DPoint, Point};

pub type Direct2DRect = windows::Win32::Graphics::Direct2D::Common::D2D_RECT_F;

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Rect(Direct2DRect);

impl Rect {
    pub fn new(place: impl Into<Point>, width: f32, height: f32) -> Self {
        let Point(Direct2DPoint { x, y }) = place.into();
        Rect(Direct2DRect {
            left: x,
            top: y,
            right: x + width,
            bottom: y + height,
        })
    }
}

impl Default for Rect {
    fn default() -> Self {
        Self(
            Direct2DRect {
                left: 0.0,
                top: 0.0,
                right: 0.0,
                bottom: 0.0,
            }
        )
    }
}

impl From<Rect> for windows::Win32::Graphics::Direct2D::Common::D2D_RECT_F {
    fn from(rect: Rect) -> Self {
        rect.0
    }
}

impl Deref for Rect {
    type Target = Direct2DRect;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct RectRound {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
}

impl RectRound {
    pub fn new(top_left: f32, top_right: f32, bottom_right: f32, bottom_left: f32) -> Self {
        RectRound {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        }
    }
}

impl Default for RectRound {
    fn default() -> Self {
        RectRound {
            top_left: 0.0,
            top_right: 0.0,
            bottom_right: 0.0,
            bottom_left: 0.0,
        }
    }
}