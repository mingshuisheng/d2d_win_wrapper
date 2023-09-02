use std::cell::RefCell;
use std::rc::Rc;
use crate::{CircleProperty, EllipseProperty, Graphic, RectProperty, TextProperty};
use crate::properties::LineProperty;


pub struct Context {
    graphic: Rc<RefCell<Graphic>>,
    end_draw: bool,
}

impl Context {
    pub(crate) fn new(graphic: Rc<RefCell<Graphic>>, end_draw: bool) -> Self {
        graphic.borrow().create_layer(1.0);
        Self {
            graphic,
            end_draw,
        }
    }

    pub fn new_layer(&self, opacity: f32) -> Context {
        let graphic = self.graphic.clone();
        graphic.borrow().create_layer(opacity);
        Context::new(graphic, false)
    }

    pub fn draw_line(&self, line_property: LineProperty) {
        self.graphic.borrow().draw_line(line_property).unwrap();
    }

    pub fn draw_rect(&self, rect_property: RectProperty) {
        self.graphic.borrow().draw_rect(rect_property).unwrap();
    }

    pub fn draw_circle(&self, circle_property: CircleProperty) {
        self.graphic.borrow().draw_circle(circle_property).unwrap();
    }

    pub fn draw_ellipse(&self, ellipse_property: EllipseProperty) {
        self.graphic.borrow().draw_ellipse(ellipse_property).unwrap();
    }

    pub fn draw_text(&self, text_property: TextProperty) {
        self.graphic.borrow().draw_text(text_property).unwrap();
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        let graphic = &self.graphic.borrow();
        graphic.exit_layer();
        if self.end_draw {
            graphic.end_draw().unwrap();
            graphic.present().unwrap();
        }
    }
}