// As this goes up, so does simulation accuracy, but the reaction-diffusion stuff slows down.
pub const ACCURACY: f64 = 10.0;
pub const TOTAL_FLUIDS: usize = 8;
pub const NORMAL_DIFFUSION: [f64; TOTAL_FLUIDS] = [
    0.0004 * ACCURACY,
    1.0,
    0.5,
    4.0 * ACCURACY,
    SIGNAL_FLUID_DIFFUSION * ACCURACY,
    SIGNAL_FLUID_DIFFUSION * ACCURACY,
    SIGNAL_FLUID_DIFFUSION * ACCURACY,
    SIGNAL_FLUID_DIFFUSION * ACCURACY,
];
pub const RELATIVE_CELL_DIFFUSION: [f64; TOTAL_FLUIDS] = [0.5, 0.5, 0.5, 0.75, 1.0, 1.0, 1.0, 1.0];
pub const KILL_FLUID_NORMAL: f64 = 0.05;
pub const KILL_FLUID_DECAY: f64 = 0.15 * ACCURACY;
pub const KILL_FLUID_UPPER_THRESHOLD: f64 = 0.052;
pub const KILL_FLUID_LOWER_THRESHOLD: f64 = 0.048;
pub const SIGNAL_FLUID_PRODUCTION: f64 = 0.5;
pub const SIGNAL_FLUID_DIFFUSION: f64 = 0.06;
pub const SIGNAL_FLUID_DECAY: f64 = 0.3 * ACCURACY;
pub const B_FOOD_RATE: f64 = 0.0002 * ACCURACY;

const TIMESTEP: f64 = 0.2 / ACCURACY;

#[derive(Serialize, Deserialize)]
pub enum DiffusionType {
    FlatSignals,
    DynSignals,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Solution {
    pub fluids: [f64; TOTAL_FLUIDS],
    pub coefficients: [[f64; TOTAL_FLUIDS]; 6],
    pub diffuse: [f64; TOTAL_FLUIDS],
}

impl Solution {
    pub fn new(fluids: [f64; TOTAL_FLUIDS], coefficients: [[f64; TOTAL_FLUIDS]; 6]) -> Self {
        Solution {
            fluids: fluids,
            coefficients: coefficients,
            diffuse: [0.0; TOTAL_FLUIDS],
        }
    }

    pub fn react_deltas(&self) -> [f64; TOTAL_FLUIDS] {
        let a = self.fluids[1];
        let b = self.fluids[2];
        let kill = self.fluids[3];
        let f = 0.029;
        let k = 0.057;
        [
            B_FOOD_RATE * b,
            -a * b * b + f * (1.0 - a),
            a * b * b - (k + f) * b,
            KILL_FLUID_DECAY * (KILL_FLUID_NORMAL - kill),
            -SIGNAL_FLUID_DECAY * self.fluids[4],
            -SIGNAL_FLUID_DECAY * self.fluids[5],
            -SIGNAL_FLUID_DECAY * self.fluids[6],
            -SIGNAL_FLUID_DECAY * self.fluids[7],
        ]
    }

    pub fn diffuse_from(&mut self, other: &Solution, dtype: DiffusionType, direction: usize) {
        // Handle normal fluids.
        for i in 0..4 {
            self.diffuse[i] += other.fluids[i] * other.coefficients[direction][i] / 6.0;
        }
        // Handle signal fluids.
        match dtype {
            DiffusionType::DynSignals => {
                for i in 4..TOTAL_FLUIDS {
                    self.diffuse[i] += other.fluids[i] * other.coefficients[direction][i] / 6.0;
                }
            }
            DiffusionType::FlatSignals => {
                for i in 4..TOTAL_FLUIDS {
                    self.diffuse[i] += SIGNAL_FLUID_PRODUCTION * other.coefficients[direction][i] /
                        6.0;
                }
            }
        }
    }

    #[inline]
    fn coefficient_sum(&self, fluid: usize) -> f64 {
        let mut acc = 0.0;
        for a in &self.coefficients {
            acc += a[fluid];
        }
        acc / 6.0
    }

    pub fn end_cycle(&mut self) {
        let reacts = self.react_deltas();
        // Handle normal fluids.
        for i in 0..4 {
            self.fluids[i] += TIMESTEP *
                (reacts[i] + self.diffuse[i] - self.coefficient_sum(i) * self.fluids[i]);
            self.diffuse[i] = 0.0;
        }
        // Handle signal fluids.
        for i in 4..TOTAL_FLUIDS {
            self.fluids[i] += TIMESTEP *
                (reacts[i] + self.diffuse[i] - NORMAL_DIFFUSION[i] * self.fluids[i]);
            self.diffuse[i] = 0.0;
        }
    }
}
