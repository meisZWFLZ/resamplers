//! Provides testing utilities for resamplers

#![cfg(test)]

use crate::{Weights, resamplers::Resampler};

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
