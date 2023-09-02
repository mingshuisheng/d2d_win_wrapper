use typed_builder::TypedBuilder;
use crate::{CircleProperty, Color, SolidColor, Point};

#[derive(Debug, Clone, TypedBuilder)]
pub struct EllipseProperty {
    #[builder(default, setter(into))]
    pub(crate) center: Point,
    #[builder(default, setter(into))]
    pub(crate) radius_x: f32,
    #[builder(default, setter(into))]
    pub(crate) radius_y: f32,
    #[builder(default, setter(into))]
    pub(crate) fill_color: Option<Color>,
    #[builder(default, setter(into))]
    pub(crate) stroke_color: Option<SolidColor>,
    #[builder(default, setter(into))]
    pub(crate) stroke_width: f32,
}

impl Default for EllipseProperty {
    fn default() -> Self {
        Self {
            center: Point::default(),
            radius_x: 1.0,
            radius_y: 1.0,
            fill_color: None,
            stroke_color: None,
            stroke_width: 1.0,
        }
    }
}

impl From<CircleProperty> for EllipseProperty {
    fn from(circle: CircleProperty) -> Self {
        Self {
            center: circle.center,
            radius_x: circle.radius,
            radius_y: circle.radius,
            fill_color: circle.fill_color,
            stroke_color: circle.stroke_color,
            stroke_width: circle.stroke_width,
        }
    }
}
