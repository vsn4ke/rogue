use crate::{maps::InitialBuilder, utils::Rng};

pub struct Empty;

impl Empty {
    pub fn new() -> Box<Self> {
        Box::new(Self)
    }
}

impl InitialBuilder for Empty {
    fn draw(&self, _rng: &mut Rng, _map: &mut crate::maps::Map) {
        // do nothing
    }
}
