use crate::{Weights, resamplers::Resampler};

#[derive(Debug, Clone, Copy)]
pub struct SystematicResampler {}

impl SystematicResampler {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Clone)]
struct SystematicIterator {
    /// Sorted greatest to least
    cum_weights: Vec<f32>,
    step: f32,
    position: f32,
    index: usize,
}

impl SystematicIterator {
    fn new<const N: usize, F: FnMut() -> f32>(weights: Weights<N>, mut rng_fn: F) -> Self {
        let mut cum_weights = weights.cum_sum();
        cum_weights.reverse();
        let cum_weights = cum_weights.to_vec();

        let step = 1f32 / N as f32;
        let start = rng_fn() * step;
        Self {
            cum_weights,
            step,
            position: start,
            index: 0,
        }
    }
}

impl Iterator for SystematicIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.cum_weights.pop_if(|x| *x < self.position).is_some() {
            self.index += 1;
        }
        self.position += self.step;
        Some(self.index)
    }
}

impl Resampler for &SystematicResampler {
    fn resample<const N: usize, F: FnMut() -> f32>(
        self,
        weights: Weights<N>,
        mut rng: F,
    ) -> impl Iterator<Item = usize> {
        SystematicIterator::new(weights, &mut rng)
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;

    use crate::densities::Gaussian;

    use super::{
        super::test::{bench, resample_faked_rng, resample_real_rng},
        *,
    };

    /// Does not test output, just that it runs without panic
    #[test]
    fn test_with_rng() {
        let output = resample_real_rng(
            &SystematicResampler::new(),
            Weights::try_new([0.1, 0.2, 0.3, 0.4]).unwrap(),
        );
        println!("{:?}", output);
    }

    #[test]
    fn test_with_faked_rng() {
        let output = resample_faked_rng(
            &SystematicResampler::new(),
            Weights::normalize([3., 1., 1., 3.]).unwrap(),
            vec![0.49f32],
        );
        assert_eq!(output, [0, 0, 2, 3]);
    }

    #[bench]
    fn bench_resample(b: &mut Bencher) {
        const N: usize = 1000;
        bench::<N, _, _, _>(
            b,
            &SystematicResampler::new(),
            -3f32..3f32,
            &Gaussian::new(0., 1.),
        )
    }
}
