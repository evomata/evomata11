mod brain;

use rand::{Isaac64Rng, Rng};
use mli::SISO;
use itertools::Itertools;
use super::fluid::{NORMAL_DIFFUSION, TOTAL_FLUIDS};

const INITIAL_INHALE: usize = 50;
const COLOR_MUTATION_CHANCE: f32 = 0.01;

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
    Nothing,
}

#[derive(Clone, Debug)]
pub struct Decision {
    pub choice: Choice,
    pub coefficients: [f64; TOTAL_FLUIDS],
}

#[derive(Clone)]
pub struct Cell {
    pub inhale: usize,
    pub color: [f32; 3],
    pub brain: brain::Brain,
    turn: usize,
}

impl Cell {
    pub fn new(rng: &mut Isaac64Rng) -> Self {
        Cell {
            inhale: INITIAL_INHALE,
            color: [rng.next_f32(), rng.next_f32(), rng.next_f32()],
            brain: brain::Brain::new(rng),
            turn: rng.gen_range(0, 6),
        }
    }

    pub fn color(&self) -> [f32; 4] {
        [self.color[0], self.color[1], self.color[2], 1.0]
    }

    pub fn decide(&mut self, fluids: [&[f64; TOTAL_FLUIDS]; 7], cells: &[bool; 6]) -> Decision {
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
                      self.inhale as f64,
                      fluids[0][0],
                      fluids[0][1],
                      fluids[0][2],
                      fluids[0][3],
                      fluids[(0 + self.turn) % 6 + 1][0],
                      fluids[(0 + self.turn) % 6 + 1][1],
                      fluids[(0 + self.turn) % 6 + 1][2],
                      fluids[(0 + self.turn) % 6 + 1][3],
                      fluids[(1 + self.turn) % 6 + 1][0],
                      fluids[(1 + self.turn) % 6 + 1][1],
                      fluids[(1 + self.turn) % 6 + 1][2],
                      fluids[(1 + self.turn) % 6 + 1][3],
                      fluids[(2 + self.turn) % 6 + 1][0],
                      fluids[(2 + self.turn) % 6 + 1][1],
                      fluids[(2 + self.turn) % 6 + 1][2],
                      fluids[(2 + self.turn) % 6 + 1][3],
                      fluids[(3 + self.turn) % 6 + 1][0],
                      fluids[(3 + self.turn) % 6 + 1][1],
                      fluids[(3 + self.turn) % 6 + 1][2],
                      fluids[(3 + self.turn) % 6 + 1][3],
                      fluids[(4 + self.turn) % 6 + 1][0],
                      fluids[(4 + self.turn) % 6 + 1][1],
                      fluids[(4 + self.turn) % 6 + 1][2],
                      fluids[(4 + self.turn) % 6 + 1][3],
                      fluids[(5 + self.turn) % 6 + 1][0],
                      fluids[(5 + self.turn) % 6 + 1][1],
                      fluids[(5 + self.turn) % 6 + 1][2],
                      fluids[(5 + self.turn) % 6 + 1][3],
                      nc(cells[(0 + self.turn) % 6]),
                      nc(cells[(1 + self.turn) % 6]),
                      nc(cells[(2 + self.turn) % 6]),
                      nc(cells[(3 + self.turn) % 6]),
                      nc(cells[(4 + self.turn) % 6]),
                      nc(cells[(5 + self.turn) % 6]),
                      self.brain.memory[0],
                      self.brain.memory[1],
                      self.brain.memory[2],
                      self.brain.memory[3]];

        let mut compute = self.brain.mep.compute(&inputs[..]);
        let coefficients = [compute.next().unwrap(),
                            compute.next().unwrap(),
                            compute.next().unwrap(),
                            compute.next().unwrap()];
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

        let divide_attempt = compute.next().unwrap();

        let turn_directions = [compute.next().unwrap(),
                               compute.next().unwrap(),
                               compute.next().unwrap(),
                               compute.next().unwrap(),
                               compute.next().unwrap(),
                               compute.next().unwrap()];

        // Handle turn immediately so they can turn to stimuli.
        if let Some(dir) = turn_directions.iter()
            .cloned()
            .enumerate()
            .fold((None, 0.0), |best, n| if n.1 > best.1 {
                (Some(n.0), n.1)
            } else {
                best
            })
            .0 {
            self.turn = dir;
        }

        self.brain.memory.iter_mut().set_from(compute);
        Decision {
            choice: match [move_attempt, divide_attempt, mate_attempt]
                .iter()
                .cloned()
                .enumerate()
                .fold((None, 0.0), |best, n| if n.1 > best.1 {
                    (Some(n.0), n.1)
                } else {
                    best
                })
                .0 {
                Some(0) => {
                    Choice::Move(move_directions[..]
                        .iter()
                        .cycle()
                        .skip(1 + self.turn)
                        .take(5)
                        .cloned()
                        .zip([Direction::UpRight,
                              Direction::UpLeft,
                              Direction::Left,
                              Direction::DownLeft,
                              Direction::DownRight,
                              Direction::Right]
                            .iter()
                            .cycle()
                            .skip(self.turn)
                            .take(6))
                        .fold((move_directions[self.turn], Direction::UpRight),
                              |(bestval, bestdir), (val, &dir)| if val > bestval {
                                  (val, dir)
                              } else {
                                  (bestval, bestdir)
                              })
                        .1)
                }
                Some(1) => {
                    let direction = spawn_directions[..]
                        .iter()
                        .cycle()
                        .skip(1 + self.turn)
                        .take(5)
                        .cloned()
                        .zip([Direction::UpRight,
                              Direction::UpLeft,
                              Direction::Left,
                              Direction::DownLeft,
                              Direction::DownRight,
                              Direction::Right]
                            .iter()
                            .cycle()
                            .skip(self.turn)
                            .take(6))
                        .fold((spawn_directions[self.turn], Direction::UpRight),
                              |(bestval, bestdir), (val, &dir)| if val > bestval {
                                  (val, dir)
                              } else {
                                  (bestval, bestdir)
                              })
                        .1;
                    Choice::Divide {
                        mate: direction,
                        spawn: direction,
                    }
                }
                Some(2) => {
                    Choice::Divide {
                        mate: mate_directions[..]
                            .iter()
                            .cycle()
                            .skip(1 + self.turn)
                            .take(5)
                            .cloned()
                            .zip([Direction::UpRight,
                                  Direction::UpLeft,
                                  Direction::Left,
                                  Direction::DownLeft,
                                  Direction::DownRight,
                                  Direction::Right]
                                .iter()
                                .cycle()
                                .skip(self.turn)
                                .take(6))
                            .fold((mate_directions[self.turn], Direction::UpRight),
                                  |(bestval, bestdir), (val, &dir)| if val > bestval {
                                      (val, dir)
                                  } else {
                                      (bestval, bestdir)
                                  })
                            .1,
                        spawn: spawn_directions[..]
                            .iter()
                            .cycle()
                            .skip(1 + self.turn)
                            .take(5)
                            .cloned()
                            .zip([Direction::UpRight,
                                  Direction::UpLeft,
                                  Direction::Left,
                                  Direction::DownLeft,
                                  Direction::DownRight,
                                  Direction::Right]
                                .iter()
                                .cycle()
                                .skip(self.turn)
                                .take(6))
                            .fold((spawn_directions[self.turn], Direction::UpRight),
                                  |(bestval, bestdir), (val, &dir)| if val > bestval {
                                      (val, dir)
                                  } else {
                                      (bestval, bestdir)
                                  })
                            .1,
                    }
                }
                _ => Choice::Nothing,
            },
            coefficients: {
                let mut ncoef = [0.0; TOTAL_FLUIDS];
                for i in 0..TOTAL_FLUIDS {
                    let f = coefficients[i];
                    ncoef[i] = if f.is_normal() {
                        let nf = sig(f);
                        if nf > 0.0 {
                            NORMAL_DIFFUSION[i] * (nf * 0.5 + 1.0)
                        } else {
                            NORMAL_DIFFUSION[i] * (nf * 1.0 / 3.0 + 1.0)
                        }
                    } else {
                        NORMAL_DIFFUSION[i]
                    };
                }
                ncoef
            },
        }
    }

    pub fn mate(&mut self, other: &Cell, rng: &mut Isaac64Rng) -> Cell {
        self.inhale /= 2;
        Cell {
            inhale: self.inhale,
            color: if rng.next_f32() < COLOR_MUTATION_CHANCE {
                [rng.next_f32(), rng.next_f32(), rng.next_f32()]
            } else {
                self.color
            },
            brain: self.brain.mate(&other.brain, rng),
            turn: self.turn,
        }
    }

    pub fn divide(&mut self, rng: &mut Isaac64Rng) -> Cell {
        self.inhale /= 2;
        Cell {
            inhale: self.inhale,
            color: if rng.next_f32() < COLOR_MUTATION_CHANCE {
                [rng.next_f32(), rng.next_f32(), rng.next_f32()]
            } else {
                self.color
            },
            brain: self.brain.divide(rng),
            turn: self.turn,
        }
    }
}

/// In the range (-1.0, 1.0).
fn sig(v: f64) -> f64 {
    2.0 / (1.0 + (-v).exp()) - 1.0
}
