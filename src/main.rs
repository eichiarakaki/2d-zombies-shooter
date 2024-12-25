use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Spawning 2d camera
    commands.spawn(Camera2d::default());

    // Load the sprite sheet
    let texture = asset_server.load("soldier/idle/idle01.png");

    // spawning the sprite
    commands.spawn(Sprite {
        image: texture.clone(),
        ..default()
    });
}
