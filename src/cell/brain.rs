use rand::{Rng, Rand, Isaac64Rng};
use mli::{MateRand, Stateless, Mutate};
use mli_mep::Mep;

// 0.0, 0.5, -0.5, 1.0, -1.0, 2.0, -2.0, MAX, MIN
pub const CONST_INPUTS: usize = 9;
// Inhale, Fluids (7 * 8), neighbor present (6).
pub const STATIC_INPUTS: usize = 1 + 7 * 8 + 6;
pub const TOTAL_MEMORY: usize = 4;
pub const TOTAL_INPUTS: usize = CONST_INPUTS + STATIC_INPUTS + TOTAL_MEMORY;
// Coefficients (8 * 6), Movement(7), Mate(13), Divide, Turn(6), Explode, Suicide
pub const STATIC_OUTPUTS: usize = 8 * 6 + 7 + 13 + 1 + 6 + 1 + 1;
pub const TOTAL_OUTPUTS: usize = STATIC_OUTPUTS + TOTAL_MEMORY;
pub const DEFAULT_MUTATE_LAMBDA: usize = 8;
pub const DEFAULT_CROSSOVER_POINTS: usize = 1;
pub const INTERNAL_INSTRUCTIONS: usize = 128;
const MUTATE_PROBABILITY: f64 = 1.0;

#[derive(Clone, Debug, Serialize, Deserialize)]
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

impl<'a> Stateless<'a, (f64, f64), f64> for Ins {
    fn process(&'a self, (a, b): (f64, f64)) -> f64 {
        match *self {
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
}

impl Rand for Ins {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        use std::mem;
        unsafe { mem::transmute(rng.gen_range::<u8>(0, Ins::MAX as u8)) }
    }
}

impl<R> Mutate<R> for Ins
    where R: Rng
{
    fn mutate(&mut self, rng: &mut R) {
        *self = rng.gen()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Brain {
    pub mep: Mep<Ins>,
    pub memory: [f64; TOTAL_MEMORY],
}

impl Brain {
    pub fn new(rng: &mut Isaac64Rng) -> Self {
        Brain {
            mep: Mep::new(TOTAL_INPUTS,
                               TOTAL_OUTPUTS,
                               INTERNAL_INSTRUCTIONS,
                               DEFAULT_MUTATE_LAMBDA,
                               DEFAULT_CROSSOVER_POINTS,
                               rng),
            memory: [0.0; TOTAL_MEMORY],
        }
    }

    pub fn mutate(&mut self, rng: &mut Isaac64Rng) {
        if rng.gen_range(0.0, 1.0) < MUTATE_PROBABILITY {
            self.mep.mutate(rng);
        }
    }

    pub fn mate(&self, other: &Self, rng: &mut Isaac64Rng) -> Self {
        let mut b = Brain {
            mep: self.mep.mate(&other.mep, rng),
            memory: [0.0; TOTAL_MEMORY],
        };
        // Perform unit mutations on offspring.
        b.mutate(rng);
        b
    }

    pub fn divide(&self, rng: &mut Isaac64Rng) -> Self {
        let mut b = Brain {
            mep: self.mep.clone(),
            memory: [0.0; TOTAL_MEMORY],
        };
        // Perform unit mutations on offspring.
        b.mutate(rng);
        b
    }
}
