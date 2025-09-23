//! Provides testing utilities for resamplers

#![cfg(test)]

extern crate test;
use crate::{Weights, densities::DensityFunc, resamplers::Resampler};

/// Resample using a real RNG (Xoroshiro128PlusPlus with a fixed seed)
pub(crate) fn resample_real_rng<R: Resampler, const N: usize>(
    resampler: R,
    weights: Weights<N>,
) -> [usize; N] {
    use rand::Rng;
    use rand_xoshiro::Xoroshiro128PlusPlus;
    use rand_xoshiro::rand_core::SeedableRng;

    let mut rng = Xoroshiro128PlusPlus::seed_from_u64(42);
    let mut rngfn = move || rng.sample(rand::distr::StandardUniform);

    resampler
        .resample(weights, &mut rngfn)
        .take(N)
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap()
}

/// Resample using a faked RNG that returns values from rng_vals in order.
/// Will panic if rng_vals is exhausted.
pub(crate) fn resample_faked_rng<R: Resampler, const N: usize>(
    resampler: R,
    weights: Weights<N>,
    rng_vals: Vec<f32>,
) -> [usize; N] {
    let mut rng_iter = rng_vals.iter();
    let mut rngfn = move || {
        *rng_iter
            .next()
            .expect("Should not run out of fake RNG values")
    };

    resampler
        .resample(weights, &mut rngfn)
        .take(N)
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap()
}

pub(crate) fn bench<
    const N: usize,
    R: Resampler + Clone,
    D: DensityFunc,
    Range: rand::distr::uniform::SampleRange<f32> + Clone,
>(
    b: &mut test::Bencher,
    resampler: R,
    range: Range,
    density: &D,
) {
    use rand::Rng;
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

    let weights: Weights<N> = Weights::from_range_and_density(range, density, &mut rng)
        .expect("Density function should produce normal weights.");

    let mut rng_fn = move || rng.random::<f32>();

    b.iter(move || {
        resampler
            .clone()
            .resample(weights, &mut rng_fn)
            .take(N)
            .collect::<Vec<usize>>()
    });
}
