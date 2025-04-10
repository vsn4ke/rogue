use std::{
    ops::Range,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Clone, Copy)]
pub struct Rng {
    seed: i32,
    state: i32,
}

impl Rng {
    pub fn new(seed: i32) -> Rng {
        Rng { seed, state: seed }
    }

    pub fn get_seed(self) -> i32 {
        self.seed
    }

    pub fn random_seed() -> Rng {
        let start = SystemTime::now();
        let seed = (start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backward")
            .as_millis() as i32)
            .abs();
        Rng::new(seed)
    }

    fn next_state(&mut self) -> i32 {
        //xorshift 32bits
        self.state ^= self.state << 13;
        self.state ^= self.state >> 17;
        self.state ^= self.state << 5;
        self.state
    }

    pub fn random_range(&mut self, range: Range<i32>) -> i32 {
        let modulo = range.end - range.start;
        self.next_state().abs() % modulo + range.start
    }
}
