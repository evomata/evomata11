extern crate num;
extern crate nalgebra as na;

pub const TOTAL_FLUIDS: usize = 8;
pub const NORMAL_DIFFUSION: [f64; TOTAL_FLUIDS] = [0.0004, 1.0, 0.2, 0.5, 0.5, 0.5, 0.5, 0.5];
pub const KILL_FLUID_NORMAL: f64 = 0.05;
pub const KILL_FLUID_DECAY: f64 = 0.01;
pub const KILL_FLUID_UPPER_THRESHOLD: f64 = 0.052;
pub const KILL_FLUID_LOWER_THRESHOLD: f64 = 0.048;
pub const SIGNAL_FLUID_NORMAL: f64 = 0.5;
pub const SIGNAL_FLUID_DECAY: f64 = 0.001;
pub const B_FOOD_RATE: f64 = 0.003;

const TIMESTEP: f64 = 0.2;

#[derive(Default, Debug)]
pub struct Solution {
    pub fluids: [f64; TOTAL_FLUIDS],
    pub coefficients: [f64; TOTAL_FLUIDS],
    pub diffuse: [f64; TOTAL_FLUIDS],
}

impl Solution {
    pub fn new(fluids: [f64; TOTAL_FLUIDS], coefficients: [f64; TOTAL_FLUIDS]) -> Self {
        Solution {
            fluids: fluids,
            coefficients: coefficients,
            diffuse: [0.0; TOTAL_FLUIDS],
        }
    }

    pub fn react_deltas(&self) -> [f64; TOTAL_FLUIDS] {
        let a = self.fluids[1];
        let b = self.fluids[3];
        let kill = self.fluids[2];
        let f = 0.029;
        let k = 0.057;
        [B_FOOD_RATE * b,
         -a * b * b + f * (1.0 - a),
         KILL_FLUID_DECAY * (KILL_FLUID_NORMAL - kill),
         a * b * b - (k + f) * b,
         SIGNAL_FLUID_DECAY * (SIGNAL_FLUID_NORMAL - self.fluids[4]),
         SIGNAL_FLUID_DECAY * (SIGNAL_FLUID_NORMAL - self.fluids[5]),
         SIGNAL_FLUID_DECAY * (SIGNAL_FLUID_NORMAL - self.fluids[6]),
         SIGNAL_FLUID_DECAY * (SIGNAL_FLUID_NORMAL - self.fluids[7])]
    }

    pub fn diffuse_from(&mut self, other: &Solution) {
        for i in 0..TOTAL_FLUIDS {
            self.diffuse[i] += other.fluids[i] * other.coefficients[i] / 6.0;
        }
    }

    pub fn end_cycle(&mut self) {
        let reacts = self.react_deltas();
        for i in 0..TOTAL_FLUIDS {
            self.fluids[i] += TIMESTEP *
                              (reacts[i] + self.diffuse[i] - self.coefficients[i] * self.fluids[i]);
            self.diffuse[i] = 0.0;
        }
    }
}
