use std::ops::Deref;
use windows::core::IntoParam;
use windows::Win32::Graphics::Direct2D::{D2D1_EXTEND_MODE_CLAMP, D2D1_GAMMA_2_2, D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES, D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, ID2D1DeviceContext, ID2D1Image};
use crate::d2d::Brush;
use crate::{Color, GradientColorProperty};

pub struct RenderTarget {
    device_context: ID2D1DeviceContext,
}

impl RenderTarget {
    pub fn new(device_context: ID2D1DeviceContext) -> Self {
        Self {
            device_context,
        }
    }

    pub fn begin_draw(&self) {
        unsafe {
            self.device_context.BeginDraw();
        }
    }

    pub fn end_draw(&self) {
        unsafe {
            self.device_context.EndDraw(None, None).unwrap();
        }
    }

    pub fn pop_layer(&self) {
        unsafe {
            self.device_context.PopLayer();
        }
    }

    pub fn set_target<P0>(&self, image: P0)
        where
            P0: IntoParam<ID2D1Image> {
        unsafe {
            self.device_context.SetTarget(image);
        }
    }

    pub fn create_brush(&self, color: Color, gradient_color_property: GradientColorProperty) -> Brush {
        match (color, gradient_color_property) {
            (Color::SolidColor(color), _) => {
                let color = color;
                unsafe {
                    self.device_context.CreateSolidColorBrush(&color, None).unwrap().into()
                }
            }
            (Color::LinearGradient(color, _), GradientColorProperty::LinearGradient(liner_property)) => {
                unsafe {
                    let collection = self.CreateGradientStopCollection(color.as_slice(), D2D1_GAMMA_2_2, D2D1_EXTEND_MODE_CLAMP).unwrap();
                    self.device_context.CreateLinearGradientBrush(
                        &D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
                            startPoint: liner_property.start,
                            endPoint: liner_property.end,
                        },
                        None,
                        Some(&collection),
                    ).unwrap().into()
                }
            }
            (Color::RadialGradient(color), GradientColorProperty::RadialGradient(radial_property)) => {
                unsafe {
                    let collection = self.CreateGradientStopCollection(color.as_slice(), D2D1_GAMMA_2_2, D2D1_EXTEND_MODE_CLAMP).unwrap();
                    self.device_context.CreateRadialGradientBrush(
                        &D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
                            center: radial_property.center,
                            gradientOriginOffset: radial_property.offset,
                            radiusX: radial_property.radius_x,
                            radiusY: radial_property.radius_y,
                        },
                        None,
                        Some(&collection),
                    ).unwrap().into()
                }
            }
            _ => {
                panic!("not support");
            }
        }
    }
}

impl Deref for RenderTarget {
    type Target = ID2D1DeviceContext;

    fn deref(&self) -> &Self::Target {
        &self.device_context
    }
}