use super::fluid::*;
use itertools::Itertools;
use std::mem;
use rand::{Isaac64Rng, Rng};
use noise::{Brownian2, perlin2};

#[derive(Default, Debug)]
pub struct Hex {
    pub solution: Solution,
}

impl Hex {
    pub fn color(&self) -> [f32; 4] {
        [0.25 * self.solution.fluids[1] as f32, 0.0, 0.25 * self.solution.fluids[0] as f32, 1.0]
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
            tiles: randomizing_vec(width, height, rng),
        }
    }

    pub fn randomize(&mut self, rng: &mut Isaac64Rng) {
        self.tiles = randomizing_vec(self.width, self.height, rng);
    }

    pub fn hex(&self, x: usize, y: usize) -> &Hex {
        &self.tiles[x + y * self.width]
    }

    pub fn hex_mut(&mut self, x: usize, y: usize) -> &mut Hex {
        &mut self.tiles[x + y * self.width]
    }

    pub fn cycle(&mut self) {
        // Then update diffusion.
        for x in 0..self.width {
            for y in 0..self.height {
                let mut this: &mut Hex = unsafe { mem::transmute(self.hex_mut(x, y)) };

                // Left
                this.solution.diffuse_from(&self.hex((x + self.width - 1) % self.width, y)
                    .solution);
                // Right
                this.solution.diffuse_from(&self.hex((x + self.width + 1) % self.width, y)
                    .solution);

                if y % 2 == 0 {
                    // UpRight
                    this.solution
                        .diffuse_from(&self.hex((x + self.width + 1) % self.width,
                                 (y + self.height - 1) % self.height)
                            .solution);
                    // UpLeft
                    this.solution.diffuse_from(&self.hex(x, (y + self.height - 1) % self.height)
                        .solution);
                    // DownLeft
                    this.solution.diffuse_from(&self.hex(x, (y + self.height + 1) % self.height)
                        .solution);
                    // DownRight
                    this.solution.diffuse_from(&self.hex((x + self.width + 1) % self.width,
                             (y + self.height + 1) % self.height)
                        .solution);
                } else {
                    // UpRight
                    this.solution
                        .diffuse_from(&self.hex(x, (y + self.height - 1) % self.height).solution);
                    // UpLeft
                    this.solution.diffuse_from(&self.hex((x + self.width - 1) % self.width,
                             (y + self.height - 1) % self.height)
                        .solution);
                    // DownLeft
                    this.solution.diffuse_from(&self.hex((x + self.width - 1) % self.width,
                             (y + self.height + 1) % self.height)
                        .solution);
                    // DownRight
                    this.solution.diffuse_from(&self.hex(x, (y + self.height + 1) % self.height)
                        .solution);
                }
            }
        }

        // Finish the cycle.
        for hex in &mut self.tiles {
            hex.solution.end_cycle();
        }
    }
}

fn randomizing_vec(width: usize, height: usize, rng: &mut Isaac64Rng) -> Vec<Hex> {
    let seeds = [rng.gen(), rng.gen()];
    let noise = Brownian2::new(perlin2, 4).wavelength(32.0);
    (0..height)
        .cartesian_product((0..width))
        .map(|(x, y)| {
            Hex {
                solution: Solution::new([1.0, noise.apply(&seeds[0], &[x as f64, y as f64])],
                                        [0.5, 0.25],
                                        [0.062, 0.061]),
            }
        })
        .collect_vec()
}
