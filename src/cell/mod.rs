mod brain;

use rand::Isaac64Rng;

use mli::SISO;

use itertools::Itertools;

use super::fluid::{NORMAL_DIFFUSION, TOTAL_FLUIDS};

enum_from_primitive! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Direction {
        UpRight,
        UpLeft,
        Left,
        DownLeft,
        DownRight,
        Right,
    }
}

impl Direction {
    pub fn delta(&self, even_y: bool) -> (isize, isize) {
        use self::Direction::*;
        match *self {
            UpRight => {
                if even_y {
                    (1, -1)
                } else {
                    (0, -1)
                }
            }
            UpLeft => {
                if even_y {
                    (0, -1)
                } else {
                    (-1, -1)
                }
            }
            Left => (-1, 0),
            DownLeft => {
                if even_y {
                    (0, 1)
                } else {
                    (-1, 1)
                }
            }
            DownRight => {
                if even_y {
                    (1, 1)
                } else {
                    (0, 1)
                }
            }
            Right => (1, 0),
        }
    }

    pub fn flip(&self) -> Direction {
        use self::Direction::*;
        match *self {
            UpRight => DownLeft,
            UpLeft => DownRight,
            Left => Right,
            DownLeft => UpRight,
            DownRight => UpLeft,
            Right => Left,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Choice {
    // If the mate and spawn direction are the same, cause a divide.
    Divide {
        mate: Direction,
        spawn: Direction,
    },
    Move(Direction),
}

#[derive(Clone, Debug)]
pub struct Decision {
    pub choice: Choice,
    pub coefficients: [f64; 2],
}

#[derive(Clone)]
pub struct Cell {
    pub brain: brain::Brain,
}

impl Cell {
    pub fn new(rng: &mut Isaac64Rng) -> Self {
        Cell { brain: brain::Brain::new(rng) }
    }

    pub fn color(&self) -> [f32; 4] {
        [1.0, 1.0, 1.0, 1.0]
    }

    pub fn decide(&mut self, fluids: [&[f64; 2]; 7], cells: &[bool; 6]) -> Decision {
        use std::f64::{MAX, MIN};
        let nc = |n: bool| if n {
            1.0
        } else {
            0.0
        };
        let inputs = [0.0,
                      1.0,
                      -1.0,
                      2.0,
                      -2.0,
                      MAX,
                      MIN,
                      fluids[0][0],
                      fluids[0][1],
                      fluids[1][0],
                      fluids[1][1],
                      fluids[2][0],
                      fluids[2][1],
                      fluids[3][0],
                      fluids[3][1],
                      fluids[4][0],
                      fluids[4][1],
                      fluids[5][0],
                      fluids[5][1],
                      fluids[6][0],
                      fluids[6][1],
                      nc(cells[0]),
                      nc(cells[1]),
                      nc(cells[2]),
                      nc(cells[3]),
                      nc(cells[4]),
                      nc(cells[5]),
                      self.brain.memory[0],
                      self.brain.memory[1],
                      self.brain.memory[2],
                      self.brain.memory[3]];

        let mut compute = self.brain.mep.compute(&inputs[..]);
        let coefficients = [compute.next().unwrap(), compute.next().unwrap()];
        let move_attempt = compute.next().unwrap();
        let move_directions = [compute.next().unwrap(),
                               compute.next().unwrap(),
                               compute.next().unwrap(),
                               compute.next().unwrap(),
                               compute.next().unwrap(),
                               compute.next().unwrap()];
        let mate_attempt = compute.next().unwrap();
        let mate_directions = [compute.next().unwrap(),
                               compute.next().unwrap(),
                               compute.next().unwrap(),
                               compute.next().unwrap(),
                               compute.next().unwrap(),
                               compute.next().unwrap()];
        let spawn_directions = [compute.next().unwrap(),
                                compute.next().unwrap(),
                                compute.next().unwrap(),
                                compute.next().unwrap(),
                                compute.next().unwrap(),
                                compute.next().unwrap()];

        self.brain.memory.iter_mut().set_from(compute);
        Decision {
            choice: {
                if move_attempt >= 1.0 {
                    if mate_attempt >= 1.0 {
                        if mate_attempt > move_attempt {
                            Choice::Divide {
                                mate: mate_directions[1..]
                                    .iter()
                                    .cloned()
                                    .zip(&[Direction::UpRight,
                                           Direction::UpLeft,
                                           Direction::Left,
                                           Direction::DownLeft,
                                           Direction::DownRight,
                                           Direction::Right])
                                    .fold((mate_directions[0], Direction::UpRight),
                                          |(bestval, bestdir), (val, &dir)| if val > bestval {
                                              (val, dir)
                                          } else {
                                              (bestval, bestdir)
                                          })
                                    .1,
                                spawn: spawn_directions[1..]
                                    .iter()
                                    .cloned()
                                    .zip(&[Direction::UpRight,
                                           Direction::UpLeft,
                                           Direction::Left,
                                           Direction::DownLeft,
                                           Direction::DownRight,
                                           Direction::Right])
                                    .fold((spawn_directions[0], Direction::UpRight),
                                          |(bestval, bestdir), (val, &dir)| if val > bestval {
                                              (val, dir)
                                          } else {
                                              (bestval, bestdir)
                                          })
                                    .1,
                            }
                        } else {
                            Choice::Move(move_directions[1..]
                                .iter()
                                .cloned()
                                .zip(&[Direction::UpRight,
                                       Direction::UpLeft,
                                       Direction::Left,
                                       Direction::DownLeft,
                                       Direction::DownRight,
                                       Direction::Right])
                                .fold((move_directions[0], Direction::UpRight),
                                      |(bestval, bestdir), (val, &dir)| if val > bestval {
                                          (val, dir)
                                      } else {
                                          (bestval, bestdir)
                                      })
                                .1)
                        }
                    } else {
                        Choice::Move(move_directions[1..]
                            .iter()
                            .cloned()
                            .zip(&[Direction::UpRight,
                                   Direction::UpLeft,
                                   Direction::Left,
                                   Direction::DownLeft,
                                   Direction::DownRight,
                                   Direction::Right])
                            .fold((move_directions[0], Direction::UpRight),
                                  |(bestval, bestdir), (val, &dir)| if val > bestval {
                                      (val, dir)
                                  } else {
                                      (bestval, bestdir)
                                  })
                            .1)
                    }
                } else {
                    Choice::Divide {
                        mate: mate_directions[1..]
                            .iter()
                            .cloned()
                            .zip(&[Direction::UpRight,
                                   Direction::UpLeft,
                                   Direction::Left,
                                   Direction::DownLeft,
                                   Direction::DownRight,
                                   Direction::Right])
                            .fold((mate_directions[0], Direction::UpRight),
                                  |(bestval, bestdir), (val, &dir)| if val > bestval {
                                      (val, dir)
                                  } else {
                                      (bestval, bestdir)
                                  })
                            .1,
                        spawn: spawn_directions[1..]
                            .iter()
                            .cloned()
                            .zip(&[Direction::UpRight,
                                   Direction::UpLeft,
                                   Direction::Left,
                                   Direction::DownLeft,
                                   Direction::DownRight,
                                   Direction::Right])
                            .fold((spawn_directions[0], Direction::UpRight),
                                  |(bestval, bestdir), (val, &dir)| if val > bestval {
                                      (val, dir)
                                  } else {
                                      (bestval, bestdir)
                                  })
                            .1,
                    }
                }
            },
            coefficients: {
                let mut ncoef = [0.0; TOTAL_FLUIDS];
                for i in 0..TOTAL_FLUIDS {
                    let f = coefficients[i];
                    ncoef[i] = if f.is_normal() {
                        NORMAL_DIFFUSION[i] * (sig(f) * 0.5 + 1.0)
                    } else {
                        NORMAL_DIFFUSION[i]
                    };
                }
                ncoef
            },
        }
    }

    pub fn mate(&self, other: &Cell, rng: &mut Isaac64Rng) -> Cell {
        Cell { brain: self.brain.mate(&other.brain, rng) }
    }
}

/// In the range [0.0, 1.0).
fn sig(v: f64) -> f64 {
    1.0 / (1.0 + (-v).exp())
}
