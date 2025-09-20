use crate::Weights;
use crate::resamplers::Resampler;

pub struct StratifiedResampler {}

impl StratifiedResampler {
    pub fn new() -> Self {
        Self {}
    }
}

impl Resampler for StratifiedResampler {
    fn resample<const N: usize, F: FnMut() -> f32>(
        self,
        weights: Weights<N>,
        mut rngfn: F,
    ) -> impl Iterator<Item = usize> {
        let cumsum: [f32; N] = weights.cum_sum();

        let positions = (0..N).map(move |i| (i as f32 + (rngfn)()) / (N as f32));

        positions.map(move |target| cumsum.iter().position(|&x| x >= target).unwrap())
    }
}

#[cfg(test)]
mod tests {

    use super::super::test;
    use super::*;

    /// Does not test output, just that it runs without panic
    #[test]
    fn with_real_rng() {
        let output = test::resample_real_rng(
            StratifiedResampler::new(),
            Weights::try_new([0.1, 0.2, 0.3, 0.4]).unwrap(),
        );

        println!("{:?}", output);
    }

    #[test]
    fn with_faked_rng() {
        let output = test::resample_faked_rng(
            StratifiedResampler::new(),
            Weights::normalize([3., 3., 1., 1.]).unwrap(),
            vec![0., 0., 0., 0.75]
        );

        assert_eq!(output, [0, 0, 1, 3]);
    }
}
