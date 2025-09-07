use bevy::prelude::*;

#[allow(dead_code)]
#[derive(Component)]
pub struct Tile {
    pub id: u32,
    pub grid_pos: (usize, usize),
    pub name: String,
    pub revealed: bool,
}