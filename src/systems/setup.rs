use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::components::{Platform, FollowCamera};

pub fn setup_camera(mut commands: Commands) {
    // Camera - skybox will be added later by the skybox system
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
        (Vec3::new(-8.0, 2.0, 0.0), Vec3::new(4.0, 0.5, 3.0)),   // Small left platform
        (Vec3::new(-12.0, 4.5, 0.0), Vec3::new(3.0, 0.5, 2.5)),  // Higher left platform
        (Vec3::new(-6.0, 6.0, 0.0), Vec3::new(2.5, 0.5, 2.0)),   // Narrow high platform

        // Right side platforms
        (Vec3::new(8.0, 1.5, 0.0), Vec3::new(3.5, 0.5, 4.0)),    // Low right platform
        (Vec3::new(12.0, 3.5, 0.0), Vec3::new(2.0, 0.5, 2.0)),   // Small high right
        (Vec3::new(10.0, 5.5, 0.0), Vec3::new(3.0, 0.5, 2.5)),   // Medium high right

        // Center platforms for vertical progression
        (Vec3::new(0.0, 3.0, 0.0), Vec3::new(2.0, 0.5, 2.0)),    // Center low
        (Vec3::new(-2.0, 5.0, 0.0), Vec3::new(1.5, 0.5, 1.5)),   // Center-left high
        (Vec3::new(2.0, 7.0, 0.0), Vec3::new(1.5, 0.5, 1.5)),    // Center-right very high

        // Stepping stone platforms
        (Vec3::new(-4.0, 8.5, 0.0), Vec3::new(1.0, 0.5, 1.0)),   // Tiny stepping stone
        (Vec3::new(4.0, 8.5, 0.0), Vec3::new(1.0, 0.5, 1.0)),    // Another tiny stepping stone

        // Long bridge-like platforms
        (Vec3::new(0.0, 10.0, 0.0), Vec3::new(8.0, 0.5, 1.5)),   // High bridge
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