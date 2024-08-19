use crate::ascii_buffer::scene::Scene;

pub struct Conway {
    seed: usize,
    life_count: usize,
    tick: usize,
    scene: Scene,
}

impl Conway {
    pub fn new(seed: usize, life_count: usize, height: usize, width: usize) -> Self {
        Self {
            seed,
            life_count,
            tick: 0,
            scene: Scene::new(height, width),
        }
    }

    pub fn seed(&mut self, seed: usize) {
        self.seed = seed;
    }

    pub fn neighbour_count(&self, row: usize, col: usize) -> usize {
        let mut count: usize = 0;
        for i in 0..=2 {
            for j in 0..=2 {
                if row + i > 0
                    && col + j > 0
                    && self.scene.read_buf.get(row + i - 1, col + j - 1) == Some(&'$')
                {
                    count += 1;
                }
            }
        }
        count
    }
}
