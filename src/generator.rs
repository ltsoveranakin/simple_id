use crate::id::{Data, Id};
use rand::prelude::SmallRng;
use rand::{Rng, RngExt, SeedableRng, make_rng};
use std::time::SystemTime;

pub trait IdDataProvider {
    fn get_data(&mut self) -> Data;
}

pub trait PropertyProvider {
    fn get_increment(&mut self) -> u8;

    fn get_time(&mut self) -> u16;
}

#[derive(Default)]
pub struct StandardPropertyProvider {
    increment: u8,
}

impl PropertyProvider for StandardPropertyProvider {
    fn get_increment(&mut self) -> u8 {
        if self.increment == 0 {
            self.increment += 1;
        }

        let increment = self.increment;

        self.increment += 1;
        self.increment = self.increment % 0x80;

        increment
    }

    fn get_time(&mut self) -> u16 {
        const MAX_TIME: u128 = u16::MAX as u128;

        let time = (SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            % MAX_TIME) as u16;

        time
    }
}

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

pub struct IdGenerator<D, P = StandardPropertyProvider> {
    data_provider: D,
    property_provider: P,
}

pub type RngIdGenerator<R> = IdGenerator<RandomDataProvider<R>>;
pub type SmallRngIdGenerator = RngIdGenerator<SmallRng>;

impl<D, P> IdGenerator<D, P>
where
    D: IdDataProvider,
    P: PropertyProvider,
{
    pub fn new(data_provider: D, property_provider: P) -> Self {
        Self {
            data_provider,
            property_provider,
        }
    }

    pub fn generate_new_id(&mut self) -> Id {
        let header = 0b00000000;
        let increment = self.property_provider.get_increment();
        let time = self.property_provider.get_time();
        let data = self.data_provider.get_data();

        Id {
            header,
            increment,
            time,
            data,
        }
    }
}

impl<D, P> Default for IdGenerator<D, P>
where
    D: Default + IdDataProvider,
    P: Default + PropertyProvider,
{
    fn default() -> Self {
        Self::new(D::default(), P::default())
    }
}
