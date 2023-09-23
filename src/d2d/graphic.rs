use std::iter::once;
use windows::core::{ComInterface, Error, PCWSTR, Result};
use windows::core::w;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Direct2D::{D2D1_DRAW_TEXT_OPTIONS_NO_SNAP, D2D1_ELLIPSE, D2D1_LAYER_PARAMETERS, ID2D1PathGeometry};
use windows::Win32::Graphics::DirectComposition::{IDCompositionDevice, IDCompositionTarget, IDCompositionVisual};
use windows::Win32::Graphics::DirectWrite::{DWRITE_LINE_METRICS, DWRITE_LINE_SPACING_METHOD_UNIFORM, DWRITE_PARAGRAPH_ALIGNMENT_CENTER, DWRITE_TEXT_METRICS, DWRITE_TEXT_RANGE, IDWriteFactory2, IDWriteTextFormat1, IDWriteTextLayout};
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM;
use windows::Win32::Graphics::Dxgi::IDXGISwapChain1;
use crate::{CircleProperty, Color, EllipseProperty, GradientColorProperty, LinearGradientProperty, LineProperty, RadialGradientProperty, RectProperty, TextLayoutInfo, TextProperty};
use crate::d2d::{create_write_factory, Factory};
use crate::{create_point, Direct2DPoint, Direct2DRect};
use super::RenderTarget;
use super::{create_composition_device, create_composition_target, create_device, create_dxgi_factory, create_factory, create_render_context, create_swap_chain, create_swap_chain_bitmap, create_visual, get_dpi, get_window_size};

pub struct Graphic {
    handle: HWND,
    factory: Factory,
    write_factory: IDWriteFactory2,
    render_target: RenderTarget,
    swap_chain: IDXGISwapChain1,
    comp_device: IDCompositionDevice,
    comp_target: IDCompositionTarget,
    visual: IDCompositionVisual,
}

impl Graphic {
    pub fn new(handle: isize) -> Result<Self> {
        let handle = HWND(handle);
        let size = get_window_size(handle)?;
        let factory = create_factory()?;
        let write_factory = create_write_factory()?;
        let dxgi_factory = create_dxgi_factory()?;
        let (dpi_x, dpi_y) = get_dpi(&factory)?;
        // let frequency = get_frequency()?;
        let device = create_device()?;
        let context = create_render_context(&factory, &device)?;
        unsafe {
            context.SetDpi(dpi_x, dpi_y);
        }
        let swap_chain = create_swap_chain(&dxgi_factory, &device, size)?;

        create_swap_chain_bitmap(&swap_chain, &context)?;

        let comp_device = create_composition_device(&device)?;
        let comp_target = create_composition_target(&comp_device, handle)?;
        let visual = create_visual(&comp_device, &comp_target, &swap_chain)?;

        Ok(Self {
            handle,
            factory: Factory::new(factory),
            write_factory,
            render_target: RenderTarget::new(context),
            swap_chain,
            comp_device,
            comp_target,
            visual,
        })
    }

    pub fn resize(&mut self) -> Result<()> {
        let size = get_window_size(self.handle)?;
        unsafe {
            //调用ResizeBuffers之前必须先释放相关资源
            self.visual.SetContent(None)?;
            self.comp_target.SetRoot(None)?;
            // self.swap_chain.SetFullscreenState(false, None)?;
            self.render_target.set_target(None);
            self.swap_chain.ResizeBuffers(
                2,
                size.0,
                size.1,
                DXGI_FORMAT_B8G8R8A8_UNORM,
                0,
            )?;
            //重新创建相关资源
            create_swap_chain_bitmap(&self.swap_chain, &self.render_target)?;
            self.visual = create_visual(&self.comp_device, &self.comp_target, &self.swap_chain)?;
        }

        Ok(())
    }
}


// draw shape

impl Graphic {
    unsafe fn get_baseline(text_layout: &IDWriteTextLayout) -> f32 {
        let mut text_metrics = DWRITE_TEXT_METRICS::default();
        let mut line_count = text_metrics.lineCount;
        let mut raw_line_metrics = vec![DWRITE_LINE_METRICS::default(); line_count as usize];
        let result = text_layout.GetLineMetrics(Some(raw_line_metrics.as_mut_slice()), &mut line_count);

        if result.is_err() {
            let mut raw_line_metrics = vec![DWRITE_LINE_METRICS::default(); line_count as usize];
            let result = text_layout.GetLineMetrics(Some(raw_line_metrics.as_mut_slice()), &mut line_count);
            if result.is_err() {
                return 0.0;
            }
        }

        raw_line_metrics.get(0).map(|lm| lm.baseline).unwrap_or(0.0)
    }

    pub(crate) unsafe fn create_text_layout(&self, text_property: TextProperty) -> Result<IDWriteTextLayout> {
        let text = text_property.text.as_ref().encode_utf16().chain(once(0)).collect::<Vec<u16>>();
        let fallback = self.write_factory.GetSystemFontFallback()?;
        let font_family = match &text_property.font_family {
            Some(font_family) => PCWSTR::from_raw(font_family.as_ref().encode_utf16().chain(once(0)).collect::<Vec<u16>>().as_ptr()),
            _ => w!("Microsoft YaHei"),
        };
        let text_format = self.write_factory.CreateTextFormat(
            font_family,
            None,
            text_property.font_weight.into(),
            text_property.font_style.into(),
            text_property.font_stretch.into(),
            text_property.font_size,
            w!(""),
        )?;
        let text_format = text_format.cast::<IDWriteTextFormat1>()?;
        text_format.SetFontFallback(Some(&fallback))?;
        // text_format.SetTextAlignment(DWRITE_TEXT_ALIGNMENT_CENTER)?;
        text_format.SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_CENTER)?;
        let text_layout = self.write_factory.CreateTextLayout(text.as_slice(), &text_format, f32::MAX, f32::MAX)?;
        let mut text_metrics = DWRITE_TEXT_METRICS::default();

        text_layout.GetMetrics(&mut text_metrics)?;

        let max_width = text_property.width.unwrap_or(text_metrics.width);
        text_layout.SetMaxWidth(max_width)?;

        //设置宽度后需要重新获取
        text_layout.GetMetrics(&mut text_metrics)?;

        text_layout.SetMaxWidth(max_width.max(text_metrics.width))?;

        let max_height = text_property.height.unwrap_or(text_metrics.height);
        text_layout.SetMaxHeight(max_height.max(text_metrics.height))?;

        if let Some(line_height) = text_property.line_height {
            let baseline = Self::get_baseline(&text_layout);
            text_layout.SetLineSpacing(DWRITE_LINE_SPACING_METHOD_UNIFORM, text_property.font_size * line_height, baseline)?;
        }

        if text_property.underline {
            text_layout.SetUnderline(true, DWRITE_TEXT_RANGE{startPosition: 0, length: text_property.text.as_ref().len() as u32}).unwrap();
        }

        Ok(text_layout)
    }

    pub(crate) fn draw_text(&self, text_property: TextProperty) -> Result<()> {
        unsafe {
            let context = &self.render_target;

            let text_color = text_property.color.clone();
            let text_position = text_property.position.clone();

            let text_layout = self.create_text_layout(text_property)?;

            let brush = context.create_brush(text_color, GradientColorProperty::None);
            context.DrawTextLayout(text_position, &text_layout, &brush, D2D1_DRAW_TEXT_OPTIONS_NO_SNAP);
        }
        Ok(())
    }

    pub(crate) fn draw_line(&self, line_property: LineProperty) -> Result<()> {
        unsafe {
            let context = &self.render_target;
            let start = *line_property.start.clone();
            let end = *line_property.end.clone();
            let width = line_property.width;
            let gradient_color_property = LinearGradientProperty::new(*line_property.start.clone(), *line_property.end.clone());
            let brush = context.create_brush(line_property.color.clone(), gradient_color_property.into());
            context.DrawLine(start, end, &brush, width, None);
        }
        Ok(())
    }

    pub(crate) fn draw_rect(&self, rect_property: RectProperty) -> Result<()> {
        unsafe {
            let context = &self.render_target;
            let rect = rect_property.rect.clone();
            let shape = self.create_react_shape(rect_property.clone())?;
            match rect_property.stroke_color {
                Some(color) => {
                    let width = rect_property.stroke_width;
                    let brush = context.create_brush(color.into(), GradientColorProperty::None);
                    // context.DrawRectangle(&rect, &brush, width, None);
                    context.DrawGeometry(&shape, &brush, width, None);
                }
                None => {}
            }
            match rect_property.fill_color {
                Some(color) => {
                    let start = create_point(rect.left, rect.top);
                    let end = create_point(rect.right, rect.bottom);
                    let gradient_color_property = Self::build_gradient_color_property(&color, start, end);
                    let brush = context.create_brush(color, gradient_color_property);
                    // context.FillRectangle(&rect, &brush);
                    context.FillGeometry(&shape, &brush, None);
                }
                None => {}
            }
        }
        Ok(())
    }

    fn create_react_shape(&self, rect_property: RectProperty) -> Result<ID2D1PathGeometry> {
        self.factory.create_path_geometry(|sink| {
            let rect = rect_property.rect.clone();

            //对圆角进行限制，避免大于短边的一半
            let width = rect.right - rect.left;
            let height = rect.bottom - rect.top;
            let max_radius = width.min(height) / 2.0;

            let left_top = rect_property.round.top_left.min(max_radius);

            // 从左上角开始
            let start = if left_top > 0.0 {
                create_point(rect.left, rect.top + left_top)
            } else {
                create_point(rect.left, rect.top)
            };
            sink.begin(start.clone());

            // 画左上角圆角
            if left_top > 0.0 {
                let end_point = create_point(rect.left + left_top, rect.top);
                sink.add_circle_arc(end_point, left_top, -90.0);
            }

            let right_top = rect_property.round.top_right.min(max_radius);

            // 画上边线
            let line_end = if right_top > 0.0 {
                create_point(rect.right - right_top, rect.top)
            } else {
                create_point(rect.right, rect.top)
            };
            sink.add_line(line_end);

            // 画右上角圆角
            if right_top > 0.0 {
                let end_point = create_point(rect.right, rect.top + right_top);
                sink.add_circle_arc(end_point, right_top, 90.0);
            }

            let right_bottom = rect_property.round.bottom_right.min(max_radius);

            // 画右边线
            let line_end = if right_bottom > 0.0 {
                create_point(rect.right, rect.bottom - right_bottom)
            } else {
                create_point(rect.right, rect.bottom)
            };
            sink.add_line(line_end);

            // 画右下角圆角
            if right_bottom > 0.0 {
                let end_point = create_point(rect.right - right_bottom, rect.bottom);
                sink.add_circle_arc(end_point, right_bottom, 90.0);
            }

            let left_bottom = rect_property.round.bottom_left.min(max_radius);

            // 画下边线
            let line_end = if left_bottom > 0.0 {
                create_point(rect.left + left_bottom, rect.bottom)
            } else {
                create_point(rect.left, rect.bottom)
            };
            sink.add_line(line_end);

            // 画左下角圆角
            if left_bottom > 0.0 {
                let end_point = create_point(rect.left, rect.bottom - left_bottom);
                sink.add_circle_arc(end_point, left_bottom, 90.0);
            }

            // 画左边线
            sink.add_line(start);
            Ok(())
        })
    }

    pub(crate) fn draw_circle(&self, circle_property: CircleProperty) -> Result<()> {
        self.draw_ellipse(circle_property.into())
    }

    pub(crate) fn draw_ellipse(&self, ellipse_property: EllipseProperty) -> Result<()> {
        unsafe {
            let context = &self.render_target;
            let center = *ellipse_property.center;
            let radius_x = ellipse_property.radius_x;
            let radius_y = ellipse_property.radius_y;
            match ellipse_property.stroke_color {
                Some(color) => {
                    let width = ellipse_property.stroke_width;
                    let brush = context.create_brush(color.into(), GradientColorProperty::None);
                    context.DrawEllipse(
                        &D2D1_ELLIPSE {
                            point: center,
                            radiusX: radius_x,
                            radiusY: radius_y,
                        },
                        &brush,
                        width,
                        None,
                    );
                }
                None => {}
            }
            match ellipse_property.fill_color {
                Some(color) => {
                    let start = create_point(center.x - radius_x, center.y - radius_y);
                    let end = create_point(center.x + radius_x, center.y + radius_y);
                    let gradient_color_property = Self::build_gradient_color_property(&color, start, end);
                    let brush = context.create_brush(color, gradient_color_property);
                    context.FillEllipse(
                        &D2D1_ELLIPSE {
                            point: center,
                            radiusX: radius_x,
                            radiusY: radius_y,
                        },
                        &brush,
                    );
                }
                None => {}
            }
        }
        Ok(())
    }

    /// 创建渐变色
    /// point1: 图形的左上角
    /// point2: 图形的右下角
    pub fn build_gradient_color_property(color_type: &Color, point1: Direct2DPoint, point2: Direct2DPoint) -> GradientColorProperty {
        match color_type {
            Color::LinearGradient(_, angle) => {
                //假设angle为0时，渐变色从下到上，那么start为下，end为上
                //根据point1和point2的位置，计算出圆心和半径
                let width = point2.x - point1.x;
                let height = point2.y - point1.y;
                let half_width = width / 2.0;
                let half_height = height / 2.0;
                let center = create_point(point1.x + half_width, point1.y + half_height);
                let radius = (height.powf(2.0) + width.powf(2.0)).sqrt() / 2.0;
                //计算出渐变色的起始点和结束点
                let (start, end) = calc_gradient_start_end(center, radius, *angle);

                LinearGradientProperty::new(start, end).into()
            }
            Color::RadialGradient(_) => {
                let width = point2.x - point1.x;
                let height = point2.y - point1.y;
                let half_width = width / 2.0;
                let half_height = height / 2.0;
                let center = create_point(point1.x + half_width, point1.y + half_height);
                let offset = create_point(0.0, 0.0);
                let radius = half_width.max(half_height);
                println!("point1:{:?},point2:{:?}", center, offset);
                RadialGradientProperty::new_circle(center, offset, radius).into()
            }
            Color::SolidColor(_) => {
                GradientColorProperty::None
            }
        }
    }
}


// state
impl Graphic {
    pub(crate) fn begin_draw(&self) -> Result<()> {
        self.render_target.begin_draw();
        Ok(())
    }

    pub(crate) fn end_draw(&self) -> Result<()> {
        self.render_target.end_draw();
        Ok(())
    }

    pub(crate) fn create_layer(&self, opacity: f32) {
        unsafe {
            let context = &self.render_target;
            let layer = context.CreateLayer(None).unwrap();
            let size = context.GetSize();
            let rect = Direct2DRect {
                left: 0.0,
                top: 0.0,
                right: size.width,
                bottom: size.height,
            };
            context.PushLayer(&D2D1_LAYER_PARAMETERS {
                contentBounds: rect,
                opacity,
                ..Default::default()
            }, &layer);
            context.Clear(None);
        }
    }

    pub(crate) fn exit_layer(&self) {
        self.render_target.pop_layer();
    }

    pub(crate) fn present(&self) -> Result<()> {
        unsafe {
            let hresult = self.swap_chain.Present(1, 0);
            if hresult.is_err() {
                return Err(Error::from_win32());
            }
        }
        Ok(())
    }
}

/// impl other trait


/// impl Drop
impl Drop for Graphic {
    fn drop(&mut self) {
        unsafe {
            self.visual.SetContent(None).unwrap();
            self.comp_target.SetRoot(None).unwrap();
            // self.swap_chain.SetFullscreenState(false, None).unwrap();
        }
    }
}

/// 根据圆心、半径、角度计算渐变色的起始点和结束点
fn calc_gradient_start_end(center: Direct2DPoint, radius: f32, angle: f32) -> (Direct2DPoint, Direct2DPoint) {
    let angle = angle % 360.0;
    let angle = if angle < 0.0 { angle + 360.0 } else { angle };

    //判断是锐角还是钝角
    match angle {
        //锐角
        angle if angle >= 0.0 && angle <= 90.0 => acute_calc(&center, radius, angle),
        //钝角
        angle if angle > 90.0 && angle < 180.0 => obtuse_calc(&center, radius, angle),
        //锐角对称
        angle if angle >= 180.0 && angle <= 270.0 => {
            let (start, end) = acute_calc(&center, radius, angle - 180.0);
            // 翻转
            (end, start)
        }
        //钝角对称
        angle if angle > 270.0 && angle < 360.0 => {
            let (start, end) = obtuse_calc(&center, radius, angle - 180.0);
            // 反转
            (end, start)
        }
        _ => {
            //不可能出现的情况
            //因为已经对angle进行了取余
            unreachable!("angle is not in 0..360")
        }
    }
}

//锐角计算
fn acute_calc(center: &Direct2DPoint, radius: f32, angle: f32) -> (Direct2DPoint, Direct2DPoint) {
    let angle = 90.0 - angle;
    let angle = angle.to_radians();
    let start = create_point(center.x - radius * angle.cos(), center.y + radius * angle.sin());
    let end = create_point(center.x + radius * angle.cos(), center.y - radius * angle.sin());
    (start, end)
}

//钝角计算
fn obtuse_calc(center: &Direct2DPoint, radius: f32, angle: f32) -> (Direct2DPoint, Direct2DPoint) {
    let angle = angle - 90.0;
    let angle = angle.to_radians();
    let start = create_point(center.x - radius * angle.cos(), center.y - radius * angle.sin());
    let end = create_point(center.x + radius * angle.cos(), center.y + radius * angle.sin());
    (start, end)
}
