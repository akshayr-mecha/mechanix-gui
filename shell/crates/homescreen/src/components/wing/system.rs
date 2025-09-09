use bevy::prelude::*;

use super::{Wing, WingMesh, WingMeshBuilder};

#[derive(Component)]
pub struct WingConfigured;
pub fn on_wing_added(
    mut commands: Commands,
    wings: Query<(Entity, &Wing), Without<WingConfigured>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, wing) in &wings {
        let transform = Transform::from_xyz(wing.position.x, wing.position.y, wing.z_index);
        let mut wing_mesh = WingMeshBuilder::create_empty();

        wing_mesh.border_radius(5.0);

        wing_mesh.height(wing.size.y);
        wing_mesh.width(wing.size.x);

        wing_mesh.upper_wing_height(if wing.upper_wing > 0.0 { 20.0 } else { 0.0 });
        wing_mesh.lower_wing_height(if wing.lower_wing > 0.0 { 20.0 } else { 0.0 });

        wing_mesh.upper_wing_width(wing.size.x.min(wing.upper_wing));
        wing_mesh.lower_wing_width(wing.size.x.min(wing.lower_wing));

        wing_mesh.mesh_handle(None);
        wing_mesh.vertices(wing.vertices.clone());

        let mut wing_mesh = wing_mesh.build().unwrap();

        let mesh_handle = meshes.add(wing_mesh.get_mesh());
        wing_mesh.mesh_handle = Some(mesh_handle.clone());

        let material = materials.add(ColorMaterial::from_color(wing.background_color));

        commands.entity(entity).insert((
            transform,
            wing_mesh,
            Mesh2d(mesh_handle),
            MeshMaterial2d(material),
            WingConfigured,
        ));
    }
}

pub fn on_wing_changed(
    mut commands: Commands,
    mut wings: Query<(Entity, &Wing, &mut WingMesh), (With<WingConfigured>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, wing, mut wing_mesh) in &mut wings {
        let prev_wing = wing_mesh.clone();

        wing_mesh.border_radius = 10.0;

        wing_mesh.height = wing.size.y;
        wing_mesh.width = wing.size.x;

        wing_mesh.upper_wing_height = if wing.upper_wing > 0.0 { 20.0 } else { 0.0 };
        wing_mesh.lower_wing_height = if wing.lower_wing > 0.0 { 20.0 } else { 0.0 };

        wing_mesh.upper_wing_width = wing.size.x.min(wing.upper_wing);
        wing_mesh.lower_wing_width = wing.size.x.min(wing.lower_wing);

        wing_mesh.vertices = wing.vertices.clone();

        let transform = Transform::from_xyz(wing.position.x, wing.position.y, wing.z_index);
        commands.entity(entity).insert((transform,));
        if prev_wing == *wing_mesh {
            continue;
        }

        let mesh = meshes
            .get_mut(
                wing_mesh
                    .mesh_handle
                    .as_ref()
                    .expect("Mesh Handle Not found"),
            )
            .expect("Mesh Not found");

        *mesh = wing_mesh.get_mesh();
    }
}
