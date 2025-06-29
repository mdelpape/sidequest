use bevy::prelude::*;
use crate::components::{Boss, FollowLight};

pub fn setup_lighting(mut commands: Commands) {
    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.00, // Low intensity ambient light
    });

    // Directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Point light that will follow the boss
    let light_offset = Vec3::new(0.0, 3.0, 0.0); // Light positioned right above the boss

    commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                intensity: 1000.0, // Moderate intensity for focused lighting
                shadows_enabled: true,
                color: Color::rgb(0.9, 0.9, 1.0), // Slightly bluish tint for spooky effect
                range: 15.0, // Limited range for the light
                radius: 1.0, // Makes the light source slightly larger
                ..default()
            },
            transform: Transform::from_xyz(
                light_offset.x,
                light_offset.y,
                light_offset.z,
            ),
            ..default()
        },
        FollowLight {
            offset: light_offset,
        },
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