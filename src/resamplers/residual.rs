use crate::{
    resamplers::{Resampler, multinomial::MultinomialResampler},
    weights::Weights,
};

pub struct ResidualResampler {}

impl ResidualResampler {
    pub fn new() -> Self {
        Self {}
    }
}

impl Resampler for ResidualResampler {
    fn resample<const N: usize, F: FnMut() -> f32>(
        self,
        weights: Weights<N>,
        rng: F,
    ) -> impl Iterator<Item = usize> {
        let weights: [f32; N] = weights.as_array().map(|w| w * N as f32);

        weights
            .into_iter()
            .enumerate()
            .flat_map(|(i, x)| {
                let n = x.floor() as usize;
                std::iter::repeat(i).take(n)
            })
            .chain({
                let multinomial_resampler = MultinomialResampler::new();
                multinomial_resampler
                    .resample(Weights::normalize(weights.map(|x| x.fract())).unwrap(), rng)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Does not test output, just that it runs without panic
    #[test]
    fn test_with_rng() {
        let output = crate::resamplers::test::resample_real_rng(
            ResidualResampler::new(),
            Weights::try_new([0.1, 0.2, 0.3, 0.4]).unwrap(),
        );
        println!("{:?}", output);
    }
}
