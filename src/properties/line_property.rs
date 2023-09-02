use typed_builder::TypedBuilder;
use crate::{Color, Point};

#[derive(Debug, Clone, TypedBuilder)]
pub struct LineProperty {
    #[builder(default, setter(into))]
    pub(crate) start: Point,
    #[builder(default, setter(into))]
    pub(crate) end: Point,
    #[builder(default, setter(into))]
    pub(crate) width: f32,
    #[builder(default, setter(into))]
    pub(crate) color: Color,
}

impl Default for LineProperty {
    fn default() -> Self {
        Self {
            start: Point::default(),
            end: Point::default(),
            width: 1.0,
            color: Color::default().into(),
        }
    }
}