use std::iter;

use crate::{resamplers::Resampler, weights::Weights};

#[derive(Debug, Clone, Copy)]
pub struct MultinomialResampler;

impl MultinomialResampler {
    pub fn new() -> Self {
        Self {}
    }
}

impl Resampler for MultinomialResampler {
    fn resample<const N: usize, F: FnMut() -> f32>(
        self,
        weights: Weights<N>,
        rng_fn: F,
    ) -> impl Iterator<Item = usize> {
        let cumsum: [f32; N] = weights.cum_sum();
        let mut rng_fn = rng_fn;

        iter::from_fn(move || {
            let target: f32 = rng_fn();
            Some(cumsum.iter().position(|&x| x >= target).unwrap())
        })
    }
}

impl Resampler for &MultinomialResampler {
    fn resample<const N: usize, F: FnMut() -> f32>(
        self,
        weights: Weights<N>,
        rng_fn: F,
    ) -> impl Iterator<Item = usize> {
        (*self).resample(weights, rng_fn)
    }
}

#[cfg(test)]
mod tests {
    use super::{super::test, *};

    /// Does not test output, just that it runs without panic
    #[test]
    fn test_with_rng() {
        let output = test::resample_real_rng(
            &MultinomialResampler::new(),
            Weights::try_new([0.1, 0.2, 0.3, 0.4]).unwrap(),
        );
        println!("{:?}", output);
    }

    #[test]
    fn test_with_faked_rng() {
        let output = test::resample_faked_rng(
            &MultinomialResampler::new(),
            Weights::try_new([0.1, 0.2, 0.3, 0.4]).unwrap(),
            vec![0.05f32, 0.8f32, 0.2f32, 0.35f32],
        );

        assert_eq!(output, [0, 3, 1, 2]);
    }
}
