use rand::{Rng, Isaac64Rng};
use itertools::Itertools;
use mli;

// 0.0, 1.0, -1.0, 2.0, -2.0, MAX, MIN
pub const CONST_INPUTS: usize = 7;
// Inhale, Fluids (7 * 3), neighbor present (6).
pub const STATIC_INPUTS: usize = 1 + 7 * 3 + 6;
pub const TOTAL_MEMORY: usize = 4;
pub const TOTAL_INPUTS: usize = CONST_INPUTS + STATIC_INPUTS + TOTAL_MEMORY;
// Coefficients (3), Movement(7), Mate(13), Divide, Turn(6)
pub const STATIC_OUTPUTS: usize = 3 + 7 + 13 + 1 + 6;
pub const TOTAL_OUTPUTS: usize = STATIC_OUTPUTS + TOTAL_MEMORY;
pub const DEFAULT_MUTATE_SIZE: usize = 8;
pub const DEFAULT_CROSSOVER_POINTS: usize = 1;
pub const DEFAULT_INSTRUCTIONS: usize = 128;
const MUTATE_PROBABILITY: f64 = 1.0;

#[derive(Clone)]
pub enum Ins {
    _NOP,
    _ADD,
    _SUB,
    _MUL,
    _DIV,
    _MOD,
    _GRT,
    _LES,
    _AND,
    _OR,
    _POW,
    _EXP,
    _LOG,
    _LN,
    _SIN,
    _SQT,
    MAX,
}

fn processor(ins: &Ins, a: f64, b: f64) -> f64 {
    match *ins {
        Ins::_NOP => a,
        Ins::_ADD => a + b,
        Ins::_SUB => a - b,
        Ins::_MUL => a * b,
        Ins::_DIV => a / b,
        Ins::_MOD => a % b,
        Ins::_GRT => {
            if a > b {
                1.0
            } else {
                0.0
            }
        }
        Ins::_LES => {
            if a < b {
                1.0
            } else {
                0.0
            }
        }
        Ins::_AND => {
            if a >= 1.0 && b >= 1.0 {
                1.0
            } else {
                0.0
            }
        }
        Ins::_OR => {
            if a >= 1.0 || b >= 1.0 {
                1.0
            } else {
                0.0
            }
        }
        Ins::_POW => a.powf(b),
        Ins::_EXP => a.exp(),
        Ins::_LOG => a.log(b),
        Ins::_LN => a.ln(),
        Ins::_SIN => a.sin(),
        Ins::_SQT => a.sqrt(),
        Ins::MAX => unreachable!(),
    }
}

fn mutator(ins: &mut Ins, rng: &mut Isaac64Rng) {
    use std::mem;
    *ins = unsafe { mem::transmute(rng.gen_range::<u8>(0, Ins::MAX as u8)) };
}

#[derive(Clone)]
pub struct Decision {
    pub mate: i64,
    pub node: i64,
    // This will be ran through a sigmoid
    pub rate: i64,
    pub signal: i64,
    pub connect_signal: i64,
    pub sever_choice: i64,
    pub pull: i64,
}

impl Default for Decision {
    fn default() -> Self {
        Decision {
            mate: -1,
            node: -1,
            rate: 0,
            signal: 0,
            connect_signal: 0,
            sever_choice: 0,
            pull: 0,
        }
    }
}

#[derive(Clone)]
pub struct Brain {
    pub mep: mli::Mep<Ins,
                      Isaac64Rng,
                      f64,
                      fn(&mut Ins, &mut Isaac64Rng),
                      fn(&Ins, f64, f64) -> f64>,
    pub memory: [f64; TOTAL_MEMORY],
}

impl Brain {
    pub fn new(rng: &mut Isaac64Rng) -> Self {
        let v = (0..DEFAULT_INSTRUCTIONS)
            .map(|_| {
                let mut ins = Ins::_NOP;
                mutator(&mut ins, rng);
                ins
            })
            .collect_vec();
        Brain {
            mep: mli::Mep::new(TOTAL_INPUTS,
                               TOTAL_OUTPUTS,
                               DEFAULT_MUTATE_SIZE,
                               DEFAULT_CROSSOVER_POINTS,
                               rng,
                               v.into_iter(),
                               mutator,
                               processor),
            memory: [0.0; TOTAL_MEMORY],
        }
    }

    pub fn mutate(&mut self, rng: &mut Isaac64Rng) {
        use mli::Genetic;
        if rng.gen_range(0.0, 1.0) < MUTATE_PROBABILITY {
            self.mep.mutate(rng);
        }
    }

    pub fn mate(&self, other: &Self, rng: &mut Isaac64Rng) -> Self {
        use mli::Genetic;
        let mut b = Brain {
            mep: Genetic::mate((&self.mep, &other.mep), rng),
            memory: [0.0; TOTAL_MEMORY],
        };
        // Perform unit mutations on offspring
        b.mutate(rng);
        b
    }

    pub fn divide(&self, rng: &mut Isaac64Rng) -> Self {
        let mut b = Brain {
            mep: self.mep.clone(),
            memory: [0.0; TOTAL_MEMORY],
        };
        // Perform unit mutations on offspring
        b.mutate(rng);
        b
    }
}
