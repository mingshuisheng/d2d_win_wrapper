use std::ops::Deref;

pub type Direct2DPoint = windows::Win32::Graphics::Direct2D::Common::D2D_POINT_2F;

pub fn create_point(x: f32, y: f32) -> Direct2DPoint {
    Direct2DPoint {
        x,
        y,
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Point(pub(crate) Direct2DPoint);

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self(
            Direct2DPoint {
                x,
                y,
            }
        )
    }
}

impl Default for Point {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl From<Direct2DPoint> for Point {
    fn from(point: Direct2DPoint) -> Self {
        Self(point)
    }
}

impl From<Point> for Direct2DPoint {
    fn from(point: Point) -> Self {
        point.0
    }
}

impl From<(f32, f32)> for Point {
    fn from(point: (f32, f32)) -> Self {
        Self(
            Direct2DPoint {
                x: point.0,
                y: point.1,
            }
        )
    }
}

impl Deref for Point {
    type Target = Direct2DPoint;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}