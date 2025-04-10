use specs::prelude::*;

use crate::{
    components::{Monster, Position, Viewshed},
    utils::Point,
};

pub struct MonsterAI;

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        ReadExpect<'a, Point>,
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Monster>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_position, viewsheds, positions, monsters) = data;

        for (viewshed, _position, _) in (&viewsheds, &positions, &monsters).join() {
            if viewshed.visible_tiles.contains(&player_position) {
                println!("Monster considers their on existence.");
            }
        }
    }
}
