use bevy::prelude::*;
use bevy::pbr::DirectionalLightShadowMap;
use bevy_rapier3d::prelude::*;
use crate::components::{Player, FollowLight, FloorLight, FloorLightType};

pub fn setup_lighting(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Configure shadow settings for better quality
    commands.insert_resource(DirectionalLightShadowMap { size: 2048 });

    // Ambient light - keep it low for dramatic lamp post effect
    commands.insert_resource(AmbientLight {
        color: Color::rgb(0.4, 0.4, 0.6), // Cool nighttime ambient
        brightness: 0.1, // Very low ambient light
    });

    // Create detailed lamp posts positioned around the scene
    let lamp_post_positions = vec![
        Vec3::new(-15.0, 0.0, -1.0),
        Vec3::new(15.0, 0.0, -1.0),
        Vec3::new(2.0, 0.0, 2.0),
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

    // Create floor lights positioned around the scene
    let floor_light_positions = vec![
        (Vec3::new(-4.0, 0.05, 2.0), FloorLightType::Accent),
        (Vec3::new(4.0, 0.05, 2.0), FloorLightType::Accent),
        (Vec3::new(-4.0, 0.05, -2.0), FloorLightType::Accent),
        (Vec3::new(4.0, 0.05, -2.0), FloorLightType::Accent),
        (Vec3::new(-8.0, 0.05, 2.0), FloorLightType::Accent),
        (Vec3::new(8.0, 0.05, 2.0), FloorLightType::Accent),
        (Vec3::new(-8.0, 0.05, -2.0), FloorLightType::Accent),
        (Vec3::new(8.0, 0.05, -2.0), FloorLightType::Accent),

        // Final victory platform - celebration lighting
        (Vec3::new(40.0, 72.05, 2.0), FloorLightType::Accent),
        (Vec3::new(40.0, 72.05, -2.0), FloorLightType::Accent),

        // Checkpoint areas
        (Vec3::new(24.0, 6.30, 1.0), FloorLightType::Accent),
        (Vec3::new(24.0, 6.30, -1.0), FloorLightType::Accent),
        (Vec3::new(39.0, 23.30, 1.0), FloorLightType::Accent),
        (Vec3::new(39.0, 23.30, -1.0), FloorLightType::Accent),
        (Vec3::new(18.0, 32.30, 1.0), FloorLightType::Accent),
        (Vec3::new(18.0, 32.30, -1.0), FloorLightType::Accent)
    ];

    // Material for floor light housing
    let floor_light_material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.1, 0.1, 0.1),
        metallic: 0.9,
        perceptual_roughness: 0.1,
        reflectance: 0.8,
        emissive: Color::rgb(0.2, 0.2, 0.4),
        ..default()
    });

    for (i, (position, light_type)) in floor_light_positions.iter().enumerate() {
        create_floor_light(
            &mut commands,
            &mut meshes,
            floor_light_material.clone(),
            *position,
            light_type.clone(),
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

    // Add directional lighting for shadows and general illumination
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(0.7, 0.8, 1.0), // Slightly brighter moonlight
            illuminance: 2000.0, // Increased illuminance
            shadows_enabled: true,
            shadow_depth_bias: 0.02, // Reduce shadow acne
            shadow_normal_bias: 0.6, // Reduce shadow acne
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -std::f32::consts::FRAC_PI_3, // 60 degrees down
            std::f32::consts::FRAC_PI_6, // 30 degrees to the side
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

    // Main light source - point light
    let main_light = commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                intensity: 1200.0,
                shadows_enabled: true,
                color: Color::rgb(1.0, 0.9, 0.7), // Warm yellow
                range: 15.0,
                radius: 0.3,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 2.0, 0.0)), // At top of lamp
            ..default()
        },
        Name::new(format!("LampLight_{}", index)),
    )).id();

    // Make the housing and light children of the post
    commands.entity(post_entity).push_children(&[light_housing, main_light]);
}

fn create_floor_light(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    housing_material: Handle<StandardMaterial>,
    position: Vec3,
    light_type: FloorLightType,
    index: usize,
) {
    // Create the floor light housing (small cylinder embedded in ground)
    let housing_entity = commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cylinder {
                radius: 0.15,
                height: 0.1,
                resolution: 8,
                segments: 1,
            })),
            material: housing_material.clone(),
            transform: Transform::from_translation(position),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cylinder(0.05, 0.15),
        FloorLight {
            light_type: light_type.clone(),
            intensity: match &light_type {
                FloorLightType::Spotlight => 800.0,
                FloorLightType::Point => 600.0,
                FloorLightType::Accent => 300.0,
            },
        },
        Name::new(format!("FloorLight_{}", index)),
    )).id();

    // Create the appropriate light source based on type
    let light_entity = match light_type {
        FloorLightType::Spotlight => {
            commands.spawn((
                SpotLightBundle {
                    spot_light: SpotLight {
                        intensity: 800.0,
                        color: Color::rgb(1.0, 0.9, 0.7), // Warm white
                        shadows_enabled: true,
                        range: 12.0,
                        radius: 0.1,
                        inner_angle: 0.3,
                        outer_angle: 0.8,
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(0.0, 0.05, 0.0))
                        .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
                    ..default()
                },
                Name::new(format!("FloorSpotLight_{}", index)),
            )).id()
        },
        FloorLightType::Point => {
            commands.spawn((
                PointLightBundle {
                    point_light: PointLight {
                        intensity: 600.0,
                        color: Color::rgb(0.8, 0.9, 1.0), // Cool blue-white
                        shadows_enabled: false, // Disable shadows for performance
                        range: 8.0,
                        radius: 0.2,
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(0.0, 0.05, 0.0)),
                    ..default()
                },
                Name::new(format!("FloorPointLight_{}", index)),
            )).id()
        },
        FloorLightType::Accent => {
            commands.spawn((
                PointLightBundle {
                    point_light: PointLight {
                        intensity: 300.0,
                        color: Color::rgb(0.9, 0.7, 1.0), // Purple accent
                        shadows_enabled: false,
                        range: 4.0,
                        radius: 0.1,
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(0.0, 0.05, 0.0)),
                    ..default()
                },
                Name::new(format!("FloorAccentLight_{}", index)),
            )).id()
        },
    };

    // Make the light a child of the housing
    commands.entity(housing_entity).push_children(&[light_entity]);
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