pub mod finalized;
pub mod initial;
pub mod meta;

use crate::utils::Rng;

use super::Map;

pub struct MapBuilder {
    starter: Box<dyn InitialBuilder>,
    builders: Vec<Box<dyn MetaBuilder>>,
}

impl MapBuilder {
    pub fn new(starter: Box<dyn InitialBuilder>) -> Self {
        Self {
            starter,
            builders: Vec::new(),
        }
    }

    pub fn with(mut self, builder: Box<dyn MetaBuilder>) -> Self {
        self.builders.push(builder);
        self
    }

    pub fn build(self, rng: &mut Rng) -> Map {
        let mut map = Map::default();

        self.starter.draw(rng, &mut map);

        for i in 0..self.builders.len() {
            self.builders[i].draw(rng, &mut map);
        }

        map
    }
}

pub trait InitialBuilder {
    fn draw(&self, rng: &mut Rng, map: &mut Map);
}
pub trait MetaBuilder {
    fn draw(&self, rng: &mut Rng, map: &mut Map);
}
