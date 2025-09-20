use crate::{resamplers::Resampler, weights::Weights};

pub struct MultinomialResampler {}

impl MultinomialResampler {
    pub fn new() -> Self {
        Self {}
    }
}

impl Resampler for MultinomialResampler {
    fn resample<const N: usize, F: FnMut() -> f32>(
        self,
        input: Weights<N>,
        mut rng: F,
    ) -> impl Iterator<Item = usize> {
        let cumsum: [f32; N] = input.cum_sum();

        std::iter::from_fn(move || {
            let target: f32 = (rng)();
            Some(cumsum.iter().position(|&x| x >= target).unwrap())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resamplers::test;

    /// Does not test output, just that it runs without panic
    #[test]
    fn test_with_rng() {
        let output = test::resample_real_rng(
            MultinomialResampler::new(),
            Weights::try_new([0.1, 0.2, 0.3, 0.4]).unwrap(),
        );
        println!("{:?}", output);
    }

    #[test]
    fn test_with_faked_rng() {
        let output = test::resample_faked_rng(
            MultinomialResampler::new(),
            Weights::try_new([0.1, 0.2, 0.3, 0.4]).unwrap(),
            vec![0.05f32, 0.8f32, 0.2f32, 0.35f32],
        );

        assert_eq!(output, [0, 3, 1, 2]);
    }
}
