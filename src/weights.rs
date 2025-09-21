/// Array of normalized finite weights, meaning they sum to 1.
/// Also has some utility methods.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Weights<const N: usize>([f32; N]);

impl<const N: usize> Weights<N> {
    pub fn try_new(weights: [f32; N]) -> Option<Self> {
        if weights.iter().all(|w: &f32| w.is_finite())
            && weights.iter().sum::<f32>() - 1.0f32 <= f32::EPSILON
        {
            Some(Self(weights))
        } else {
            None
        }
    }

    /// Takes an unnormalized weights and returns normalized weights.
    pub fn normalize(bad_weights: [f32; N]) -> Option<Self> {
        if !bad_weights.iter().all(|w: &f32| w.is_finite()) {
            return None;
        }
        let sum: f32 = bad_weights.iter().sum();
        let weights: [f32; N] = bad_weights.map(|w| w / sum);
        Some(Self(weights))
    }

    pub unsafe fn unsafe_new(weights: [f32; N]) -> Self {
        Self(weights)
    }

    pub fn as_array(&self) -> &[f32; N] {
        &self.0
    }

    pub fn cum_sum(&self) -> [f32; N] {
        self.as_array()
            .into_iter()
            .scan(0.0f32, |prev_sum, x| {
                *prev_sum += x;
                Some(*prev_sum)
            })
            .collect::<Vec<f32>>()
            .try_into()
            .unwrap()
    }

    pub fn from_samples_and_density<
        T,
        Xs: Iterator<Item = T>,
        D: crate::densities::DensityFunc<T>,
    >(
        xs: Xs,
        density: &D,
    ) -> Option<Self> {
        let weights = xs
            .map(|x| density.eval(x))
            .take(N)
            .collect::<Vec<f32>>()
            .try_into()
            .ok()?;

        Self::normalize(weights)
    }

    pub fn from_range_and_density<
        Range: rand::distr::uniform::SampleRange<f32> + Clone,
        D: crate::densities::DensityFunc<f32>,
        R: rand::Rng,
    >(
        range: Range,
        density: &D,
        rng: &mut R,
    ) -> Option<Self> {
        let xs = std::iter::from_fn(move || Some(rng.random_range(range.clone())));
        Self::from_samples_and_density(xs, density)
    }
}

impl<const N: usize> From<Weights<N>> for [f32; N] {
    fn from(norm: Weights<N>) -> Self {
        norm.0
    }
}
