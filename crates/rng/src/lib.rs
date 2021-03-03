use std::{fmt, ops::Range};

pub struct Rng {
    rand_32: oorandom::Rand32,
    rand_64: oorandom::Rand64,
}

impl Rng {
    pub fn new() -> Result<Self, Error> {
        let (seed_64, seed_128) = gen_seed()?;

        Ok(Self {
            rand_32: oorandom::Rand32::new(seed_64),
            rand_64: oorandom::Rand64::new(seed_128),
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

    pub fn gen_range_64(&mut self, range: Range<u64>) -> u64 {
        self.rand_64.rand_range(range)
    }

    pub fn gen_range_size(&mut self, range: Range<usize>) -> usize {
        self.rand_64
            .rand_range(range.start as u64..range.end as u64) as usize
    }

    pub fn gen_bool(&mut self, probability: f32) -> bool {
        assert!(probability >= 0.0);
        assert!(probability <= 1.0);

        self.rand_32.rand_float() < probability
    }
}

fn gen_seed() -> Result<(u64, u128), Error> {
    let mut seed_64 = [0; 8];
    getrandom::getrandom(&mut seed_64).map_err(|_| Error)?;

    let mut seed_128 = [0; 16];
    getrandom::getrandom(&mut seed_128).map_err(|_| Error)?;

    Ok((u64::from_le_bytes(seed_64), u128::from_le_bytes(seed_128)))
}

#[derive(Debug)]
pub struct Error;

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to get randomness from the OS")
    }
}
