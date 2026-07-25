use std::time::SystemTime;

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

pub trait PropertyProvider {
    fn get_increment(&mut self) -> u8;

    fn get_time(&mut self) -> u16;
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_increment() {
        for _ in 0..256 {
            let mut provider = StandardPropertyProvider::default();

            let increment = provider.get_increment();

            assert_ne!(increment, 0);
            assert_ne!(increment >> 7, 1);
        }
    }
}
