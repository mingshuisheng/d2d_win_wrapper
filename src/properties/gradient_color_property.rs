use crate::Direct2DPoint;

#[derive(Debug, Clone, PartialEq)]
pub struct LinearGradientProperty {
    pub start: Direct2DPoint,
    pub end: Direct2DPoint,
}

impl LinearGradientProperty {
    pub fn new(start: Direct2DPoint, end: Direct2DPoint) -> Self {
        Self {
            start,
            end,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RadialGradientProperty {
    pub center: Direct2DPoint,
    pub offset: Direct2DPoint,
    pub radius_x: f32,
    pub radius_y: f32,
}

impl RadialGradientProperty {
    pub fn new(center: Direct2DPoint, offset: Direct2DPoint, radius_x: f32, radius_y: f32) -> Self {
        Self {
            center,
            offset,
            radius_x,
            radius_y,
        }
    }


    pub fn new_circle(center: Direct2DPoint, offset: Direct2DPoint, radius: f32) -> Self {
        Self {
            center,
            offset,
            radius_x: radius,
            radius_y: radius,
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum GradientColorProperty {
    LinearGradient(LinearGradientProperty),
    RadialGradient(RadialGradientProperty),
    None,
}

impl From<LinearGradientProperty> for GradientColorProperty {
    fn from(property: LinearGradientProperty) -> Self {
        Self::LinearGradient(property)
    }
}

impl From<RadialGradientProperty> for GradientColorProperty {
    fn from(property: RadialGradientProperty) -> Self {
        Self::RadialGradient(property)
    }
}