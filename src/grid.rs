use super::cell::*;
use super::fluid::*;
use itertools::Itertools;
use std::mem;
use rand::{Isaac64Rng, Rng};

#[derive(Default, Debug)]
pub struct Hex {
    pub solution: Solution,
}

impl Hex {
    pub fn color(&self) -> [f32; 4] {
        [self.solution.fluids[0] as f32, self.solution.fluids[1] as f32, 0.0, 1.0]
    }
}

pub struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<Hex>,
}

impl Grid {
    pub fn new(width: usize, height: usize, rng: &mut Isaac64Rng) -> Self {
        Grid {
            width: width,
            height: height,
            tiles: (0..(width * height))
                .map(|_| Hex { solution: Solution::new([rng.next_f64(), rng.next_f64()]) })
                .collect_vec(),
        }
    }

    pub fn randomize(&mut self, rng: &mut Isaac64Rng) {
        for hex in &mut self.tiles {
            hex.solution = Solution::new([rng.next_f64(), rng.next_f64()]);
        }
    }

    pub fn get_hex(&self, x: usize, y: usize) -> &Hex {
        &self.tiles[x + y * self.width]
    }

    pub fn size(&self) -> usize {
        self.width * self.height
    }

    pub fn cycle(&mut self) {
        // Then update diffusion.
        for x in 0..self.width {
            for y in 0..self.height {
                let mut this: &'static mut Hex =
                    unsafe { mem::transmute(&mut self.tiles[x + y * self.width]) };
                // Right
                this.solution.diffuse_from(&self.get_hex((x + self.width + 1) % self.width, y)
                    .solution);
                // Left
                this.solution.diffuse_from(&self.get_hex((x + self.width - 1) % self.width, y)
                    .solution);
                // UpRight
                this.solution
                    .diffuse_from(&self.get_hex(x, (y + self.height - 1) % self.height).solution);
                // DownLeft
                this.solution.diffuse_from(&self.get_hex((x + self.width - 1) % self.width,
                             (y + self.height + 1) % self.height)
                    .solution);
                // UpLeft
                this.solution.diffuse_from(&self.get_hex((x + self.width - 1) % self.width,
                             (y + self.height - 1) % self.height)
                    .solution);
                // DownRight
                this.solution.diffuse_from(&self.get_hex(x, (y + self.height + 1) % self.height)
                    .solution);
            }
        }

        // Finish cycle.
        for hex in &mut self.tiles {
            hex.solution.end_cycle();
        }
    }
}
