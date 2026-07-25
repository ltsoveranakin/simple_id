use crate::id::Data;
use rand::{Rng, RngExt, SeedableRng, make_rng};

pub struct RandomDataProvider<R> {
    pub rng: R,
}

impl<R> IdDataProvider for RandomDataProvider<R>
where
    R: Rng,
{
    fn get_data(&mut self) -> Data {
        self.rng.random()
    }
}

impl<R> Default for RandomDataProvider<R>
where
    R: SeedableRng,
{
    fn default() -> Self {
        Self { rng: make_rng() }
    }
}

pub trait IdDataProvider {
    fn get_data(&mut self) -> Data;
}
