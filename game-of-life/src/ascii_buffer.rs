use std::{borrow::BorrowMut, cell::RefCell};

pub const BLACK: &'static str = "\x1B[40m  \x1B[0m";
pub const WHITE: &'static str = "\x1B[47m  \x1B[0m";

#[derive(Debug, Clone)]
struct FrameBuffer {
    height: u16,
    width: u16,
    pixels: RefCell<Vec<bool>>,
}

impl FrameBuffer {
    fn new(height: u16, width: u16) -> Self {
        let size = height * width;
        Self {
            height,
            width,
            pixels: vec![false; size.into()].into(),
        }
    }

    fn clear(self) -> Self {
        self.pixels.borrow_mut().fill(false);
        self
    }
}
