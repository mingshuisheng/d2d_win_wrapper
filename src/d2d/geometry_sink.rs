use std::ops::Deref;
use windows::core::{Result};
use windows::Win32::Graphics::Direct2D::Common::{D2D1_FIGURE_BEGIN_FILLED, D2D1_FIGURE_END_CLOSED, D2D_SIZE_F};
use windows::Win32::Graphics::Direct2D::{D2D1_ARC_SEGMENT, D2D1_ARC_SIZE_SMALL, D2D1_SWEEP_DIRECTION_CLOCKWISE, ID2D1GeometrySink};
use crate::Direct2DPoint;

pub struct GeometrySink {
    sink: ID2D1GeometrySink,
    closed: bool,
}

impl GeometrySink {
    pub fn new(sink: ID2D1GeometrySink) -> Self {
        Self {
            sink,
            closed: false,
        }
    }

    pub fn begin(&self, start: Direct2DPoint) {
        unsafe {
            self.sink.BeginFigure(start, D2D1_FIGURE_BEGIN_FILLED);
        }
    }

    pub fn add_circle_arc(&self, end_point: Direct2DPoint, radius: f32, angle: f32) {
        unsafe {
            self.sink.AddArc(
                &D2D1_ARC_SEGMENT {
                    point: end_point,
                    size: D2D_SIZE_F {
                        width: radius,
                        height: radius,
                    },
                    rotationAngle: angle,
                    sweepDirection: D2D1_SWEEP_DIRECTION_CLOCKWISE,
                    arcSize: D2D1_ARC_SIZE_SMALL,
                });
        }
    }

    pub fn add_line(&self, point: Direct2DPoint) {
        unsafe {
            self.sink.AddLine(point);
        }
    }

    pub fn close(&self) -> Result<()> {
        if self.closed {
            return Ok(());
        }
        unsafe {
            self.sink.EndFigure(D2D1_FIGURE_END_CLOSED);
            self.sink.Close()
        }
    }
}

impl Drop for GeometrySink {
    fn drop(&mut self) {
        self.close().unwrap();
    }
}

impl Deref for GeometrySink {
    type Target = ID2D1GeometrySink;

    fn deref(&self) -> &Self::Target {
        &self.sink
    }
}