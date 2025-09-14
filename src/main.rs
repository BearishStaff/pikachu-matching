use bevy::prelude::*;
use bevy::audio::AudioPlugin;
mod components;
mod resources;
mod systems;

use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AudioPlugin::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, tile_click_system)
        .add_systems(Update, check_match_system)
        .run();
}