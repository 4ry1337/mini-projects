pub const WHITE: &'static str = "\x1B[47m  \x1B[0m";
pub const BLACK: &'static str = "\x1B[40m  \x1B[0m";

#[derive(Debug, Clone)]
pub struct FrameBuffer {
    height: usize,
    width: usize,
    pixels: Vec<bool>,
}

impl FrameBuffer {
    pub fn new(height: usize, width: usize) -> Self {
        let size = height * width;
        Self {
            height,
            width,
            pixels: vec![false; size].into(),
        }
    }

    pub fn clear(mut self) -> Self {
        self.pixels.fill(false);
        self
    }

    pub fn draw(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                if self.pixels[row * self.height + col] {
                    print!("{}", WHITE);
                } else {
                    print!("{}", BLACK);
                }
            }
            println!();
        }
    }

    pub fn toggle(mut self, row: usize, col: usize) -> Self {
        self.pixels[row * self.height + col] = !self.pixels[row * self.height + col];
        self
    }
}
