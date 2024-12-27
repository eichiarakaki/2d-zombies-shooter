mod player;
mod window;

use bevy::prelude::*;
use player::{aim, move_bullet_system, player, player_movement, spawn_bullet_system};
use window::window_properties;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, player, window_properties))
        .add_systems(Update, (player_movement, aim))
        .add_systems(Update, (spawn_bullet_system, move_bullet_system))
        .run();
}

fn setup(mut commands: Commands) {
    // Spawning 2d camera
    commands.spawn(Camera2d::default());
}
