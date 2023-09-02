use std::ops::Deref;
use crate::SolidColor;

pub type Direct2DGradientStop = windows::Win32::Graphics::Direct2D::D2D1_GRADIENT_STOP;

#[macro_export]
macro_rules! gradient_color {
    ($(($color:expr, $position: expr),)*) => {
        {
            let color = $crate::GradientColor {
                stops: vec![
                    $(
                        $crate::GradientStop::new(
                            $color,
                            $position,
                        ),
                    )*
                ],
            };
            color.check();
            color
        }
    }
}

#[repr(transparent)]
#[derive(Clone, Debug, PartialEq)]
pub struct GradientStop(Direct2DGradientStop);

impl GradientStop {
    pub fn new(color: SolidColor, position: f32) -> Self {
        Self(
            Direct2DGradientStop {
                color: color.0,
                position,
            }
        )
    }
}

impl Deref for GradientStop {
    type Target = Direct2DGradientStop;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub struct GradientColor {
    pub stops: Vec<GradientStop>,
}

impl GradientColor {

    pub fn new(stops: Vec<GradientStop>) -> Self {
        Self { stops }
    }

    pub fn check(&self) {
        self.check_sum();
        self.check_positions();
    }

    pub fn check_sum(&self) {
        let sum = self.stops.iter().fold(0.0, |acc, stop| acc + stop.position);
        if sum != 1.0 {
            panic!(
                "Gradient color stops must add up to 1.0, got {}",
                sum
            );
        }
    }

    pub fn check_positions(&self) {
        let mut last_position = 0.0;
        for stop in self.stops.iter() {
            if stop.position <= last_position {
                panic!("Gradient color stops must be in ascending order");
            }
            last_position = stop.position;
        }
    }

    pub fn as_slice(&self) -> &[Direct2DGradientStop] {
        let slice: &[GradientStop] = self.stops.as_slice();
        let ptr = slice.as_ptr() as *const Direct2DGradientStop;
        unsafe { std::slice::from_raw_parts(ptr, slice.len()) }
    }
}

impl PartialEq for GradientColor {
    fn eq(&self, other: &Self) -> bool {
        if self.stops.len() != other.stops.len() {
            return false;
        }
        for (a, b) in self.stops.iter().zip(other.stops.iter()) {
            if a != b {
                return false;
            }
        }
        true
    }
}