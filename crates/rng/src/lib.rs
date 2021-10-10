use std::ops::Range;

pub struct Rng {
    rand_32: oorandom::Rand32,
}

impl Rng {
    pub fn new() -> Result<Self, getrandom::Error> {
        let seed_64 = gen_seed()?;

        Ok(Self {
            rand_32: oorandom::Rand32::new(seed_64),
        })
    }

    pub fn gen_range(&mut self, range: Range<u32>) -> u32 {
        self.rand_32.rand_range(range)
    }

    pub fn gen_range_float(&mut self, range: Range<f32>) -> f32 {
        self.rand_32.rand_float() * (range.end - range.start) + range.start
    }

    pub fn gen_range_16(&mut self, range: Range<u16>) -> u16 {
        self.rand_32
            .rand_range(range.start as u32..range.end as u32) as u16
    }

    pub fn gen_bool(&mut self, probability: f32) -> bool {
        assert!(probability >= 0.0);
        assert!(probability <= 1.0);

        self.rand_32.rand_float() < probability
    }
}

fn gen_seed() -> Result<u64, getrandom::Error> {
    let mut seed_64 = [0; 8];
    getrandom::getrandom(&mut seed_64)?;

    Ok(u64::from_le_bytes(seed_64))
}
