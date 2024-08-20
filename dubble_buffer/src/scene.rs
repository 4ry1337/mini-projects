use std::{
    borrow::{Borrow, BorrowMut},
    fmt::{Debug, Formatter},
    ops::{Deref, DerefMut},
};

use super::buffer::FrameBuffer;

pub struct Scene {
    pub height: usize,
    pub width: usize,
    pub read_buf: FrameBuffer,
    pub write_buf: FrameBuffer,
}

impl Scene {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            height,
            width,
            read_buf: FrameBuffer::new(height, width),
            write_buf: FrameBuffer::new(height, width),
        }
    }

    pub fn display(&self) {
        self.read_buf.display();
    }

    pub fn draw<T>(&mut self, mut f: T)
    where
        T: FnMut(&mut FrameBuffer),
    {
        f(&mut self.write_buf);
        self.swap();
    }

    fn swap(&mut self) {
        std::mem::swap(&mut self.read_buf, &mut self.write_buf);
    }

    pub fn swap_cloning(&mut self) {
        self.read_buf = self.write_buf.clone();
    }
}

impl Debug for Scene {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scene")
            .field("read_buf", &self.read_buf)
            .field("write_buf", &self.write_buf)
            .finish()
    }
}

impl Deref for Scene {
    type Target = FrameBuffer;

    fn deref(&self) -> &Self::Target {
        &self.read_buf
    }
}

impl DerefMut for Scene {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.write_buf
    }
}

impl Borrow<FrameBuffer> for Scene {
    fn borrow(&self) -> &FrameBuffer {
        &self.read_buf
    }
}

impl BorrowMut<FrameBuffer> for Scene {
    fn borrow_mut(&mut self) -> &mut FrameBuffer {
        &mut self.write_buf
    }
}

impl AsRef<FrameBuffer> for Scene {
    fn as_ref(&self) -> &FrameBuffer {
        &self.read_buf
    }
}

impl AsMut<FrameBuffer> for Scene {
    fn as_mut(&mut self) -> &mut FrameBuffer {
        &mut self.write_buf
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            height: 10,
            width: 10,
            read_buf: FrameBuffer::new(10, 10),
            write_buf: FrameBuffer::new(10, 10),
        }
    }
}
