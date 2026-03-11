use crate::id::{Data, Id};
use rand::prelude::SmallRng;
use rand::{make_rng, Rng, RngExt, SeedableRng};
use std::time::SystemTime;

pub trait IdDataProvider {
    fn get_data(&mut self) -> Data;
}

pub struct RandomDataProvider<R> {
    rng: R,
}

impl<R> IdDataProvider for RandomDataProvider<R>
where
    R: Rng,
{
    fn get_data(&mut self) -> Data {
        self.rng.random()
    }
}

impl<R> RandomDataProvider<R>
where
    R: SeedableRng,
{
    fn new_seeded(seed: u64) -> Self {
        Self {
            rng: R::seed_from_u64(seed),
        }
    }

    fn new() -> Self {
        Self { rng: make_rng() }
    }
}

pub struct IdGenerator<R> {
    increment: u8,
    provider: R,
}

pub type RngIdGenerator<R> = IdGenerator<RandomDataProvider<R>>;
pub type SmallRngIdGenerator = RngIdGenerator<SmallRng>;

impl<DP> IdGenerator<DP>
where
    DP: IdDataProvider,
{
    pub fn new(provider: DP) -> Self {
        Self {
            increment: 0,
            provider,
        }
    }

    pub fn generate_new_id(&mut self) -> Id {
        const MAX_TIME: u128 = u16::MAX as u128;

        let header = 0b00000000;
        let increment = self.increment;
        let time = (SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            % MAX_TIME) as u16;
        let data = self.provider.random();

        self.increment += 1;
        self.increment = self.increment % 0x80;

        // Prevent creating a zero (null) id
        if self.increment == 0 {
            self.increment += 1;
        }

        Id {
            header,
            increment,
            time,
            data,
        }
    }
}
