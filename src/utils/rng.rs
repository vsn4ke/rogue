use std::{
    hash::{DefaultHasher, Hash, Hasher},
    ops::Range,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Clone, Copy)]
pub struct Rng {
    master_seed: u64,
    //state_32: i32,
    state_64: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Rng {
        Rng {
            master_seed: seed,
            state_64: seed,
        }
    }

    pub fn get_seed(self) -> u64 {
        self.master_seed
    }

    pub fn random_seed() -> Rng {
        let start = SystemTime::now();
        let seed = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backward")
            .as_millis() as u64;
        Rng::new(seed)
    }

    pub fn from_string(string: impl Into<String>) -> Self {
        let s: String = string.into();
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        Self::new(hasher.finish())
    }

    fn next_state_64(&mut self) -> u64 {
        self.state_64 ^= self.state_64 << 13;
        self.state_64 ^= self.state_64 >> 7;
        self.state_64 ^= self.state_64 << 17;
        self.state_64
    }

    pub fn random_range(&mut self, range: Range<i32>) -> i32 {
        self.random_range_from_seed(self.next_state_64(), range)
    }

    pub fn get_seed_from_xy(self, current_seed: u64, x: i32, y: i32) -> u64 {
        let x = x as u64;
        let y = y as u64;

        x * x * y + 2 * y * y * x + current_seed
    }

    pub fn random_range_from_seed(self, seed: u64, range: Range<i32>) -> i32 {
        let modulo = range.end - range.start;
        (seed as i32).abs() % modulo + range.start
    }
}
