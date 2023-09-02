use typed_builder::TypedBuilder;
use crate::{Color, Rect, RectRound, SolidColor};

#[derive(Debug, Clone, TypedBuilder)]
pub struct RectProperty {
    #[builder(default, setter(into))]
    pub(crate) rect: Rect,
    #[builder(default, setter(into))]
    pub(crate) fill_color: Option<Color>,
    #[builder(default, setter(into))]
    pub(crate) stroke_color: Option<SolidColor>,
    #[builder(default, setter(into))]
    pub(crate) stroke_width: f32,
    #[builder(default, setter(into))]
    pub(crate) round: RectRound,
}

impl Default for RectProperty {
    fn default() -> Self {
        Self {
            rect: Rect::default(),
            fill_color: None,
            stroke_color: None,
            stroke_width: 1.0,
            round: RectRound::default(),
        }
    }
}