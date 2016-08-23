use rand::{Rng, Isaac64Rng};
use num::FromPrimitive;

enum_from_primitive! {
    #[derive(Debug, PartialEq, Eq)]
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

pub enum Choice {
    Divide {
        mate: Direction,
        spawn: Direction,
    },
    Move(Direction),
}

pub struct Decision {
    pub choice: Choice,
}

pub struct Delta {
    placeholder: i32,
}

struct Cell {
    delta: Delta,
}

impl Cell {
    fn new() -> Self {
        Cell { delta: Delta { placeholder: 0 } }
    }

    fn decide(&mut self, rng: &mut Isaac64Rng) -> Decision {
        Decision {
            choice: Choice::Move(Direction::from_u32(rng.gen_range(0, Direction::TOTAL as u32))
                .unwrap()),
        }
    }
}
