use bevy::{
    math::IVec2,
    prelude::{Query, Window, WindowPosition},
};

pub fn window_properties(mut window: Query<&mut Window>) {
    let mut window = window.single_mut();

    window.title = "Zombies Shooter".into();
    window.position = WindowPosition::At(IVec2::new(10, 10)).into();
    window.resolution = (1000., 850.).into();
}
