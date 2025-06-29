use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::components::{Platform, FollowCamera};

pub fn setup_camera(mut commands: Commands) {
    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.0, 8.0) // Slightly higher for better view
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        },
        FollowCamera {
            offset: Vec3::new(0.0, 2.0, 8.0), // Match the initial position
            lerp_speed: 10.0, // Increased for smoother following
        },
    ));
}

pub fn setup_platform(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Main ground platform - extended length
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(50.0, 0.5, 5.0))), // Wider and deeper platform
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(0.0, -0.25, 0.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(25.0, 0.25, 2.5),
        Platform,
    ));

    // Create additional floating platforms
    let platform_configs = vec![
        // Left side platforms
        (Vec3::new(5.0, 1.0, 0.0), Vec3::new(5.0, 2.0, 5.0)),
    ];

    // Spawn all additional platforms
    for (position, size) in platform_configs {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(size.x, size.y, size.z))),
                material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                transform: Transform::from_translation(position),
                ..default()
            },
            RigidBody::Fixed,
            Collider::cuboid(size.x * 0.5, size.y * 0.5, size.z * 0.5),
            Platform,
        ));
    }
}