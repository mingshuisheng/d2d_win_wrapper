mod context;
mod properties;
mod d2d;
mod context_holder;
#[macro_use]
pub mod types;
pub use types::*;

pub(crate) use d2d::*;
pub use context_holder::*;

pub use context::*;

pub use properties::*;

pub(crate) const BASE_DPI: f32 = 96.0;