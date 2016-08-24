use super::cell::*;
use super::fluid::*;
use itertools::Itertools;
use std::mem;
use rand::{Isaac64Rng, Rng};

pub const MAX_FLUID_INITIAL: u64 = 4294967296;

#[derive(Default, Debug)]
struct Hex {
    pub solution: Solution,
}

impl Hex {
    pub fn color(&self) -> [f32; 4] {
        [self.solution.fluids[0] as f32 / MAX_FLUID_INITIAL as f32,
         self.solution.fluids[1] as f32 / MAX_FLUID_INITIAL as f32,
         self.solution.fluids[2] as f32 / MAX_FLUID_INITIAL as f32,
         0.6]
    }
}

pub struct Grid {
    width: usize,
    height: usize,
    reaction: [[f64; TOTAL_FLUIDS]; TOTAL_FLUIDS],
    tiles: Vec<Hex>,
}

impl Grid {
    pub fn new(width: usize, height: usize, rng: &mut Isaac64Rng) -> Self {
        Grid {
            width: width,
            height: height,
            reaction: {
                let mut reaction = [[0f64; TOTAL_FLUIDS]; TOTAL_FLUIDS];
                let mut column_sums = [0f64; TOTAL_FLUIDS];
                // Add random values and sum the columns.
                for row in 0..TOTAL_FLUIDS {
                    for col in 0..TOTAL_FLUIDS {
                        let v = rng.next_f64();
                        reaction[row][col] = v;
                        column_sums[col] += v;
                    }
                }
                // Normalize each column.
                for row in 0..TOTAL_FLUIDS {
                    for col in 0..TOTAL_FLUIDS {
                        reaction[row][col] /= column_sums[col];
                    }
                }
                reaction
            },
            tiles: (0..(width * height))
                .map(|_| {
                    Hex {
                        solution: Solution::new({
                            let mut def: [u64; TOTAL_FLUIDS] = Default::default();
                            for f in &mut def {
                                *f = rng.gen_range(0, MAX_FLUID_INITIAL);
                            }
                            def
                        }),
                    }
                })
                .collect_vec(),
        }
    }

    pub fn get_hex(&self, x: usize, y: usize) -> &Hex {
        &self.tiles[x + y * self.width]
    }

    pub fn size(&self) -> usize {
        self.width * self.height
    }

    pub fn cycle(&mut self) {
        // Reactions happen first.
        for hex in &mut self.tiles {
            hex.solution.react(&self.reaction);
        }

        // Then update diffusion.
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

        // Reactions happen first.
        for hex in &mut self.tiles {
            hex.solution.diffuse_cycle();
        }
    }
}
