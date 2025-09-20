use crate::{Weights, resamplers::Resampler};

pub struct SystematicResampler {}

impl SystematicResampler {
    pub fn new() -> Self {
        Self {}
    }
}

impl Resampler for &SystematicResampler {
    fn resample<const N: usize, F: FnMut() -> f32>(
        self,
        weights: Weights<N>,
        mut rng: F,
    ) -> impl Iterator<Item = usize> {
        let cum_weights = weights.cum_sum();
        let step = 1.0f32 / N as f32;
        let start = rng() * step;
        let positions = (0..N).map(move |i| start + i as f32 * step);

        positions.map(move |target| cum_weights.iter().position(|&x| x >= target).unwrap())
    }
}

#[cfg(test)]
mod tests {

    use super::{super::test, *};

    /// Does not test output, just that it runs without panic
    #[test]
    fn test_with_rng() {
        let output = test::resample_real_rng(
            &SystematicResampler::new(),
            Weights::try_new([0.1, 0.2, 0.3, 0.4]).unwrap(),
        );
        println!("{:?}", output);
    }

    #[test]
    fn test_with_faked_rng() {
        let output = test::resample_faked_rng(
            &SystematicResampler::new(),
            Weights::normalize([3., 1., 1., 3.]).unwrap(),
            vec![0.49f32],
        );
        assert_eq!(output, [0, 0, 2, 3]);
    }
}
