use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::ops::Range;

static RNG: Lazy<Mutex<Rng>> = Lazy::new(|| {
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time cannot be before unix epoch")
        .as_millis() as u64;

    Mutex::new(Rng {
        rand_32: oorandom::Rand32::new(seed),
    })
});

struct Rng {
    rand_32: oorandom::Rand32,
}

pub fn gen_range(range: Range<u32>) -> u32 {
    RNG.lock().rand_32.rand_range(range)
}

pub fn gen_range_float(range: Range<f32>) -> f32 {
    RNG.lock().rand_32.rand_float() * (range.end - range.start) + range.start
}

pub fn gen_range_16(range: Range<u16>) -> u16 {
    RNG.lock()
        .rand_32
        .rand_range(range.start as u32..range.end as u32) as u16
}

pub fn gen_bool(probability: f32) -> bool {
    assert!(probability >= 0.0);
    assert!(probability <= 1.0);

    RNG.lock().rand_32.rand_float() < probability
}
