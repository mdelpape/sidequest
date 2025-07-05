use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::components::{Player, FollowLight};

pub fn setup_lighting(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ambient light - keep it low for dramatic lamp post effect
    commands.insert_resource(AmbientLight {
        color: Color::rgb(0.4, 0.4, 0.6), // Cool nighttime ambient
        brightness: 0.02, // Very low ambient light
    });

    // Create detailed lamp posts positioned around the scene
    let lamp_post_positions = vec![
        // Vec3::new(-8.0, 0.0, 2.0),   // Left lamp post
        // Vec3::new(8.0, 0.0, 2.0),    // Right lamp post
        Vec3::new(-15.0, 0.0, -1.0), // Far left lamp post
        Vec3::new(15.0, 0.0, -1.0),  // Far right lamp post
        Vec3::new(0.0, 0.0, 5.0),    // Center back lamp post
    ];

    // Materials for different parts
    let post_material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.15, 0.15, 0.15),
        metallic: 0.8,
        perceptual_roughness: 0.3,
        reflectance: 0.1,
        ..default()
    });

    let ornate_material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.2, 0.15, 0.1),
        metallic: 0.9,
        perceptual_roughness: 0.2,
        reflectance: 0.3,
        ..default()
    });

    let glass_material = materials.add(StandardMaterial {
        base_color: Color::rgba(1.0, 0.95, 0.8, 0.3),
        alpha_mode: AlphaMode::Blend,
        metallic: 0.0,
        perceptual_roughness: 0.1,
        reflectance: 0.9,
        ..default()
    });

    for (i, position) in lamp_post_positions.iter().enumerate() {
        create_detailed_lamp_post(
            &mut commands,
            &mut meshes,
            post_material.clone(),
            ornate_material.clone(),
            glass_material.clone(),
            *position,
            i,
        );
    }

    // Add a following light that moves with the player for better visibility
    let follow_light_offset = Vec3::new(0.0, 3.0, 3.0); // Behind and above the player

    commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                intensity: 500.0,
                shadows_enabled: false, // Disable shadows for performance
                color: Color::rgb(0.9, 0.9, 1.0), // Cool blue tint
                range: 12.0,
                radius: 0.4,
                ..default()
            },
            transform: Transform::from_translation(follow_light_offset),
            ..default()
        },
        FollowLight {
            offset: follow_light_offset,
        },
        Name::new("FollowingLight"),
    ));

    // Add some directional lighting for better overall scene lighting
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(0.6, 0.7, 1.0),
            illuminance: 1000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -std::f32::consts::FRAC_PI_4,
            std::f32::consts::FRAC_PI_4,
            0.0,
        )),
        ..default()
    });
}

fn create_detailed_lamp_post(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    post_material: Handle<StandardMaterial>,
    ornate_material: Handle<StandardMaterial>,
    _glass_material: Handle<StandardMaterial>,
    position: Vec3,
    index: usize,
) {
    // Main post cylinder
    let post_entity = commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cylinder {
                radius: 0.1,
                height: 4.0,
                resolution: 12,
                segments: 1,
            })),
            material: post_material.clone(),
            transform: Transform::from_translation(position + Vec3::new(0.0, 2.0, 0.0)),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cylinder(2.0, 0.1),
        Name::new(format!("LampPost_{}", index)),
    )).id();

    // Top light housing (shorter, wider cylinder)
    let light_housing = commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cylinder {
                radius: 0.3,
                height: 0.2,
                resolution: 12,
                segments: 1,
            })),
            material: ornate_material.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 2.0, 0.0)),
            ..default()
        },
        Name::new(format!("LampHousing_{}", index)),
    )).id();

    // Main light source
    let main_light = commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                intensity: 1000.0,
                shadows_enabled: true,
                color: Color::rgb(1.0, 0.9, 0.7), // Warm yellow
                range: 20.0,
                radius: 0.3,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 4.3, 0.0)),
            ..default()
        },
        Name::new(format!("LampLight_{}", index)),
    )).id();

    // Make the housing and light children of the post
    commands.entity(post_entity).push_children(&[light_housing, main_light]);
}

pub fn update_light_position(
    player_query: Query<&Transform, With<Player>>,
    mut light_query: Query<(&mut Transform, &FollowLight), Without<Player>>,
) {
    // Get the player position
    let player_transform = if let Ok(transform) = player_query.get_single() {
        transform
    } else {
        return;
    };

    // Update light position
    for (mut light_transform, follow_light) in light_query.iter_mut() {
        light_transform.translation = player_transform.translation + follow_light.offset;
    }
}