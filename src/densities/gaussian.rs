use super::DensityFunc;

pub struct Gaussian {
    mean: f32,
    std_dev: f32,
    coeff: f32,
}
impl Gaussian {
    pub fn new(mean: f32, std_dev: f32) -> Self {
        use std::f32::consts;
        Self {
            mean,
            std_dev,
            coeff: 1. / (std_dev * (2.0 * consts::PI).sqrt()),
        }
    }
}
impl DensityFunc for Gaussian {
    fn eval(&self, x: f32) -> f32 {
        // f(x) = (1 / sqrt(2π σ²)) * exp(-((x - μ)² / (2σ²)))
        self.coeff * (-0.5 * ((x - self.mean) / self.std_dev).powi(2)).exp()
    }
}
