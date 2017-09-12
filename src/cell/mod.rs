mod brain;

use rand::{Isaac64Rng, Rng};
use itertools::Itertools;
use fluid::{NORMAL_DIFFUSION, RELATIVE_CELL_DIFFUSION, TOTAL_FLUIDS};

use std::cmp::Ordering::*;

const INITIAL_INHALE: usize = 2000;

enum_from_primitive! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
            UpRight => if even_y { (1, -1) } else { (0, -1) },
            UpLeft => if even_y { (0, -1) } else { (-1, -1) },
            Left => (-1, 0),
            DownLeft => if even_y { (0, 1) } else { (-1, 1) },
            DownRight => if even_y { (1, 1) } else { (0, 1) },
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Choice {
    // If the mate and spawn direction are the same, cause a divide.
    Divide { mate: Direction, spawn: Direction },
    Move(Direction),
    Explode(bool),
    Suicide,
    Nothing,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Decision {
    pub choice: Choice,
    pub coefficients: [[f64; TOTAL_FLUIDS]; 6],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cell {
    pub inhale: usize,
    pub suicide: bool,
    pub brain: brain::Brain,
    turn: usize,
}

impl Cell {
    pub fn new(rng: &mut Isaac64Rng) -> Self {
        Cell {
            inhale: INITIAL_INHALE,
            suicide: false,
            brain: brain::Brain::new(rng),
            turn: rng.gen_range(0, 6),
        }
    }

    fn choose_direction(&self, choices: [f64; 6]) -> Direction {
        choices[..]
            .iter()
            .cloned()
            .zip(
                [
                    Direction::UpRight,
                    Direction::UpLeft,
                    Direction::Left,
                    Direction::DownLeft,
                    Direction::DownRight,
                    Direction::Right,
                ].iter()
                    .cycle()
                    .skip(self.turn)
                    .take(6),
            )
            .max_by(|n0, n1| if n0.0 < n1.0 { Less } else { Greater })
            .unwrap()
            .1
            .clone()
    }

    pub fn decide(&mut self, fluids: [&[f64; TOTAL_FLUIDS]; 7], cells: &[bool; 6]) -> Decision {
        use mli::Stateless;
        use std::f64::{MAX, MIN};
        let nc = |n: bool| if n { 1.0 } else { 0.0 };
        let inputs = [
            0.0,
            0.5,
            -0.5,
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
            fluids[0][4],
            fluids[0][5],
            fluids[0][6],
            fluids[0][7],
            fluids[(0 + self.turn) % 6 + 1][0],
            fluids[(0 + self.turn) % 6 + 1][1],
            fluids[(0 + self.turn) % 6 + 1][2],
            fluids[(0 + self.turn) % 6 + 1][3],
            fluids[(0 + self.turn) % 6 + 1][4],
            fluids[(0 + self.turn) % 6 + 1][5],
            fluids[(0 + self.turn) % 6 + 1][6],
            fluids[(0 + self.turn) % 6 + 1][7],
            fluids[(1 + self.turn) % 6 + 1][0],
            fluids[(1 + self.turn) % 6 + 1][1],
            fluids[(1 + self.turn) % 6 + 1][2],
            fluids[(1 + self.turn) % 6 + 1][3],
            fluids[(1 + self.turn) % 6 + 1][4],
            fluids[(1 + self.turn) % 6 + 1][5],
            fluids[(1 + self.turn) % 6 + 1][6],
            fluids[(1 + self.turn) % 6 + 1][7],
            fluids[(2 + self.turn) % 6 + 1][0],
            fluids[(2 + self.turn) % 6 + 1][1],
            fluids[(2 + self.turn) % 6 + 1][2],
            fluids[(2 + self.turn) % 6 + 1][3],
            fluids[(2 + self.turn) % 6 + 1][4],
            fluids[(2 + self.turn) % 6 + 1][5],
            fluids[(2 + self.turn) % 6 + 1][6],
            fluids[(2 + self.turn) % 6 + 1][7],
            fluids[(3 + self.turn) % 6 + 1][0],
            fluids[(3 + self.turn) % 6 + 1][1],
            fluids[(3 + self.turn) % 6 + 1][2],
            fluids[(3 + self.turn) % 6 + 1][3],
            fluids[(3 + self.turn) % 6 + 1][4],
            fluids[(3 + self.turn) % 6 + 1][5],
            fluids[(3 + self.turn) % 6 + 1][6],
            fluids[(3 + self.turn) % 6 + 1][7],
            fluids[(4 + self.turn) % 6 + 1][0],
            fluids[(4 + self.turn) % 6 + 1][1],
            fluids[(4 + self.turn) % 6 + 1][2],
            fluids[(4 + self.turn) % 6 + 1][3],
            fluids[(4 + self.turn) % 6 + 1][4],
            fluids[(4 + self.turn) % 6 + 1][5],
            fluids[(4 + self.turn) % 6 + 1][6],
            fluids[(4 + self.turn) % 6 + 1][7],
            fluids[(5 + self.turn) % 6 + 1][0],
            fluids[(5 + self.turn) % 6 + 1][1],
            fluids[(5 + self.turn) % 6 + 1][2],
            fluids[(5 + self.turn) % 6 + 1][3],
            fluids[(5 + self.turn) % 6 + 1][4],
            fluids[(5 + self.turn) % 6 + 1][5],
            fluids[(5 + self.turn) % 6 + 1][6],
            fluids[(5 + self.turn) % 6 + 1][7],
            nc(cells[(0 + self.turn) % 6]),
            nc(cells[(1 + self.turn) % 6]),
            nc(cells[(2 + self.turn) % 6]),
            nc(cells[(3 + self.turn) % 6]),
            nc(cells[(4 + self.turn) % 6]),
            nc(cells[(5 + self.turn) % 6]),
            self.brain.memory[0],
            self.brain.memory[1],
            self.brain.memory[2],
            self.brain.memory[3],
        ];

        let mut compute = self.brain.mep.process(&inputs[..]);

        let mut coefficients = [[0.0; TOTAL_FLUIDS]; 6];
        for da in &mut coefficients {
            for f in da {
                *f = compute.next().unwrap();
            }
        }

        let move_attempt = compute.next().unwrap();

        let mut move_directions = [0f64; 6];
        for f in &mut move_directions {
            *f = compute.next().unwrap();
        }

        let mate_attempt = compute.next().unwrap();

        let mut mate_directions = [0f64; 6];
        for f in &mut mate_directions {
            *f = compute.next().unwrap();
        }

        let mut spawn_directions = [0f64; 6];
        for f in &mut spawn_directions {
            *f = compute.next().unwrap();
        }

        let mut turn_directions = [0f64; 6];
        for f in &mut turn_directions {
            *f = compute.next().unwrap();
        }

        let explode_attempt = compute.next().unwrap();

        let suicide_attempt = compute.next().unwrap();

        // Handle turn immediately so they can turn to stimuli.
        if let Some(dir) = turn_directions
            .iter()
            .cloned()
            .enumerate()
            .max_by(|n0, n1| if n0.1 < n1.1 { Less } else { Greater })
            .map(|n| n.0)
        {
            // Shift the turn direction by how much we have turned so it is relative.
            self.turn = (dir + self.turn) % 6;
        }

        self.brain.memory.iter_mut().set_from(compute);
        Decision {
            choice: match [move_attempt, mate_attempt, explode_attempt, suicide_attempt]
                .iter()
                .cloned()
                .enumerate()
                .filter(|p| p.1 > 0.0)
                .max_by(|n0, n1| if n0.1 < n1.1 { Less } else { Greater })
                .map(|n| n.0) {
                Some(0) => Choice::Move(self.choose_direction(move_directions)),
                Some(1) => {
                    Choice::Divide {
                        mate: self.choose_direction(mate_directions),
                        spawn: self.choose_direction(spawn_directions),
                    }
                }
                Some(2) => {
                    if explode_attempt > 1.0 {
                        Choice::Explode(true)
                    } else if explode_attempt < -1.0 {
                        Choice::Explode(false)
                    } else {
                        Choice::Nothing
                    }
                }
                Some(3) => {
                    if suicide_attempt > 1.0 {
                        Choice::Suicide
                    } else {
                        Choice::Nothing
                    }
                }
                _ => Choice::Nothing,
            },
            coefficients: {
                let mut ncoef = [[0.0; TOTAL_FLUIDS]; 6];
                // Handle normal fluids.
                for (i, fa) in ncoef.iter_mut().enumerate() {
                    for (j, f) in fa.iter_mut().enumerate() {
                        *f =
                        // Handle normal fluids.
                        if j < 4 {
                            let f = coefficients[i][j];
                            if f.is_normal() {
                                NORMAL_DIFFUSION[j] * (sig(f) * RELATIVE_CELL_DIFFUSION[j] + 1.0)
                            } else {
                                NORMAL_DIFFUSION[j]
                            }
                        // Handle signal fluids.
                        } else {
                            let f = coefficients[i][j];
                            if f.is_normal() {
                                sig(f)
                            } else {
                                0.0
                            }
                        };
                    }
                }
                ncoef
            },
        }
    }

    pub fn mate(&mut self, other: &Cell, rng: &mut Isaac64Rng) -> Cell {
        self.inhale /= 2;
        Cell {
            inhale: self.inhale,
            suicide: false,
            brain: self.brain.mate(&other.brain, rng),
            turn: self.turn,
        }
    }

    pub fn divide(&mut self, rng: &mut Isaac64Rng) -> Cell {
        self.inhale /= 2;
        Cell {
            inhale: self.inhale,
            suicide: false,
            brain: self.brain.divide(rng),
            turn: self.turn,
        }
    }
}

/// In the range (-1.0, 1.0).
fn sig(v: f64) -> f64 {
    2.0 / (1.0 + (-v).exp()) - 1.0
}
