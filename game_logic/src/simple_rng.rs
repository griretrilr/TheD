use rand::Rng;

/// A small subset of the sys::rand::Rng trait that we use. So we can mock it easily.
pub trait SimpleRng {
    /// `Rng::gen_range<i32>`
    fn gen_range_i32(&mut self, low: i32, high: i32) -> i32;
}

impl<T> SimpleRng for T
where
    T: Rng,
{
    fn gen_range_i32(&mut self, low: i32, high: i32) -> i32 {
        self.gen_range(low, high)
    }
}
