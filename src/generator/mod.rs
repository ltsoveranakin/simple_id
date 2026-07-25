mod data_provider;
mod property_provider;

pub use crate::generator::data_provider::*;
pub use crate::generator::property_provider::*;
use crate::id::Id;
use rand::prelude::SmallRng;

pub type RngIdGenerator<R> = IdGenerator<RandomDataProvider<R>>;
pub type SmallRngIdGenerator = RngIdGenerator<SmallRng>;

pub struct IdGenerator<D, P = StandardPropertyProvider> {
    data_provider: D,
    property_provider: P,
}

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
