//! Shows how to modify mesh assets after spawning.
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(Update, update_materials)
        .run();
}

fn startup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    cmd.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(materials.add(Color::srgb(1., 0., 0.))),
        Transform::from_xyz(-2., 0., 0.),
    ));

    cmd.spawn((
        Mesh3d(meshes.add(Cuboid::from_length(1.))),
        MeshMaterial3d(materials.add(Color::srgb(0., 0., 1.))),
        Transform::from_xyz(2., 0., 0.),
    ));

    cmd.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 0., 10.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn update_materials(
    mut meshes: Query<&mut MeshMaterial3d<StandardMaterial>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        for mut mesh_mat in meshes.iter_mut() {
            mesh_mat.0 = materials.add(Color::srgb(0., 0., 1.));
        }
    }
}
