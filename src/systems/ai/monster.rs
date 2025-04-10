use specs::prelude::*;

use crate::components::{Monster, Position, Viewshed};

pub struct MonsterAI;

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Monster>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (viewsheds, positions, monsters) = data;

        for (_viewshed, _position, _) in (&viewsheds, &positions, &monsters).join() {
            //println!("Monster considers their on existence.");
        }
    }
}
