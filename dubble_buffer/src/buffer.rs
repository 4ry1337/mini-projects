#[derive(Debug, Clone)]
pub struct FrameBuffer {
    pub height: usize,
    pub width: usize,
    pub pixels: Vec<char>,
}

impl FrameBuffer {
    pub fn new(height: usize, width: usize) -> Self {
        let size = height * width;
        Self {
            height,
            width,
            pixels: vec![' '; size].into(),
        }
    }

    pub fn display(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                print!("{}", self.pixels[row * self.height + col]);
            }
            println!();
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&char> {
        self.pixels.get(row * self.height + col)
    }

    pub fn set(&mut self, row: usize, col: usize, c: char) {
        self.pixels[row * self.height + col] = c;
    }

    pub fn fill(&mut self, c: char) {
        self.pixels.fill(c);
    }

    pub fn clear(&mut self) {
        self.fill(' ');
    }
}

impl Default for FrameBuffer {
    fn default() -> Self {
        Self {
            height: 10,
            width: 10,
            pixels: vec![' '; 100].into(),
        }
    }
}
