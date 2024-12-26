use bevy::{
    ecs::{
        component::Component,
        query::With,
        system::{Commands, Query},
    },
    prelude::*,
    transform,
};

#[derive(Component)]
pub struct MainPlayer {
    speed: f32,
    x: f32,
    y: f32,
}

#[derive(Component)]
pub struct PlayerEntity(Entity);

#[derive(Component)]
pub struct Bullet {
    speed: f32,
}

pub fn player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    //mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load the sprite sheet
    let texture = asset_server.load("soldier/idle/idle01.png");

    // spawning the sprite
    let entity = commands
        .spawn((
            Sprite {
                image: texture.clone(),
                ..default()
            },
            MainPlayer {
                speed: 200.,
                x: 0.,
                y: 0.,
            },
        ))
        .id();

    println!("{entity}");
    commands.spawn(PlayerEntity(entity));
}

pub fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut MainPlayer, &mut Transform)>,
) {
    let (mut player, mut transform) = query.single_mut();

    if keys.pressed(KeyCode::KeyW) {
        let p = transform.translation.y + player.speed * time.delta_secs();
        transform.translation.y = p;
        player.y = p;
    }
    if keys.pressed(KeyCode::KeyS) {
        let p = transform.translation.y - player.speed * time.delta_secs();
        player.y = p;
        transform.translation.y = p;
    }

    if keys.pressed(KeyCode::KeyD) {
        let p = transform.translation.x + player.speed * time.delta_secs();
        player.x = p;
        transform.translation.x = p;
    }
    if keys.pressed(KeyCode::KeyA) {
        let p = transform.translation.x - player.speed * time.delta_secs();
        player.x = p;
        transform.translation.x = p;
    }
}

pub fn spawn_bullet_system(
    buttons: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    player: Query<&MainPlayer>,
    asset_server: Res<AssetServer>,
) {
    let player = player.single();

    // Si se presiona el botón derecho del mouse, crear la bala
    if buttons.pressed(MouseButton::Left) {
        commands.spawn((
            Sprite {
                image: asset_server.load("soldier/bullet/bullet0.png"),
                custom_size: Some(Vec2::new(10., 10.)), // Cambiando en tamaño de la bala

                flip_x: false,
                flip_y: true,
                ..default()
            },
            Bullet { speed: 1000. },
            Transform::from_xyz(player.x, player.y, 0.),
            // Transform::default(), // Posición inicial por defecto
        ));
    }
}

pub fn move_bullet_system(
    mut query: Query<(&mut Transform, &Bullet), With<Bullet>>,
    timer: Res<Time>,
) {
    for (mut transform, bullet) in query.iter_mut() {
        transform.translation.x += bullet.speed * timer.delta_secs();
    }
}
