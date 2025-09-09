use bevy::{
    prelude::*,
    render::view::RenderLayers,
    window::{CompositeAlphaMode, WindowResolution},
};
use bevy_wayland::prelude::*;

use crate::{HomescreenCamera, HomescreenWindow};

pub fn setup(mut commands: Commands) {
    let width = 540.;
    let height = 576.;

    let window_ent = commands
        .spawn((
            Window {
                resolution: WindowResolution::new(width, height),
                composite_alpha_mode: CompositeAlphaMode::PreMultiplied,
                transparent: true,
                ..default()
            },
            LayerShellSettings {
                anchor: Anchor::empty(),
                layer: Layer::Bottom,
                exclusive_zone: 0,
                keyboard_interactivity: KeyboardInteractivity::OnDemand,
                ..default()
            },
            HomescreenWindow,
            InputRegion(Rect::new(0., 0., width, height)),
        ))
        .id();

    commands.spawn((
        Camera2d,
        RenderLayers::layer(1),
        HomescreenCamera,
        MeshPickingCamera,
        Camera {
            target: bevy::render::camera::RenderTarget::Window(bevy::window::WindowRef::Entity(
                window_ent,
            )),
            ..default()
        },
    ));
}

pub fn exit_on_esc(keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
}
