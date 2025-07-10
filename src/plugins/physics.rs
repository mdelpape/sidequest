use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_mod_rounded_box::{RoundedBox, BoxMeshOptions};
use crate::{
    components::*,
    states::*,
    events::*,
};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(PlayState::Setup), (
                setup_platforms,
                setup_physics_world,
                setup_coins_immediately,
            ))
            .add_systems(Update, (
                handle_platform_interactions,
                handle_trampoline_collisions,
                handle_trampoline_proximity,
                update_physics_debug,
                handle_coin_collection,
                animate_coins,
                setup_coins_delayed,
                trigger_trampoline_animation,
                update_trampoline_animation,
            ).run_if(in_state(GameState::Playing)));
    }
}

fn setup_platforms(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Main ground platform
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(RoundedBox {
                size: Vec3::new(50.0, 0.5, 5.0),
                radius: 0.2,
                subdivisions: 8,
                options: BoxMeshOptions::DEFAULT,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.5, 0.5, 0.5),
                perceptual_roughness: 0.3,
                metallic: 0.1,
                reflectance: 0.8,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, -0.25, 0.0),
            ..default()
        },
        RigidBody::Fixed,
        // No collider on the main entity
        Platform {
            platform_type: PlatformType::Ground,
            is_active: true,
            has_coin: false,
            has_lights: true, // Ground platform should have lights for testing
        },
        Name::new("GroundPlatform"),
    ))
    .with_children(|parent| {
        // Top surface with friction for walking
        parent.spawn((
            TransformBundle::from_transform(Transform::from_xyz(0.0, 0.2, 0.0)),
            Collider::round_cuboid(25.0, 0.05, 2.5, 0.05),
            Friction {
                coefficient: 0.8,
                combine_rule: CoefficientCombineRule::Max,
            },
            Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
        ));

        // Side/bottom collider without friction
        parent.spawn((
            TransformBundle::from_transform(Transform::from_xyz(0.0, -0.1, 0.0)),
            Collider::round_cuboid(25.0, 0.15, 2.5, 0.1),
            Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
        ));

        // Add point light for ground platform
        parent.spawn((
            PointLightBundle {
                point_light: PointLight {
                    intensity: 1000.0,
                    color: Color::rgb(0.4, 0.8, 1.0),
                    shadows_enabled: true,
                    range: 30.0, // Large range for ground platform
                    radius: 2.0,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 1.0, 0.0), // Position above ground
                ..default()
            },
            Name::new("GroundPlatformLight"),
        ));
    });

    // Add trampoline platform near starting position for testing
    let trampoline_transform = Transform::from_xyz(6.0, 1.0, 0.0);
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(RoundedBox {
                size: Vec3::new(3.0, 0.8, 3.0),
                radius: 0.3,
                subdivisions: 8,
                options: BoxMeshOptions::DEFAULT,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.2, 0.8, 0.2), // Bright green for trampoline
                perceptual_roughness: 0.3,
                metallic: 0.1,
                reflectance: 0.8,
                emissive: Color::rgb(0.5, 2.0, 0.5), // Much stronger green glow for trampoline
                ..default()
            }),
            transform: trampoline_transform,
            ..default()
        },
        RigidBody::Fixed,
        Platform {
            platform_type: PlatformType::Trampoline,
            is_active: true,
            has_coin: false,
            has_lights: true, // Trampolines have lights
        },
        TrampolineAnimation {
            original_transform: trampoline_transform,
            ..default()
        },
        Name::new("TrampolinePlatform"),
    ))
    .with_children(|parent| {
        // Top surface with bouncy properties
        parent.spawn((
            TransformBundle::from_transform(Transform::from_xyz(0.0, 0.3, 0.0)),
            Collider::round_cuboid(1.5, 0.1, 1.5, 0.1),
            Friction {
                coefficient: 0.9,
                combine_rule: CoefficientCombineRule::Max,
            },
            Restitution {
                coefficient: 1.2, // Super bouncy!
                combine_rule: CoefficientCombineRule::Max,
            },
            TrampolineTopSurface, // Mark this as the bouncy surface
        ));

        // Side/bottom collider
        parent.spawn((
            TransformBundle::from_transform(Transform::from_xyz(0.0, -0.2, 0.0)),
            Collider::round_cuboid(1.5, 0.3, 1.5, 0.15),
            Friction {
                coefficient: 0.2,
                combine_rule: CoefficientCombineRule::Min,
            },
            Restitution {
                coefficient: 0.3,
                combine_rule: CoefficientCombineRule::Min,
            },
        ));

        // Add point light for trampoline
        parent.spawn((
            PointLightBundle {
                point_light: PointLight {
                    intensity: 800.0,
                    color: Color::rgb(0.2, 1.0, 0.2), // Bright green light
                    shadows_enabled: true,
                    range: 12.0, // Good range for trampoline
                    radius: 1.0,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 1.0, 0.0), // Position above trampoline
                ..default()
            },
            Name::new("TrampolinePlatformLight"),
        ));
    });

    let platform_configs = vec![
        // === SECTION 1: TUTORIAL JUMPS (Easy) ===
        // Simple progression to teach basic jumping
        (Vec3::new(12.0, 2.5, 0.0), Vec3::new(4.0, 0.5, 4.0), PlatformType::Floating),
        (Vec3::new(18.0, 4.0, 0.0), Vec3::new(4.0, 0.5, 4.0), PlatformType::Floating),
        (Vec3::new(24.0, 6.0, 0.0), Vec3::new(4.0, 0.5, 4.0), PlatformType::Floating),

        // === SECTION 2: FIRST CHALLENGE (Medium) ===
        // Smaller platforms, requires precision
        (Vec3::new(30.0, 8.5, 0.0), Vec3::new(3.0, 0.5, 3.0), PlatformType::Small),
        (Vec3::new(35.0, 10.0, 0.0), Vec3::new(2.5, 0.5, 2.5), PlatformType::Small),

        // First gap that requires a running jump
        (Vec3::new(42.0, 12.0, 0.0), Vec3::new(3.0, 0.5, 3.0), PlatformType::Floating),

        // === SECTION 3: BRANCHING PATHS (Medium-Hard) ===
        // Left path - More platforms, easier but longer
        (Vec3::new(36.0, 14.0, 0.0), Vec3::new(2.5, 0.5, 2.5), PlatformType::Small),
        (Vec3::new(32.0, 16.0, 0.0), Vec3::new(2.5, 0.5, 2.5), PlatformType::Small),
        (Vec3::new(28.0, 18.0, 0.0), Vec3::new(2.5, 0.5, 2.5), PlatformType::Small),
        (Vec3::new(32.0, 20.0, 0.0), Vec3::new(3.0, 0.5, 3.0), PlatformType::Floating),

        // Right path - Fewer platforms, harder but shorter
        (Vec3::new(46.0, 15.0, 0.0), Vec3::new(2.0, 0.5, 2.0), PlatformType::SteppingStone),
        (Vec3::new(50.0, 18.0, 0.0), Vec3::new(2.0, 0.5, 2.0), PlatformType::SteppingStone),
        (Vec3::new(46.0, 21.0, 0.0), Vec3::new(3.0, 0.5, 3.0), PlatformType::Floating),

        // Convergence point
        (Vec3::new(39.0, 23.0, 0.0), Vec3::new(4.0, 0.5, 4.0), PlatformType::Bridge),

        // === SECTION 4: PRECISION CHALLENGE (Hard) ===
        // Stepping stones that require precise timing
        (Vec3::new(35.0, 25.5, 0.0), Vec3::new(1.8, 0.5, 1.8), PlatformType::SteppingStone),
        (Vec3::new(31.0, 27.0, 0.0), Vec3::new(1.8, 0.5, 1.8), PlatformType::SteppingStone),
        (Vec3::new(27.0, 28.5, 0.0), Vec3::new(1.8, 0.5, 1.8), PlatformType::SteppingStone),
        (Vec3::new(23.0, 30.0, 0.0), Vec3::new(1.8, 0.5, 1.8), PlatformType::SteppingStone),

        // Safe platform after challenge
        (Vec3::new(18.0, 32.0, 0.0), Vec3::new(4.0, 0.5, 4.0), PlatformType::Floating),

        // === SECTION 5: VERTICAL WALL CLIMB (Hard) ===
        // Alternating platforms that require wall-jump-like movement
        (Vec3::new(12.0, 34.0, 0.0), Vec3::new(2.5, 0.5, 2.5), PlatformType::Small),
        (Vec3::new(16.0, 36.0, 0.0), Vec3::new(2.5, 0.5, 2.5), PlatformType::Small),
        (Vec3::new(10.0, 38.0, 0.0), Vec3::new(2.5, 0.5, 2.5), PlatformType::Small),
        (Vec3::new(14.0, 40.0, 0.0), Vec3::new(2.5, 0.5, 2.5), PlatformType::Small),
        (Vec3::new(8.0, 42.0, 0.0), Vec3::new(2.5, 0.5, 2.5), PlatformType::Small),
        (Vec3::new(12.0, 44.0, 0.0), Vec3::new(2.5, 0.5, 2.5), PlatformType::Small),

        // === SECTION 6: THE GAUNTLET (Very Hard) ===
        // Series of maximum-distance jumps
        (Vec3::new(18.0, 46.0, 0.0), Vec3::new(2.0, 0.5, 2.0), PlatformType::SteppingStone),
        (Vec3::new(26.0, 47.0, 0.0), Vec3::new(2.0, 0.5, 2.0), PlatformType::SteppingStone),
        (Vec3::new(34.0, 48.0, 0.0), Vec3::new(2.0, 0.5, 2.0), PlatformType::SteppingStone),
        (Vec3::new(42.0, 49.0, 0.0), Vec3::new(2.0, 0.5, 2.0), PlatformType::SteppingStone),

        // === SECTION 7: FINAL ASCENT (Expert) ===
        // Multiple path choices with varying difficulty

        // Left path - Safer but requires backtracking
        (Vec3::new(36.0, 51.0, 0.0), Vec3::new(2.5, 0.5, 2.5), PlatformType::Small),
        (Vec3::new(30.0, 53.0, 0.0), Vec3::new(2.5, 0.5, 2.5), PlatformType::Small),
        (Vec3::new(34.0, 55.0, 0.0), Vec3::new(2.5, 0.5, 2.5), PlatformType::Small),
        (Vec3::new(38.0, 57.0, 0.0), Vec3::new(2.5, 0.5, 2.5), PlatformType::Small),

        // Right path - Direct but very challenging
        (Vec3::new(46.0, 52.0, 0.0), Vec3::new(1.5, 0.5, 1.5), PlatformType::SteppingStone),
        (Vec3::new(50.0, 55.0, 0.0), Vec3::new(1.5, 0.5, 1.5), PlatformType::SteppingStone),
        (Vec3::new(46.0, 58.0, 0.0), Vec3::new(1.5, 0.5, 1.5), PlatformType::SteppingStone),

        // Center path - Balanced difficulty
        (Vec3::new(42.0, 53.0, 0.0), Vec3::new(2.0, 0.5, 2.0), PlatformType::SteppingStone),
        (Vec3::new(38.0, 56.0, 0.0), Vec3::new(2.0, 0.5, 2.0), PlatformType::SteppingStone),
        (Vec3::new(42.0, 59.0, 0.0), Vec3::new(2.0, 0.5, 2.0), PlatformType::SteppingStone),

        // === SECTION 8: FINAL CONVERGENCE ===
        // All paths lead here
        (Vec3::new(40.0, 61.0, 0.0), Vec3::new(5.0, 0.5, 5.0), PlatformType::Bridge),

        // === SECTION 9: VICTORY CHALLENGE (Master) ===
        // Final test of all skills learned
        (Vec3::new(35.0, 63.5, 0.0), Vec3::new(1.8, 0.5, 1.8), PlatformType::SteppingStone),
        (Vec3::new(31.0, 65.0, 0.0), Vec3::new(1.8, 0.5, 1.8), PlatformType::SteppingStone),
        (Vec3::new(35.0, 66.5, 0.0), Vec3::new(1.8, 0.5, 1.8), PlatformType::SteppingStone),
        (Vec3::new(39.0, 68.0, 0.0), Vec3::new(1.8, 0.5, 1.8), PlatformType::SteppingStone),
        (Vec3::new(43.0, 69.5, 0.0), Vec3::new(1.8, 0.5, 1.8), PlatformType::SteppingStone),

        // === FINAL PLATFORM (Victory) ===
        (Vec3::new(40.0, 72.0, 0.0), Vec3::new(8.0, 0.5, 8.0), PlatformType::Bridge),

        // === OPTIONAL SECRET AREAS ===
        // Hidden high-skill bonus platforms
        (Vec3::new(0.0, 45.0, 0.0), Vec3::new(2.0, 0.5, 2.0), PlatformType::SteppingStone),
        (Vec3::new(-6.0, 48.0, 0.0), Vec3::new(2.0, 0.5, 2.0), PlatformType::SteppingStone),
        (Vec3::new(6.0, 48.0, 0.0), Vec3::new(2.0, 0.5, 2.0), PlatformType::SteppingStone),
        (Vec3::new(0.0, 51.0, 0.0), Vec3::new(3.0, 0.5, 3.0), PlatformType::Floating),

        // Emergency fallback platforms (slightly hidden)
        (Vec3::new(20.0, 25.0, 0.0), Vec3::new(3.0, 0.5, 3.0), PlatformType::Small),
        (Vec3::new(25.0, 35.0, 0.0), Vec3::new(3.0, 0.5, 3.0), PlatformType::Small),
        (Vec3::new(30.0, 45.0, 0.0), Vec3::new(3.0, 0.5, 3.0), PlatformType::Small),
    ];

    for (i, (position, size, platform_type)) in platform_configs.iter().enumerate() {
        let color = match platform_type {
            PlatformType::Ground => Color::rgb(0.5, 0.5, 0.5),
            PlatformType::Floating => Color::rgb(0.5, 0.5, 0.5),
            PlatformType::Small => Color::rgb(0.6, 0.6, 0.6),
            PlatformType::SteppingStone => Color::rgb(0.4, 0.4, 0.4),
            PlatformType::Bridge => Color::rgb(0.7, 0.7, 0.7),
            PlatformType::Moving => Color::rgb(0.8, 0.4, 0.4),
            PlatformType::Trampoline => Color::rgb(0.2, 0.8, 0.2),
        };

        // Determine if this platform should have lights
        let has_lights = match platform_type {
            PlatformType::Bridge => true, // All bridges have lights
            PlatformType::Floating => i % 3 == 0, // Every 3rd floating platform
            PlatformType::Small => i % 4 == 0, // Every 4th small platform
            PlatformType::SteppingStone => i % 5 == 0, // Every 5th stepping stone
            PlatformType::Trampoline => true, // All trampolines have lights
            _ => false,
        };

        // Create material based on whether platform has lights
        let material = if has_lights {
            materials.add(StandardMaterial {
                base_color: color,
                perceptual_roughness: 0.3,
                metallic: 0.1,
                reflectance: 0.8,
                emissive: match platform_type {
                    PlatformType::Trampoline => Color::rgb(0.5, 2.0, 0.5), // Strong green glow
                    PlatformType::Bridge => Color::rgb(0.8, 0.4, 1.0), // Purple glow
                    _ => Color::rgb(0.4, 0.8, 1.0), // Cyan glow for others
                },
                ..default()
            })
        } else {
            materials.add(StandardMaterial {
                base_color: color,
                perceptual_roughness: 0.8,
                metallic: 0.0,
                reflectance: 0.4,
                ..default()
            })
        };

        let platform_entity = commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(RoundedBox {
                    size: *size,
                    radius: 0.2,
                    subdivisions: 6,
                    options: BoxMeshOptions::DEFAULT,
                })),
                material,
                transform: Transform::from_translation(*position),
                ..default()
            },
            RigidBody::Fixed,
            // No collider on the main entity
            Platform {
                platform_type: platform_type.clone(),
                is_active: true,
                has_coin: false,
                has_lights,
            },
            Name::new(format!("Platform_{}", i)),
        ))
        .with_children(|parent| {
            // Top surface with friction for walking
            parent.spawn((
                TransformBundle::from_transform(Transform::from_xyz(0.0, size.y * 0.3, 0.0)),
                Collider::round_cuboid(size.x * 0.5, size.y * 0.1, size.z * 0.5, 0.05),
                Friction {
                    coefficient: 0.8,
                    combine_rule: CoefficientCombineRule::Max,
                },
                Restitution {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Min,
                },
            ));

            // Side/bottom collider without friction
            parent.spawn((
                TransformBundle::from_transform(Transform::from_xyz(0.0, -size.y * 0.2, 0.0)),
                Collider::round_cuboid(size.x * 0.5, size.y * 0.5, size.z * 0.5, 0.1),
                Friction {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Min,
                },
                Restitution {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Min,
                },
            ));

            // Add point light if platform has lights
            if has_lights {
                let (light_color, light_intensity) = match platform_type {
                    PlatformType::Trampoline => (Color::rgb(0.2, 1.0, 0.2), 800.0), // Bright green
                    PlatformType::Bridge => (Color::rgb(0.8, 0.4, 1.0), 600.0), // Purple
                    _ => (Color::rgb(0.4, 0.8, 1.0), 500.0), // Cyan
                };

                parent.spawn((
                    PointLightBundle {
                        point_light: PointLight {
                            intensity: light_intensity,
                            color: light_color,
                            shadows_enabled: true,
                            range: size.x.max(size.z) * 3.0, // Scale range with platform size
                            radius: 0.8,
                            ..default()
                        },
                        transform: Transform::from_xyz(0.0, size.y * 0.5, 0.0), // Position above platform
                        ..default()
                    },
                    Name::new(format!("PlatformLight_{}", i)),
                ));
            }
        });
    }

    info!("Platforms setup complete");
}

fn setup_physics_world(mut commands: Commands) {
    // Configure physics world
    commands.insert_resource(RapierConfiguration {
        gravity: Vec3::new(0.0, -9.81, 0.0),
        physics_pipeline_active: true,
        query_pipeline_active: true,
        timestep_mode: TimestepMode::Variable {
            max_dt: 1.0 / 60.0,
            time_scale: 1.0,
            substeps: 1,
        },
        scaled_shape_subdivision: 10,
        force_update_from_transform_changes: false,
    });

    info!("Physics world configured");
}

fn setup_coins(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut platform_query: Query<(Entity, &mut Platform, &Transform)>,
) {
    // Define which platforms should have coins (using indices for simplicity)
    let coin_platform_indices = vec![
        2, 5, 8, 12, 15, 18, 22, 25, 28, 32, 35, 38, 42, 45, 48, 52, 55, 58, 62, 65
    ];

    // Gold coin material
    let coin_material = materials.add(StandardMaterial {
        base_color: Color::rgb(1.0, 0.8, 0.0), // Gold color
        metallic: 0.9,
        perceptual_roughness: 0.1,
        reflectance: 0.8,
        emissive: Color::rgb(0.2, 0.15, 0.0), // Slight glow
        ..default()
    });

    // Coin mesh - cylinder to look like a coin
    let coin_mesh = meshes.add(Mesh::from(shape::Cylinder {
        radius: 0.3,
        height: 0.1,
        resolution: 16,
        segments: 1,
    }));

    let platform_count = platform_query.iter().count();
    info!("Found {} platforms for coin setup", platform_count);

    let mut platform_index = 0;
    for (platform_entity, mut platform, platform_transform) in platform_query.iter_mut() {
        // Skip ground platform (index 0)
        if platform_index == 0 {
            platform_index += 1;
            continue;
        }

        // Check if this platform should have a coin
        if coin_platform_indices.contains(&platform_index) {
            platform.has_coin = true;

                        // Spawn coin above the platform
            let coin_base_position = platform_transform.translation + Vec3::new(0.0, 1.5, 0.0);

            let _coin_entity = commands.spawn((
                PbrBundle {
                    mesh: coin_mesh.clone(),
                    material: coin_material.clone(),
                    transform: Transform::from_translation(coin_base_position)
                        .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                    ..default()
                },
                RigidBody::KinematicPositionBased,
                Collider::cylinder(0.05, 0.3), // Thin collider for coin
                Coin {
                    platform_entity: Some(platform_entity),
                    float_height: coin_base_position.y, // Set the base floating height
                    ..default()
                },
                Name::new(format!("Coin_{}", platform_index)),
            )).id();

            info!("Spawned coin on platform {}", platform_index);
        }

        platform_index += 1;
    }

    info!("Coins setup complete");
}

fn setup_coins_delayed(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut platform_query: Query<(Entity, &mut Platform, &Transform)>,
    mut done: Local<bool>,
) {
    // Only run once
    if *done {
        return;
    }

    let platform_count = platform_query.iter().count();
    if platform_count == 0 {
        return; // Wait for platforms to be spawned
    }

    info!("Setting up coins with {} platforms found", platform_count);

    // Define which platforms should have coins (using indices for simplicity)
    let coin_platform_indices = vec![
        1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39
    ];

    // Gold coin material with strong glow
    let coin_material = materials.add(StandardMaterial {
        base_color: Color::rgb(1.0, 0.8, 0.0), // Gold color
        metallic: 0.7,
        perceptual_roughness: 0.1,
        reflectance: 0.9,
        emissive: Color::rgb(1.5, 1.2, 0.3), // Strong golden glow
        ..default()
    });

    // Coin mesh - cylinder to look like a coin
    let coin_mesh = meshes.add(Mesh::from(shape::Cylinder {
        radius: 0.3,
        height: 0.1,
        resolution: 16,
        segments: 1,
    }));

    let mut coins_spawned = 0;
    let mut platform_index = 0;
    for (platform_entity, mut platform, platform_transform) in platform_query.iter_mut() {
        // Check if this platform should have a coin
        if coin_platform_indices.contains(&platform_index) {
            platform.has_coin = true;

            // Spawn coin above the platform
            let coin_base_position = platform_transform.translation + Vec3::new(0.0, 1.5, 0.0);

            let _coin_entity = commands.spawn((
                PbrBundle {
                    mesh: coin_mesh.clone(),
                    material: coin_material.clone(),
                    transform: Transform::from_translation(coin_base_position)
                        .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                    ..default()
                },
                RigidBody::KinematicPositionBased,
                Collider::cylinder(0.05, 0.3), // Thin collider for coin
                Coin {
                    platform_entity: Some(platform_entity),
                    float_height: coin_base_position.y, // Set the base floating height
                    ..default()
                },
                Name::new(format!("Coin_{}", platform_index)),
            )).with_children(|parent| {
                // Add glowing point light to the coin
                parent.spawn((
                    PointLightBundle {
                        point_light: PointLight {
                            intensity: 300.0,
                            color: Color::rgb(1.0, 0.8, 0.2), // Warm golden light
                            shadows_enabled: false, // Disable shadows for performance
                            range: 6.0,
                            radius: 0.3,
                            ..default()
                        },
                        transform: Transform::from_xyz(0.0, 0.0, 0.0), // Center on coin
                        ..default()
                    },
                    Name::new(format!("CoinLight_{}", platform_index)),
                ));
            }).id();

            coins_spawned += 1;
            info!("Spawned coin on platform {}", platform_index);
        }

        platform_index += 1;
    }

    info!("Coins setup complete! Spawned {} coins", coins_spawned);
    *done = true;
}

fn setup_coins_immediately(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut platform_query: Query<(Entity, &mut Platform, &Transform)>,
) {
    let platform_count = platform_query.iter().count();
    if platform_count == 0 {
        return; // Wait for platforms to be spawned
    }

    info!("Setting up coins immediately with {} platforms found", platform_count);

    // Define which platforms should have coins (using indices for simplicity)
    let coin_platform_indices = vec![
        1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39
    ];

    // Gold coin material with strong glow
    let coin_material = materials.add(StandardMaterial {
        base_color: Color::rgb(1.0, 0.8, 0.0), // Gold color
        metallic: 0.7,
        perceptual_roughness: 0.1,
        reflectance: 0.9,
        emissive: Color::rgb(1.5, 1.2, 0.3), // Strong golden glow
        ..default()
    });

    // Coin mesh - cylinder to look like a coin
    let coin_mesh = meshes.add(Mesh::from(shape::Cylinder {
        radius: 0.3,
        height: 0.1,
        resolution: 16,
        segments: 1,
    }));

    let mut coins_spawned = 0;
    let mut platform_index = 0;
    for (platform_entity, mut platform, platform_transform) in platform_query.iter_mut() {
        // Check if this platform should have a coin
        if coin_platform_indices.contains(&platform_index) {
            platform.has_coin = true;

            // Spawn coin above the platform
            let coin_base_position = platform_transform.translation + Vec3::new(0.0, 1.5, 0.0);

            let _coin_entity = commands.spawn((
                PbrBundle {
                    mesh: coin_mesh.clone(),
                    material: coin_material.clone(),
                    transform: Transform::from_translation(coin_base_position)
                        .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                    ..default()
                },
                RigidBody::KinematicPositionBased,
                Collider::cylinder(0.05, 0.3), // Thin collider for coin
                Coin {
                    platform_entity: Some(platform_entity),
                    float_height: coin_base_position.y, // Set the base floating height
                    ..default()
                },
                Name::new(format!("Coin_{}", platform_index)),
            )).with_children(|parent| {
                // Add glowing point light to the coin
                parent.spawn((
                    PointLightBundle {
                        point_light: PointLight {
                            intensity: 300.0,
                            color: Color::rgb(1.0, 0.8, 0.2), // Warm golden light
                            shadows_enabled: false, // Disable shadows for performance
                            range: 6.0,
                            radius: 0.3,
                            ..default()
                        },
                        transform: Transform::from_xyz(0.0, 0.0, 0.0), // Center on coin
                        ..default()
                    },
                    Name::new(format!("CoinLight_{}", platform_index)),
                ));
            }).id();

            coins_spawned += 1;
            info!("Spawned coin on platform {}", platform_index);
        }

        platform_index += 1;
    }

    info!("Coins setup complete! Spawned {} coins", coins_spawned);
}

fn animate_coins(
    time: Res<Time>,
    mut coin_query: Query<(Entity, &mut Transform, &Coin)>,
    mut light_query: Query<&mut PointLight, Without<Coin>>,
    children_query: Query<&Children>,
) {
    for (coin_entity, mut transform, coin) in coin_query.iter_mut() {
        // Float up and down
        let float_offset = (time.elapsed_seconds() * coin.float_speed).sin() * 0.2;
        transform.translation.y = coin.float_height + float_offset;

        // Rotate around Z axis
        transform.rotation *= Quat::from_rotation_z(coin.rotation_speed * time.delta_seconds());

        // Find and animate the child light
        if let Ok(children) = children_query.get(coin_entity) {
            for &child in children.iter() {
                if let Ok(mut point_light) = light_query.get_mut(child) {
                    // Pulsing glow effect
                    let pulse = (time.elapsed_seconds() * 3.0).sin() * 0.5 + 0.5; // 0.0 to 1.0
                    point_light.intensity = 200.0 + (pulse * 150.0); // Pulse between 200 and 350
                }
            }
        }
    }
}

fn handle_coin_collection(
    mut commands: Commands,
    coin_query: Query<(Entity, &Transform, &Coin)>,
    player_query: Query<(Entity, &Transform), (With<Player>, Without<Coin>)>,
    mut stats: ResMut<crate::resources::GameStats>,
) {
    // Check for player-coin collisions
    if let Ok((_player_entity, player_transform)) = player_query.get_single() {
        for (coin_entity, coin_transform, coin) in coin_query.iter() {
            let distance = player_transform.translation.distance(coin_transform.translation);

            if distance < coin.collection_radius {
                // Collect the coin
                stats.coins_collected += 1;

                // Remove coin from world (including child light)
                commands.entity(coin_entity).despawn_recursive();

                // Update platform to no longer have coin
                if let Some(platform_entity) = coin.platform_entity {
                    info!("Collected coin from platform {:?}", platform_entity);
                }

                info!("Coin collected! Total coins: {}", stats.coins_collected);
            }
        }
    }
}

fn handle_platform_interactions(
    platform_query: Query<(Entity, &Platform, &Transform), With<Platform>>,
    player_query: Query<(Entity, &Transform, &Player), (With<Player>, Without<Platform>)>,
    mut stats: ResMut<crate::resources::GameStats>,
) {
    if let Ok((_player_entity, player_transform, _player)) = player_query.get_single() {
        for (_platform_entity, platform, platform_transform) in platform_query.iter() {
            let distance = player_transform.translation.distance(platform_transform.translation);

            if distance < 3.0 && platform.is_active {
                // Player is near platform
                if matches!(platform.platform_type, PlatformType::SteppingStone) {
                    // Special behavior for stepping stones
                    stats.platform_touches += 1;
                }
            }
        }
    }
}

fn handle_trampoline_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    player_query: Query<Entity, With<Player>>,
    trampoline_query: Query<Entity, With<TrampolineTopSurface>>,
    mut trampoline_events: EventWriter<TrampolineBounceEvent>,
    mut last_trampoline_time: Local<f32>,
    time: Res<Time>,
) {
    let Ok(player_entity) = player_query.get_single() else {
        return;
    };

    for collision_event in collision_events.read() {
        let (entity1, entity2) = match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                info!("Collision started between {:?} and {:?}", e1, e2);
                (e1, e2)
            },
            CollisionEvent::Stopped(_, _, _) => continue,
        };

        // Check if collision is between player and trampoline top surface
        let is_player_trampoline_collision =
            (entity1 == &player_entity && trampoline_query.get(*entity2).is_ok()) ||
            (entity2 == &player_entity && trampoline_query.get(*entity1).is_ok());

        if is_player_trampoline_collision {
            let current_time = time.elapsed_seconds();
            let time_since_last_bounce = current_time - *last_trampoline_time;

            info!("Player collision with trampoline top surface detected! Time since last bounce: {}", time_since_last_bounce);

            // Prevent spam bouncing with cooldown
            if time_since_last_bounce > 0.2 {
                let trampoline_entity = if entity1 == &player_entity { *entity2 } else { *entity1 };

                trampoline_events.send(TrampolineBounceEvent {
                    player_entity,
                    bounce_force: 12.0, // Increased force
                    platform_entity: trampoline_entity,
                });

                *last_trampoline_time = current_time;
                info!("Player bounced on trampoline top surface! Boost applied with force 15.0");
            } else {
                info!("Trampoline bounce blocked by cooldown");
            }
        }
    }
}

fn update_physics_debug(
    config: Res<crate::resources::GameConfig>,
) {
    // Log when debug mode changes
    if config.is_changed() {
        if config.show_colliders {
            info!("Collider debug visualization: ON (Press F4 to toggle)");
        }

        if config.physics_debug {
            info!("Physics debug mode: ON (Press F5 to toggle)");
        }
    }
}

// Platform components defined here to avoid conflicts
#[derive(Component)]
pub struct Platform {
    pub platform_type: PlatformType,
    pub is_active: bool,
    pub has_coin: bool,
    pub has_lights: bool,
}

#[derive(Clone, Debug)]
pub enum PlatformType {
    Ground,
    Floating,
    Small,
    SteppingStone,
    Bridge,
    Moving,
    Trampoline,
}

// Coin component
#[derive(Component)]
pub struct Coin {
    pub float_height: f32,
    pub float_speed: f32,
    pub rotation_speed: f32,
    pub collection_radius: f32,
    pub platform_entity: Option<Entity>,
}

// Trampoline animation component
#[derive(Component)]
pub struct TrampolineAnimation {
    pub is_animating: bool,
    pub animation_time: f32,
    pub animation_duration: f32,
    pub original_transform: Transform,
    pub compression_amount: f32,
}

impl Default for Coin {
    fn default() -> Self {
        Self {
            float_height: 1.5,
            float_speed: 2.0,
            rotation_speed: 2.0,
            collection_radius: 1.0,
            platform_entity: None,
        }
    }
}

impl Default for TrampolineAnimation {
    fn default() -> Self {
        Self {
            is_animating: false,
            animation_time: 0.0,
            animation_duration: 0.3, // 300ms animation
            original_transform: Transform::IDENTITY,
            compression_amount: 0.3, // 30% compression
        }
    }
}

fn handle_trampoline_proximity(
    platform_query: Query<(Entity, &Platform, &Transform), With<Platform>>,
    player_query: Query<(Entity, &Transform, &Player, &Velocity), (With<Player>, Without<Platform>)>,
    mut trampoline_events: EventWriter<TrampolineBounceEvent>,
    mut last_proximity_bounce_time: Local<f32>,
    time: Res<Time>,
) {
    let Ok((player_entity, player_transform, player, player_velocity)) = player_query.get_single() else {
        return;
    };

    for (platform_entity, platform, platform_transform) in platform_query.iter() {
        if !matches!(platform.platform_type, PlatformType::Trampoline) {
            continue;
        }

        let distance = player_transform.translation.distance(platform_transform.translation);
        let height_diff = player_transform.translation.y - platform_transform.translation.y;

        // Check if player is above and close to the trampoline
        if distance < 2.5 && height_diff > 0.5 && height_diff < 1.5 && player.is_grounded {
            let current_time = time.elapsed_seconds();
            let time_since_last_bounce = current_time - *last_proximity_bounce_time;

            // Check if player is moving downward (just landed)
            if player_velocity.linvel.y < 1.0 && time_since_last_bounce > 0.3 {
                info!("Proximity-based trampoline bounce detected! Distance: {}, Height diff: {}, Velocity Y: {}",
                      distance, height_diff, player_velocity.linvel.y);

                trampoline_events.send(TrampolineBounceEvent {
                    player_entity,
                    bounce_force: 12.0,
                    platform_entity,
                });

                *last_proximity_bounce_time = current_time;
                info!("Proximity trampoline bounce applied with force 15.0");
            }
        }
    }
}

fn trigger_trampoline_animation(
    mut bounce_events: EventReader<TrampolineBounceEvent>,
    mut trampoline_query: Query<&mut TrampolineAnimation>,
    platform_query: Query<&Platform>,
) {
    for event in bounce_events.read() {
        // Check if the platform is a trampoline
        if let Ok(platform) = platform_query.get(event.platform_entity) {
            if matches!(platform.platform_type, PlatformType::Trampoline) {
                // Find the trampoline entity with animation component
                if let Ok(mut animation) = trampoline_query.get_mut(event.platform_entity) {
                    // Start the animation
                    animation.is_animating = true;
                    animation.animation_time = 0.0;
                    info!("Trampoline animation triggered!");
                }
            }
        }
    }
}

fn update_trampoline_animation(
    time: Res<Time>,
    mut trampoline_query: Query<(&mut Transform, &mut TrampolineAnimation)>,
) {
    for (mut transform, mut animation) in trampoline_query.iter_mut() {
        if !animation.is_animating {
            continue;
        }

        animation.animation_time += time.delta_seconds();

        let progress = animation.animation_time / animation.animation_duration;

        if progress >= 1.0 {
            // Animation complete - reset to original transform
            animation.is_animating = false;
            animation.animation_time = 0.0;
            transform.translation = animation.original_transform.translation;
            transform.scale = animation.original_transform.scale;
        } else {
            // Calculate animation using a bouncy ease-out function
            let bounce_progress = if progress < 0.5 {
                // Compression phase - ease in
                let t = progress * 2.0;
                t * t
            } else {
                // Expansion phase - ease out with bounce
                let t = (progress - 0.5) * 2.0;
                1.0 - (1.0 - t) * (1.0 - t)
            };

            // Apply compression effect
            let compression_factor = if progress < 0.5 {
                // Compress down
                1.0 - (bounce_progress * animation.compression_amount)
            } else {
                // Expand back up with slight overshoot
                let overshoot = 1.0 + (1.0 - bounce_progress) * 0.1;
                1.0 - (1.0 - bounce_progress) * animation.compression_amount * overshoot
            };

            // Apply scale and position changes
            transform.scale.y = animation.original_transform.scale.y * compression_factor;

            // Move down slightly when compressed
            let vertical_offset = (1.0 - compression_factor) * 0.2;
            transform.translation.y = animation.original_transform.translation.y - vertical_offset;
        }
    }
}

