use typed_builder::TypedBuilder;
use crate::{Color, Point, SolidColor};

#[derive(Debug, Clone, TypedBuilder)]
pub struct CircleProperty {
    #[builder(default, setter(into))]
    pub(crate) center: Point,
    #[builder(default, setter(into))]
    pub(crate) radius: f32,
    #[builder(default, setter(into))]
    pub(crate) fill_color: Option<Color>,
    #[builder(default, setter(into))]
    pub(crate) stroke_color: Option<SolidColor>,
    #[builder(default, setter(into))]
    pub(crate) stroke_width: f32,
}

impl Default for CircleProperty {
    fn default() -> Self {
        Self {
            center: Point::default(),
            radius: 1.0,
            fill_color: None,
            stroke_color: None,
            stroke_width: 1.0,
        }
    }
}
