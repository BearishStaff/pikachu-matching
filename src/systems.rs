use bevy::audio::*;
use bevy::prelude::*;

use crate::components::Tile;
use crate::resources::Board;
use crate::resources::Selection;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let rows = 6;
    let cols = 8;
    let tile_size = 64.0;
    let board_width = cols as f32 * tile_size;
    let board_height = rows as f32 * tile_size;

    let pokemon_names = [
        "raichu",
        "bulbasaur",
        "charmander",
        "beedrill",
        "clefairy",
        "flareon",
        "gligar",
        "growlithe",
        "psyduck",
        "mareep",
        "mew",
        "omanyte",
        "rapidash",
        "vaporeon",
        "wartortle",
    ];

    let mut grid = vec![vec![None; cols]; rows];

    for row in 0..rows {
        for col in 0..cols {
            let x = col as f32 * tile_size - board_width / 2.0 + tile_size / 2.0;
            let y = row as f32 * tile_size - board_height / 2.0 + tile_size / 2.0;

            let pokemon_index = (row * cols + col) % pokemon_names.len();
            let pokemon_name = pokemon_names[pokemon_index];
            grid[row][col] = Some(pokemon_name.to_string());

            commands
                .spawn(SpriteBundle {
                    texture: asset_server.load(&format!("pokemons/{}.png", pokemon_name)),
                    transform: Transform::from_xyz(x, y, 0.0),
                    ..default()
                })
                .insert(Tile {
                    revealed: true,
                    id: (row * cols + col) as u32,
                    grid_pos: (row, col),
                    name: pokemon_name.to_string(),
                });
        }
    }

    commands.insert_resource(Board { grid, rows, cols });
    commands.insert_resource(Selection::default());
}

pub fn tile_click_system(
    mut commands: Commands,
    windows: Query<&mut Window>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut selection: ResMut<Selection>,
    mut query: Query<(Entity, &mut Tile, &Transform, &Sprite)>,
    asset_server: Res<AssetServer>,
) {
    let window = windows.get_single().unwrap();

    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(cursor_pos) = window.cursor_position() {
            commands.spawn(AudioBundle {
                            source: asset_server.load("sounds/select.ogg"),
                            settings: PlaybackSettings::ONCE,
                        });
            for (entity, mut tile, transform, sprite) in query.iter_mut() {
                let sprite_size = sprite.custom_size.unwrap_or(Vec2::new(64.0, 64.0));
                let min_x = transform.translation.x - sprite_size.x / 2.0;
                let max_x = transform.translation.x + sprite_size.x / 2.0;
                let min_y = transform.translation.y - sprite_size.y / 2.0;
                let max_y = transform.translation.y + sprite_size.y / 2.0;

                if cursor_pos.x >= min_x
                    && cursor_pos.x <= max_x
                    && cursor_pos.y >= min_y
                    && cursor_pos.y <= max_y
                {
                    if !tile.revealed {
                        tile.revealed = true;

                        // update selection
                        if selection.first.is_none() {
                            selection.first = Some(entity);
                        } else if selection.second.is_none() {
                            selection.second = Some(entity);
                        }
                    }
                }
            }
        }
    }
}

pub fn check_match_system(
    _commands: Commands,
    mut selection: ResMut<Selection>,
    query: Query<&Tile>,
) {
    if let (Some(first), Some(second)) = (selection.first, selection.second) {
        let first_tile = query.get(first).unwrap();
        let second_tile = query.get(second).unwrap();

        if first_tile.id == second_tile.id {
            // ✅ Tiles match — keep revealed
            println!("Matched: {}!", first_tile.name);
        } else {
            // ❌ Tiles do not match — hide again
            // You might want a timer to hide after a delay
            println!("No match: {} vs {}", first_tile.name, second_tile.name);
        }

        // reset selection
        selection.first = None;
        selection.second = None;
    }
}
