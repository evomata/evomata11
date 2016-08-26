use rand::{Rng, Isaac64Rng};
use num::FromPrimitive;

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

#[derive(Clone, Debug)]
pub struct Cell {
    placeholder: usize,
}

impl Cell {
    pub fn new() -> Self {
        Cell { placeholder: 0 }
    }

    pub fn color(&self) -> [f32; 4] {
        [1.0, 1.0, 1.0, 1.0]
    }

    pub fn decide(&mut self,
                  fluids: &[f64; 2],
                  cells: &[bool; 6],
                  rng: &mut Isaac64Rng)
                  -> Decision {
        Decision {
            choice: Choice::Move(Direction::Right),
            coefficients: [0.5, 1.0],
        }
    }

    pub fn mate(&mut self, other: &Cell, rng: &mut Isaac64Rng) -> Cell {
        self.clone()
    }
}
