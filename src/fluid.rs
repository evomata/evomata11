extern crate num;
extern crate nalgebra as na;

const TOTAL_FLUIDS: usize = 4;
const DIFFUSION_COEFFICIENT: f64 = 0.001;

#[derive(Default, Debug)]
pub struct Solution {
    pub fluids: [u64; TOTAL_FLUIDS],
    diffuse: [u64; TOTAL_FLUIDS],
}

impl Solution {
    pub fn new(fluids: [u64; TOTAL_FLUIDS]) -> Self {
        Solution {
            fluids: fluids,
            diffuse: [0; TOTAL_FLUIDS],
        }
    }

    pub fn react(&mut self) {
        // TODO: Implement.
    }

    pub fn diffuse_from(&mut self, other: &Solution) {
        for (diffuse, &fluid) in self.diffuse.iter_mut().zip(other.fluids.iter()) {
            *diffuse += (fluid as f64 * DIFFUSION_COEFFICIENT) as u64;
        }
    }

    pub fn diffuse_cycle(&mut self) {
        for (fluid, diffuse) in self.fluids.iter_mut().zip(self.diffuse.iter_mut()) {
            *fluid += *diffuse - (*fluid as f64 * DIFFUSION_COEFFICIENT * 6.0) as u64;
            *diffuse = 0;
        }
    }
}
