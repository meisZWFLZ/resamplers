pub trait DensityFunc<T = f32> {
    fn eval(&self, x: T) -> f32;
}
