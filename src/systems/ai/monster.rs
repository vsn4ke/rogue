use specs::prelude::*;

use crate::{
    components::{Monster, Name, Position, Viewshed},
    utils::Point,
};

pub struct MonsterAI;

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        ReadExpect<'a, Point>,
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_position, viewsheds, positions, monsters, name) = data;

        for (viewshed, _position, _, name) in (&viewsheds, &positions, &monsters, &name).join() {
            if viewshed.visible_tiles.contains(&player_position) {
                // todo log instead of println
                println!("{name} shouts insults.");
            }
        }
    }
}
