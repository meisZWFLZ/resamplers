use crate::{Weights, resamplers::Resampler};

pub struct ResidualResampler<Fract: Resampler + Clone> {
    fractional: Fract,
}

impl<F: Resampler + Clone> ResidualResampler<F> {
    pub fn new(fractional: F) -> Self {
        Self { fractional }
    }
}

// TODO: Figure out implementing Systematic as a fractional resampler
impl<Fract: Resampler + Clone> Resampler for &ResidualResampler<Fract> {
    fn resample<const N: usize, F: FnMut() -> f32>(
        self,
        weights: Weights<N>,
        rng: F,
    ) -> impl Iterator<Item = usize> {
        let weights: [f32; N] = weights.as_array().map(|w| w * N as f32);

        let fractional_iter = self
            .fractional
            .clone()
            .resample(Weights::normalize(weights.map(|x| x.fract())).unwrap(), rng);
        weights
            .into_iter()
            .enumerate()
            .flat_map(|(i, x)| {
                let n = x.floor() as usize;
                std::iter::repeat(i).take(n)
            })
            .chain(fractional_iter)
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;

    use super::*;
    use crate::{
        densities::Gaussian,
        resamplers::{
            multinomial::MultinomialResampler,
            test::{bench, resample_real_rng},
        },
    };

    /// Does not test output, just that it runs without panic
    #[test]
    fn test_with_rng() {
        let output = resample_real_rng(
            &ResidualResampler::new(MultinomialResampler::new()),
            Weights::try_new([0.1, 0.2, 0.3, 0.4]).unwrap(),
        );
        println!("{:?}", output);
    }

    #[bench]
    fn bench_resample(b: &mut Bencher) {
        const N: usize = 1000;
        bench::<N, _, _, _>(
            b,
            &ResidualResampler::new(MultinomialResampler::new()),
            -3f32..3f32,
            &Gaussian::new(0., 1.),
        )
    }
}
