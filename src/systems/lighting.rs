use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::components::{Boss, FollowLight};

pub fn setup_lighting(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ambient light - keep it low for dramatic lamp post effect
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.05, // Very low ambient light
    });

    // Create lamp posts positioned behind the player
    let lamp_post_positions = vec![
        Vec3::new(-5.0, 0.0, 3.0),  // Left lamp post behind player
        Vec3::new(5.0, 0.0, 3.0),   // Right lamp post behind player
        Vec3::new(0.0, 0.0, 4.0),   // Center lamp post behind player
    ];

    for (i, position) in lamp_post_positions.iter().enumerate() {
        // Create lamp post model
        let lamp_post_entity = commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cylinder {
                    radius: 0.1,
                    height: 4.0,
                    resolution: 8,
                    segments: 1,
                })),
                material: materials.add(Color::rgb(0.2, 0.2, 0.2).into()), // Dark gray post
                transform: Transform::from_translation(*position),
                ..default()
            },
            RigidBody::Fixed,
            Collider::cylinder(2.0, 0.1), // Collider for the post
            Name::new(format!("LampPost_{}", i)),
        )).id();

        // Add lamp shade on top
        let lamp_shade_entity = commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cylinder {
                    radius: 0.35,
                    height: 0.3,
                    resolution: 8,
                    segments: 1,
                })),
                material: materials.add(Color::rgb(0.1, 0.1, 0.1).into()), // Dark lamp shade
                transform: Transform::from_xyz(0.0, 2.2, 0.0),
                ..default()
            },
            Name::new(format!("LampShade_{}", i)),
        )).id();

        // Add the light source
        let light_entity = commands.spawn((
            PointLightBundle {
                point_light: PointLight {
                    intensity: 2000.0, // Higher intensity for lamp posts
                    shadows_enabled: true,
                    color: Color::rgb(1.0, 0.9, 0.7), // Warm yellow light
                    range: 20.0, // Good range for lamp posts
                    radius: 0.5,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 2.0, 0.0), // Position light at top of post
                ..default()
            },
            Name::new(format!("LampLight_{}", i)),
        )).id();

        // Make lamp shade and light children of the lamp post
        commands.entity(lamp_post_entity).push_children(&[lamp_shade_entity, light_entity]);
    }

    // Add a following light that moves with the player for better visibility
    let follow_light_offset = Vec3::new(0.0, 2.0, 2.0); // Behind and above the player

    commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                intensity: 500.0, // Lower intensity following light
                shadows_enabled: false, // Disable shadows for performance
                color: Color::rgb(0.8, 0.8, 1.0), // Cool blue tint
                range: 10.0,
                radius: 0.3,
                ..default()
            },
            transform: Transform::from_xyz(
                follow_light_offset.x,
                follow_light_offset.y,
                follow_light_offset.z,
            ),
            ..default()
        },
        FollowLight {
            offset: follow_light_offset,
        },
        Name::new("FollowingLight"),
    ));
}

pub fn update_light_position(
    boss_query: Query<&Transform, With<Boss>>,
    mut light_query: Query<(&mut Transform, &FollowLight), Without<Boss>>,
) {
    // Get the boss position
    let boss_transform = if let Ok(transform) = boss_query.get_single() {
        transform
    } else {
        return;
    };

    // Update light position
    for (mut light_transform, follow_light) in light_query.iter_mut() {
        light_transform.translation = boss_transform.translation + follow_light.offset;
    }
}