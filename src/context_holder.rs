use std::cell::RefCell;
use std::rc::Rc;
use crate::Context;
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
}