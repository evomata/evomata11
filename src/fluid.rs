extern crate num;
extern crate nalgebra as na;

pub const TOTAL_FLUIDS: usize = 2;

const TIMESTEP: f64 = 0.2;

#[derive(Default, Debug)]
pub struct Solution {
    pub fluids: [f64; TOTAL_FLUIDS],
    pub coefficients: [f64; TOTAL_FLUIDS],
    diffuse: [f64; TOTAL_FLUIDS],
}

impl Solution {
    pub fn new(fluids: [f64; TOTAL_FLUIDS]) -> Self {
        Solution {
            fluids: fluids,
            coefficients: [1.8, 4.0],
            diffuse: [0.0; TOTAL_FLUIDS],
        }
    }

    pub fn react_deltas(&self) -> [f64; TOTAL_FLUIDS] {
        let b = self.fluids[0];
        let a = self.fluids[1];
        let f = 0.062;
        let k = 0.06093;
        [a * b * b - (k + f) * b, -a * b * b + f * (1.0 - a)]
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
