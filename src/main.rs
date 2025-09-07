use bevy::prelude::*;
mod components;
mod resources;
mod systems;

use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, tile_click_system)
        .add_systems(Update, check_match_system)
        .run();
}