use super::cell::*;
use super::fluid::*;
use itertools::Itertools;
use std::mem;
use rand::{Isaac64Rng, Rng};
use noise::{Brownian2, perlin2};

const SPAWN_RATE: f64 = 0.01;

#[derive(Debug)]
struct Delta {
    movement_attempts: Vec<(usize, usize)>,
}

#[derive(Debug)]
pub struct Hex {
    pub solution: Solution,
    pub cell: Option<Cell>,
    pub decision: Option<Decision>,
    delta: Delta,
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

    fn hex_and_neighbors(&mut self, x: usize, y: usize) -> (&mut Hex, [&Hex; 6]) {
        (unsafe { mem::transmute(self.hex_mut(x, y)) },
         if y % 2 == 0 {
            [// UpRight
             self.hex((x + self.width + 1) % self.width,
                      (y + self.height - 1) % self.height),
             // UpLeft
             self.hex(x, (y + self.height - 1) % self.height),
             // Left
             self.hex((x + self.width - 1) % self.width, y),
             // DownLeft
             self.hex(x, (y + self.height + 1) % self.height),
             // DownRight
             self.hex((x + self.width + 1) % self.width,
                      (y + self.height + 1) % self.height),
             // Right
             self.hex((x + self.width + 1) % self.width, y)]
        } else {
            [// UpRight
             self.hex(x, (y + self.height - 1) % self.height),
             // UpLeft
             self.hex((x + self.width - 1) % self.width,
                      (y + self.height - 1) % self.height),
             // Left
             self.hex((x + self.width - 1) % self.width, y),
             // DownLeft
             self.hex((x + self.width - 1) % self.width,
                      (y + self.height + 1) % self.height),
             // DownRight
             self.hex(x, (y + self.height + 1) % self.height),
             // Right
             self.hex((x + self.width + 1) % self.width, y)]
        })
    }

    pub fn cycle(&mut self, rng: &mut Isaac64Rng) {
        self.cycle_spawn(rng);
        self.cycle_cells(rng);
        self.cycle_decisions();
        self.cycle_fluids();
    }

    fn cycle_spawn(&mut self, rng: &mut Isaac64Rng) {
        if rng.next_f64() < SPAWN_RATE {
            self.tiles[rng.gen_range(0, self.width * self.height)].cell = Some(Cell::new());
        }
    }

    fn cycle_cells(&mut self, rng: &mut Isaac64Rng) {
        for x in 0..self.width {
            for y in 0..self.height {
                let (this, neighbors) = self.hex_and_neighbors(x, y);
                this.decision = if let Some(ref mut this_cell) = this.cell {
                    let neighbor_presents = [neighbors[0].cell.is_some(),
                                             neighbors[1].cell.is_some(),
                                             neighbors[2].cell.is_some(),
                                             neighbors[3].cell.is_some(),
                                             neighbors[4].cell.is_some(),
                                             neighbors[5].cell.is_some()];

                    Some(this_cell.decide(&this.solution.fluids, &neighbor_presents, rng))
                } else {
                    None
                }
            }
        }
    }

    fn cycle_decisions(&mut self) {
        // Compute the deltas resulting from the decision.
        for x in 0..self.width {
            for y in 0..self.height {
                let (width, height) = (self.width, self.height);
                let (this, neighbors) = self.hex_and_neighbors(x, y);
                // Clear the movements from the previous cycle.
                this.delta.movement_attempts.clear();
                this.solution.coefficients = if let Some(ref decision) = this.decision {
                    decision.coefficients
                } else {
                    // Set the diffusion coefficients to the normal values.
                    [0.5, 1.0]
                };

                // Only add movements here if no cell is present.
                if this.cell.is_none() {
                    // Add any neighbor movements to the movement_attempts vector.
                    for (n, &facing) in neighbors.iter().zip(&[Direction::DownLeft,
                                                               Direction::DownRight,
                                                               Direction::Right,
                                                               Direction::UpRight,
                                                               Direction::UpLeft,
                                                               Direction::Left]) {
                        if let Some(Decision { choice: Choice::Move(direction), .. }) = n.decision {
                            // It attempted to move into this hex cell.
                            if facing == direction {
                                this.delta
                                    .movement_attempts
                                    .push(in_direction(x, y, width, height, direction));

                                // No need to continue if we reach 2 attempts.
                                if this.delta.movement_attempts.len() == 2 {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        // Perform the deltas.
        for x in 0..self.width {
            for y in 0..self.height {
                // Handle movement.
                if self.hex(x, y).delta.movement_attempts.len() == 1 {
                    let from_coord = self.hex(x, y).delta.movement_attempts[0];
                    self.hex_mut(x, y).cell = self.hex_mut(from_coord.0, from_coord.1).cell.take();
                }

                // Clear the decisions.
                self.hex_mut(x, y).decision = None;
            }
        }
    }

    fn cycle_fluids(&mut self) {
        // Then update diffusion.
        for x in 0..self.width {
            for y in 0..self.height {
                let (this, neighbors) = self.hex_and_neighbors(x, y);

                for n in &neighbors {
                    this.solution.diffuse_from(&n.solution);
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
    let noise = Brownian2::new(perlin2, 4).wavelength(64.0);
    (0..height)
        .cartesian_product((0..width))
        .map(|(x, y)| {
            Hex {
                solution: Solution::new([noise.apply(&seeds[0], &[x as f64, y as f64]), 1.0],
                                        [0.5, 1.0]),
                cell: None,
                decision: None,
                delta: Delta { movement_attempts: Vec::with_capacity(6) },
            }
        })
        .collect_vec()
}

fn in_direction(x: usize,
                y: usize,
                width: usize,
                height: usize,
                direction: Direction)
                -> (usize, usize) {
    let diff = direction.delta(y % 2 == 0);
    (((width + x) as isize + diff.0) as usize % width,
     ((height + y) as isize + diff.1) as usize % height)
}
