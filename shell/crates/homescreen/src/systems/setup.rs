use bevy::{
    prelude::*,
    render::view::RenderLayers,
    window::{CompositeAlphaMode, WindowResolution},
};
use bevy_wayland::prelude::*;

use crate::{components::Wing, HomescreenCamera, HomescreenData, HomescreenWidget, WidgetId};
pub fn setup_widgets(mut homescreen_data: ResMut<HomescreenData>) {
    let widgets_to_place = vec![
        (
            HomescreenWidget {
                id: WidgetId(0),
                width: 1,
                height: 1,
            },
            Vec2::new(0.0, 0.0),
        ),
        (
            HomescreenWidget {
                id: WidgetId(4),
                width: 2,
                height: 1,
            },
            Vec2::new(0.0, 0.0),
        ),
        (
            HomescreenWidget {
                id: WidgetId(5),
                width: 1,
                height: 1,
            },
            Vec2::new(0.0, 0.0),
        ),
        (
            HomescreenWidget {
                id: WidgetId(1),
                width: 1,
                height: 1,
            },
            Vec2::new(250.0, 0.0),
        ),
        (
            HomescreenWidget {
                id: WidgetId(2),
                width: 1,
                height: 2,
            },
            Vec2::new(0.0, 250.0),
        ),
        (
            HomescreenWidget {
                id: WidgetId(3),
                width: 2,
                height: 1,
            },
            Vec2::new(100.0, 400.0),
        ),
    ];

    for (widget, location) in widgets_to_place {
        homescreen_data.place(widget, location, 0);
    }
}
#[derive(Component)]
pub struct RenderedWidget;
pub fn render_widgets(
    mut commands: Commands,
    homescreen_data: Res<HomescreenData>,
    rendered_widget_query: Query<Entity, With<RenderedWidget>>,
    render_layer: Single<&RenderLayers, With<HomescreenCamera>>,
) {
    if rendered_widget_query.iter().len() > 0 {
        return;
    }
    let colors = [
        Color::srgb(0.0, 0.0, 1.0),   // BLUE
        Color::srgb(1.0, 0.0, 0.0),   // RED
        Color::srgb(0.0, 1.0, 0.0),   // GREEN
        Color::srgb(1.0, 1.0, 0.0),   // YELLOW
        Color::srgb(0.5, 0.0, 0.5),   // PURPLE
        Color::srgb(1.0, 0.647, 0.0), // ORANGE
    ];

    // For simplicity, we only render the active screen.
    let page_to_render = homescreen_data.active_screen;
    let render_list = homescreen_data.get_page_render_data(page_to_render);

    for render_data in render_list {
        println!("{:?}", render_data);
        let color = colors[render_data.data.widget.id.0 % colors.len()];
        let mut centered_location = render_data.screen_location;
        centered_location.x = -centered_location.x;
        commands.spawn((
            Wing {
                position: centered_location,
                size: render_data.screen_size.xy(),
                z_index: 0.0,
                background_color: color,
                border_color: color,
                upper_wing: false,
                lower_wing: false,
            },
            render_layer.clone(),
            RenderedWidget,
        ));
    }
}

// pub fn setup_widgets(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     render_layer: Single<&RenderLayers, With<HomescreenCamera>>,
// ) {
// let mut wing_mesh = WingMesh {
//     width: 520.0,
//     height: 2.0 * 560.0 / 4.0,
//     upper_wing_height: 10.0,
//     upper_wing_width: 250.0,

//     lower_wing_height: 0.0,
//     lower_wing_width: 0.0,

//     border_radius: 5.0,
// };
// let default_material = materials.add(ColorMaterial::from_color(Color::srgb(0.25, 0.25, 0.25)));
// let hover_material = materials.add(ColorMaterial::from_color(Color::srgb(0.65, 0.15, 0.25)));
// let wing = Wing {
//     position: Vec2::ZERO,
//     size: Vec2::new(250.0, 250.0),
//     z_index: 0.0,
//     background_color: Color::WHITE,
//     border_color: Color::NONE,
//     upper_wing: false,
//     lower_wing: false,
// };
// commands
//     .spawn((wing, render_layer.clone()))
//     .observe(update_material_on::<Pointer<Out>>(Wing {
//         upper_wing: false,
//         lower_wing: false,
//         ..wing
//     }))
//     .observe(update_material_on::<Pointer<Over>>(Wing {
//         upper_wing: true,
//         lower_wing: false,
//         ..wing
//     }))
//     .observe(update_material_on::<Pointer<Released>>(Wing {
//         upper_wing: true,
//         lower_wing: false,
//         ..wing
//     }))
//     .observe(update_material_on::<Pointer<Pressed>>(Wing {
//         upper_wing: true,
//         lower_wing: true,
//         ..wing
//     }));

// wing_mesh.width += 4.0;
// wing_mesh.height += 4.0;
// wing_mesh.upper_wing_width += 3.0;
// wing_mesh.border_radius += 2.0;
// commands.spawn((
//     Mesh2d(meshes.add(wing_mesh.get_mesh())),
//     MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.35, 0.35, 0.35)))),
//     Transform::from_xyz(0.0, -140.0, 0.0),
//     wing_mesh,
//     render_layer.clone(),
// ));

// let mut wing_mesh = WingMesh {
//     width: 280.0,
//     height: 1.75 * 560.0 / 4.0,
//
//     lower_wing_height: 20.0,
//     lower_wing_width: 260.0,
//
//     upper_wing_height: 0.0,
//     upper_wing_width: 0.0,
//
//     border_radius: 5.0,
// };
//
// commands.spawn((
//     Mesh2d(meshes.add(wing_mesh.get_mesh())),
//     MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb_u8(255, 191, 0)))),
//     Transform::from_xyz(470.0 / 4.0, 155.0, 1.0),
//     wing_mesh.clone(),
//     render_layer.clone(),
// ));
//
// wing_mesh.width += 4.0;
// wing_mesh.height += 4.0;
// wing_mesh.lower_wing_width += 3.0;
// wing_mesh.border_radius += 2.0;
// commands.spawn((
//     Mesh2d(meshes.add(wing_mesh.get_mesh())),
//     MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.51, 0.51, 0.51)))),
//     Transform::from_xyz(470.0 / 4.0, 155.0, 0.0),
//     wing_mesh.clone(),
//     render_layer.clone(),
// ));
// }

pub fn update_material_on<E>(new_wing: Wing) -> impl Fn(Trigger<E>, Query<&mut Wing>) {
    // An observer closure that captures `new_material`. We do this to avoid needing to write four
    // versions of this observer, each triggered by a different event and with a different hardcoded
    // material. Instead, the event type is a generic, and the material is passed in.
    move |trigger, mut query| {
        if let Ok(mut wing) = query.get_mut(trigger.target()) {
            *wing = new_wing.clone();
        }
    }
}

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
