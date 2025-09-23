use crate::{Weights, resamplers::Resampler};

pub struct StratifiedResampler {}

impl StratifiedResampler {
    pub fn new() -> Self {
        Self {}
    }
}

impl Resampler for &StratifiedResampler {
    fn resample<const N: usize, F: FnMut() -> f32>(
        self,
        weights: Weights<N>,
        mut rngfn: F,
    ) -> impl Iterator<Item = usize> {
        let cumsum: [f32; N] = weights.cum_sum();

        let positions = (0..N).map(move |i| (i as f32 + rngfn()) / (N as f32));

        positions.map(move |target| cumsum.iter().position(|&x| x >= target).unwrap())
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
    fn with_real_rng() {
        let output = resample_real_rng(
            &StratifiedResampler::new(),
            Weights::try_new([0.1, 0.2, 0.3, 0.4]).unwrap(),
        );

        println!("{:?}", output);
    }

    #[test]
    fn with_faked_rng() {
        let output = resample_faked_rng(
            &StratifiedResampler::new(),
            Weights::normalize([3., 3., 1., 1.]).unwrap(),
            vec![0., 0., 0., 0.75],
        );

        assert_eq!(output, [0, 0, 1, 3]);
    }

    #[bench]
    fn bench_resample(b: &mut Bencher) {
        const N: usize = 1000;
        bench::<N, _, _, _>(
            b,
            &StratifiedResampler::new(),
            -3f32..3f32,
            &Gaussian::new(0., 1.),
        )
    }
}
