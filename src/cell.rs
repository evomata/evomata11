use rand::{Rng, Isaac64Rng};
use num::FromPrimitive;

enum_from_primitive! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Direction {
        UpRight,
        UpLeft,
        Left,
        DownLeft,
        DownRight,
        Right,
        TOTAL,
    }
}

#[derive(Clone, Debug)]
pub enum Choice {
    Divide {
        mate: Direction,
        spawn: Direction,
    },
    Move(Direction),
}

#[derive(Clone, Debug)]
pub struct Decision {
    pub choice: Choice,
}

#[derive(Clone, Debug)]
pub struct Delta {
    placeholder: i32,
}

#[derive(Clone, Debug)]
pub struct Cell {
    delta: Delta,
}

impl Cell {
    pub fn new() -> Self {
        Cell { delta: Delta { placeholder: 0 } }
    }

    pub fn decide(&mut self, rng: &mut Isaac64Rng) -> Decision {
        Decision {
            choice: Choice::Move(Direction::from_u32(rng.gen_range(0, Direction::TOTAL as u32))
                .unwrap()),
        }
    }

    pub fn mate(&mut self, other: &Cell, rng: &mut Isaac64Rng) -> Cell {
        self.clone()
    }
}
