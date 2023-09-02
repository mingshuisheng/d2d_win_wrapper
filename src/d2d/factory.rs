use windows::Win32::Graphics::Direct2D::{ID2D1Factory1, ID2D1PathGeometry};
use crate::d2d::GeometrySink;
use windows::core::{Result};
use windows::Win32::Graphics::Direct2D::Common::D2D1_FILL_MODE_WINDING;

pub struct Factory {
    factory: ID2D1Factory1,
}

impl Factory {
    pub fn new(factory: ID2D1Factory1) -> Self {
        Self {
            factory,
        }
    }

    pub fn create_path_geometry<F>(&self, fun: F) -> Result<ID2D1PathGeometry> where F: FnOnce(&GeometrySink) -> Result<()> {
        unsafe {
            let shape = self.factory.CreatePathGeometry()?;
            let sink = shape.Open()?;
            let sink = GeometrySink::new(sink);
            sink.SetFillMode(D2D1_FILL_MODE_WINDING);
            fun(&sink)?;
            Ok(shape)
        }
    }
}

impl std::ops::Deref for Factory {
    type Target = ID2D1Factory1;

    fn deref(&self) -> &Self::Target {
        &self.factory
    }
}