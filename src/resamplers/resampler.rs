use crate::weights::Weights;

pub trait Resampler {
    /// Input weights must be normalized.
    /// Outputs copied indices.
    /// The iterator yields at least N samples.
    ///
    /// rng is a function that generates random numbers in [0, 1).
    fn resample<const N: usize, F: FnMut() -> f32>(
        self,
        weights: Weights<N>,
        rng: F,
    ) -> impl Iterator<Item = usize>;
}
