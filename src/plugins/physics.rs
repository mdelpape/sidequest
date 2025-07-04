use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::{
    components::*,
    states::*,
};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(PlayState::Setup), (
                setup_platforms,
                setup_physics_world,
            ))
            .add_systems(Update, (
                handle_platform_interactions,
                update_physics_debug,
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
            mesh: meshes.add(Mesh::from(shape::Box::new(50.0, 0.5, 5.0))),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(0.0, -0.25, 0.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(25.0, 0.25, 2.5),
        Friction {
            coefficient: 0.8,
            combine_rule: CoefficientCombineRule::Max,
        },
        Restitution {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        },
        Platform {
            platform_type: PlatformType::Ground,
            is_active: true,
        },
        Name::new("GroundPlatform"),
    ));

    // Floating platforms configuration
    let platform_configs = vec![
        // Left side platforms
        (Vec3::new(-8.0, 2.0, 0.0), Vec3::new(4.0, 0.5, 3.0), PlatformType::Floating),
        (Vec3::new(-12.0, 4.5, 0.0), Vec3::new(3.0, 0.5, 2.5), PlatformType::Floating),
        (Vec3::new(-6.0, 6.0, 0.0), Vec3::new(2.5, 0.5, 2.0), PlatformType::Floating),

        // Right side platforms
        (Vec3::new(8.0, 1.5, 0.0), Vec3::new(3.5, 0.5, 4.0), PlatformType::Floating),
        (Vec3::new(12.0, 3.5, 0.0), Vec3::new(2.0, 0.5, 2.0), PlatformType::Floating),
        (Vec3::new(10.0, 5.5, 0.0), Vec3::new(3.0, 0.5, 2.5), PlatformType::Floating),

        // Center progression platforms
        (Vec3::new(0.0, 3.0, 0.0), Vec3::new(2.0, 0.5, 2.0), PlatformType::Floating),
        (Vec3::new(-2.0, 5.0, 0.0), Vec3::new(1.5, 0.5, 1.5), PlatformType::Small),
        (Vec3::new(2.0, 7.0, 0.0), Vec3::new(1.5, 0.5, 1.5), PlatformType::Small),

        // Stepping stones
        (Vec3::new(-4.0, 8.5, 0.0), Vec3::new(1.0, 0.5, 1.0), PlatformType::SteppingStone),
        (Vec3::new(4.0, 8.5, 0.0), Vec3::new(1.0, 0.5, 1.0), PlatformType::SteppingStone),

        // High bridge
        (Vec3::new(0.0, 10.0, 0.0), Vec3::new(8.0, 0.5, 1.5), PlatformType::Bridge),
    ];

    for (i, (position, size, platform_type)) in platform_configs.iter().enumerate() {
        let color = match platform_type {
            PlatformType::Ground => Color::rgb(0.3, 0.5, 0.3),
            PlatformType::Floating => Color::rgb(0.4, 0.6, 0.4),
            PlatformType::Small => Color::rgb(0.5, 0.7, 0.5),
            PlatformType::SteppingStone => Color::rgb(0.6, 0.8, 0.6),
            PlatformType::Bridge => Color::rgb(0.7, 0.9, 0.7),
            PlatformType::Moving => Color::rgb(0.8, 0.4, 0.4),
        };

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(size.x, size.y, size.z))),
                material: materials.add(color.into()),
                transform: Transform::from_translation(*position),
                ..default()
            },
            RigidBody::Fixed,
            Collider::cuboid(size.x * 0.5, size.y * 0.5, size.z * 0.5),
            Friction {
                coefficient: 0.8,
                combine_rule: CoefficientCombineRule::Max,
            },
            Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            Platform {
                platform_type: platform_type.clone(),
                is_active: true,
            },
            Name::new(format!("Platform_{}", i)),
        ));
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

fn handle_platform_interactions(
    platform_query: Query<(&Platform, &Transform), With<Platform>>,
    player_query: Query<&Transform, (With<Boss>, Without<Platform>)>,
    mut stats: ResMut<crate::resources::GameStats>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (platform, platform_transform) in platform_query.iter() {
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

fn update_physics_debug(
    config: Res<crate::resources::GameConfig>,
) {
    // Physics debug rendering would need RapierDebugRenderPlugin to be added to main.rs
    // For now, just log when debug mode changes
    if config.is_changed() && config.physics_debug {
        info!("Physics debug mode: {}", config.physics_debug);
    }
}

// Platform components defined here to avoid conflicts
#[derive(Component)]
pub struct Platform {
    pub platform_type: PlatformType,
    pub is_active: bool,
}

#[derive(Clone, Debug)]
pub enum PlatformType {
    Ground,
    Floating,
    Small,
    SteppingStone,
    Bridge,
    Moving,
}