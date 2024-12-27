use bevy::{
    ecs::{
        component::Component,
        query::With,
        system::{Commands, Query},
    },
    prelude::*,
    sprite::Anchor,
    text::cosmic_text::rustybuzz::Direction,
    transform,
    window::PrimaryWindow,
};

#[derive(Component)]
pub struct MainPlayer {
    speed: f32,
    x: f32,
    y: f32,
}

#[derive(Component)]
pub struct Bullet {
    speed: f32,
}

#[derive(Component)]
pub struct BulletDirection {
    pub vector: Vec2,
}

#[derive(Component)]
pub struct LifeTime {
    time_left: f32,
}

pub fn player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    //mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load the sprite sheet
    let texture = asset_server.load("soldier/idle/idle01.png");

    // spawning the sprite
    commands.spawn((
        Sprite {
            image: texture.clone(),
            ..default()
        },
        MainPlayer {
            speed: 200.,
            x: 0.,
            y: 0.,
        },
    ));
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
    player: Query<&Transform, With<MainPlayer>>,
    asset_server: Res<AssetServer>,
) {
    // Si se presiona el botón izquierdo del mouse, crear la bala
    if buttons.pressed(MouseButton::Left) {
        // Obtener la posición y rotación del jugador
        if let player_transform = player.single() {
            let player_pos = player_transform.translation;
            let player_rotation = player_transform.rotation;

            // Calcular la dirección en la que apunta el jugador usando su rotación
            let direction = Vec2::new(0.0, 1.0).rotate(player_rotation.to_angle()); // Dirección en la que apunta el jugador

            let speed = 1000.0; // Velocidad de la bala

            // Crear la bala en la posición del jugador
            commands.spawn((
                Sprite {
                    image: asset_server.load("soldier/bullet/bullet0.png"),
                    custom_size: Some(Vec2::new(10., 10.)), // Cambiar el tamaño de la bala
                    ..default()
                },
                Bullet { speed },
                LifeTime { time_left: 1.0 },
                Transform {
                    translation: player_pos,
                    rotation: player_rotation,
                    ..default()
                },
                BulletDirection { vector: direction },
            ));
        }
    }
}

pub fn aim(
    mut player: Query<&mut Transform, With<MainPlayer>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    for mut player_transform in &mut player {
        let (camera, camera_transform) = camera.single();
        let window = q_windows.single();

        if let Some(cursor_pos) = window
            .cursor_position()
            .and_then(|cursor| Some(camera.viewport_to_world_2d(camera_transform, cursor)))
        {
            let player_dir = player_transform.local_x().truncate(); // Obteniendo la direccion donde apunta el jugador. el truncate remueve el eje z.
            let cursor_dir = cursor_pos.unwrap() - player_transform.translation.truncate();
            let angle = player_dir.angle_to(cursor_dir);

            player_transform.rotate_z(angle);
        }
    }
}

pub fn move_bullet_system(
    mut query: Query<
        (
            Entity,
            &mut Transform,
            &Bullet,
            &mut LifeTime,
            &BulletDirection,
        ),
        With<Bullet>,
    >,
    mut commands: Commands,
    timer: Res<Time>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    if let Some(position) = q_windows.single().cursor_position() {
        println!("Cursor is inside the primary window, at {:?}", position);
        // Cada bala entra en este bucle
        for (entity, mut transform, bullet, mut life_time, direction) in query.iter_mut() {
            // Actualizar el tiempo de vida de la bala
            life_time.time_left -= timer.delta_secs();

            // Si la bala ha agotado su tiempo de vida, despawnea
            if life_time.time_left <= 0.0 {
                commands.entity(entity).despawn();
            }

            // Mover la bala en la dirección que está mirando el jugador
            transform.translation.x += direction.vector.x * bullet.speed * timer.delta_secs();
            transform.translation.y += direction.vector.y * bullet.speed * timer.delta_secs();
        }
    }
}
