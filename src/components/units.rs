use std::fmt::Display;

use specs::prelude::*;
use specs_derive::Component;

use crate::utils::Point;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool,
}

impl Viewshed {
    pub fn new(range: i32) -> Self {
        Self {
            visible_tiles: Vec::new(),
            range,
            dirty: true,
        }
    }
}

#[derive(Component)]
pub struct Monster;

#[derive(Component)]
pub struct Name {
    pub name: String,
}

impl Name {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Component)]
pub struct CombatStats {
    pub max_hp: i32,
    pub current_hp: i32,
    pub defense: i32,
    pub attack: i32,
}

impl CombatStats {
    pub fn new(max_hp: i32, defense: i32, attack: i32) -> Self {
        Self {
            max_hp,
            current_hp: max_hp,
            defense,
            attack,
        }
    }
}
