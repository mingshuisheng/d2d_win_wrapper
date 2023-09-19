mod context;
mod properties;
mod d2d;
mod context_holder;
#[macro_use]
pub mod types;

use windows::Win32::Graphics::DirectWrite::{DWRITE_LINE_METRICS, DWRITE_TEXT_METRICS, IDWriteTextLayout};
pub use types::*;

pub(crate) use d2d::*;
pub use context_holder::*;

pub use context::*;

pub use properties::*;

pub(crate) const BASE_DPI: f32 = 96.0;

pub struct TextLayoutInfo {
    pub layout: IDWriteTextLayout,
    pub metrics: DWRITE_TEXT_METRICS,
    pub raw_line_metrics: Vec<DWRITE_LINE_METRICS>,
}