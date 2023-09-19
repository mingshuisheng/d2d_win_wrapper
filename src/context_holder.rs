use std::cell::RefCell;
use std::rc::Rc;
use windows::Win32::Graphics::DirectWrite::{DWRITE_LINE_METRICS, DWRITE_TEXT_METRICS};
use crate::{Context, TextLayoutInfo, TextProperty};
use crate::Graphic;

pub struct ContextHolder {
    graphic: Rc<RefCell<Graphic>>,
}

impl ContextHolder {
    pub fn new(hwnd: isize) -> Self {
        Self {
            graphic: Rc::new(RefCell::new(Graphic::new(hwnd).unwrap())),
        }
    }

    pub fn start_draw(&self) -> Context {
        let graphic = self.graphic.clone();
        graphic.borrow().begin_draw().unwrap();
        Context::new(graphic, true)
    }

    pub fn resize(&mut self) {
        self.graphic.borrow_mut().resize().unwrap();
    }

    pub fn create_text_layout(&self, text_property: TextProperty) -> TextLayoutInfo {
        unsafe {
            let layout = self.graphic.borrow().create_text_layout(text_property).unwrap();
            let mut metrics = DWRITE_TEXT_METRICS::default();
            layout.GetMetrics(&mut metrics).unwrap();
            let mut line_count = metrics.lineCount;
            let mut raw_line_metrics = vec![DWRITE_LINE_METRICS::default(); line_count as usize];
            layout.GetLineMetrics(Some(raw_line_metrics.as_mut_slice()), &mut line_count).unwrap();

            TextLayoutInfo {
                layout,
                metrics,
                raw_line_metrics
            }
        }
    }
}