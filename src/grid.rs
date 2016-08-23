use super::cell::*;
use super::fluid::*;
use itertools::Itertools;
use std::mem;

#[derive(Default, Debug)]
struct Hex {
    solution: Solution,
}

pub struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<Hex>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            width: width,
            height: height,
            tiles: (0..(width * height)).map(|_| Default::default()).collect_vec(),
        }
    }

    fn size(&self) -> usize {
        self.width * self.height
    }

    pub fn cycle(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let mut this: &'static mut Hex =
                    unsafe { mem::transmute(&mut self.tiles[x + y * self.width]) };
                // UpRight
                this.solution.diffuse_from(&self.tiles[x +
                                                       ((y + self.height - 1) % self.height) *
                                                       self.width]
                    .solution);
                // UpLeft
                this.solution.diffuse_from(&self.tiles[((x + self.width - 1) % self.width) +
                                                       ((y + self.height - 1) % self.height) *
                                                       self.width]
                    .solution);
                // Left
                this.solution.diffuse_from(&self.tiles[((x + self.width - 1) % self.width) +
                                                       y * self.width]
                    .solution);
                // DownLeft
                this.solution.diffuse_from(&self.tiles[((x + self.width - 1) % self.width) +
                                                       ((y + self.height + 1) % self.height) *
                                                       self.width]
                    .solution);
                // DownRight
                this.solution.diffuse_from(&self.tiles[x +
                                                       ((y + self.height + 1) % self.height) *
                                                       self.width]
                    .solution);
                // Right
                this.solution.diffuse_from(&self.tiles[((x + self.width + 1) % self.width) +
                                                       y * self.width]
                    .solution);
            }
        }
    }
}
