use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::components::Boss;
use crate::plugins::physics::Platform;
use crate::resources::BossAnimations;

pub fn spawn_boss(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut done: Local<bool>,
) {
    if !*done {
        // Load the boss model
        let scene = asset_server.load("boss3.glb#Scene0");

        // Spawn the boss with physics components
        commands.spawn((
            Transform::from_xyz(0.0, 7.0, 0.0),
            GlobalTransform::default(),
            RigidBody::Dynamic,
            Collider::capsule_y(0.4, 0.4),
            Velocity {
                linvel: Vec3::ZERO,
                angvel: Vec3::ZERO,
            },
            GravityScale(1.0),
            LockedAxes::ROTATION_LOCKED,
            Friction {
                coefficient: 0.7,
                combine_rule: CoefficientCombineRule::Min,
            },
            Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            Damping {
                linear_damping: 0.5,
                angular_damping: 1.0,
            },
            Boss {
                speed: 5.0,
                is_moving: false,
                is_grounded: false,
                is_front_flipping: false,
                is_dive_rolling: false,
                flip_direction: Vec3::ZERO,
                facing_left: false,
            },
        )).with_children(|parent| {
            // Spawn the mesh as a child with an offset
            parent.spawn((
                SceneBundle {
                    scene,
                    transform: Transform::from_xyz(0.0, -0.8, 0.0) // Offset the mesh down relative to the collider
                        .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2))
                        .with_scale(Vec3::splat(1.0)),
                    ..default()
                },
                Name::new("Boss Model"), // Add a name for debugging
            ));
        });

        *done = true;
    }
}

pub fn move_boss(
    keyboard: Res<Input<KeyCode>>,
    mut boss_query: Query<(Entity, &mut Transform, &mut Velocity, &mut Boss), Without<Platform>>,
    rapier_context: Res<RapierContext>,
) {
    if let Ok((entity, mut transform, mut velocity, mut boss)) = boss_query.get_single_mut() {
        if boss.is_front_flipping {
            // During front flip, only control horizontal movement
            velocity.linvel.x = boss.flip_direction.x * boss.speed * 1.5; // Keep horizontal movement
            // Let gravity handle vertical movement naturally
            return; // Skip other movement processing
        }

        if boss.is_dive_rolling {
            // During dive roll, maintain horizontal momentum but allow gravity
            velocity.linvel.x = boss.flip_direction.x * boss.speed * 2.0; // Faster than front flip
            return; // Skip other movement processing
        }

        let mut direction = 0.0;

        // Horizontal movement
        if keyboard.pressed(KeyCode::A) {
            direction -= 1.0;
            boss.is_moving = true;
            boss.facing_left = true;
            transform.rotation = Quat::from_rotation_y(-std::f32::consts::PI);
        }
        if keyboard.pressed(KeyCode::D) {
            direction += 1.0;
            boss.is_moving = true;
            boss.facing_left = false;
            transform.rotation = Quat::from_rotation_y(0.0);
        }

        if direction == 0.0 {
            boss.is_moving = false;
            // Apply strong friction to stop horizontal movement immediately
            if boss.is_grounded {
                velocity.linvel.x *= 0.07; // Much stronger friction when grounded
            } else {
                velocity.linvel.x *= 0.95; // Lighter friction when in air
            }
        }

        // Update horizontal velocity
        if direction != 0.0 {
            velocity.linvel.x = direction * boss.speed;
        }

        // Check if the boss is on the ground or a platform
        let ray_pos = transform.translation;
        let ray_dir = Vec3::new(0.0, -1.0, 0.0);
        let max_distance = 1.0;

        // Cast ray to check for ground
        let ground_hit = rapier_context.cast_ray(
            ray_pos,
            ray_dir,
            max_distance,
            true,
            QueryFilter::default()
                .exclude_rigid_body(entity),
        );

        // Update grounded state
        boss.is_grounded = ground_hit.is_some();

        // Handle jumping when on ground
        if boss.is_grounded && keyboard.just_pressed(KeyCode::Space) {
            velocity.linvel.y = 8.0; // Jump velocity
        }

        // Handle front flip trigger
        if keyboard.just_pressed(KeyCode::W) && !boss.is_front_flipping && boss.is_grounded {
            boss.is_front_flipping = true;

            boss.flip_direction = if boss.facing_left {
                Vec3::new(-0.5, 0.0, 0.0)
            } else {
                Vec3::new(0.5, 0.0, 0.0)
            };
            return;
        }

        // Handle dive roll trigger
        if keyboard.just_pressed(KeyCode::S) && !boss.is_dive_rolling && boss.is_grounded {
            boss.is_dive_rolling = true;

            boss.flip_direction = if boss.facing_left {
                Vec3::new(-0.5, 0.0, 0.0) // Faster horizontal movement for dive roll
            } else {
                Vec3::new(0.5, 0.0, 0.0)
            };
            return;
        }
    }
}

// This system controls the animation state based on boss movement
pub fn control_animation(
    mut boss_query: Query<(&mut Boss, &Velocity)>,
    mut animation_players: Query<&mut AnimationPlayer>,
    animations: Res<BossAnimations>,
) {
    let (mut boss, velocity) = if let Ok(data) = boss_query.get_single_mut() {
        data
    } else {
        return;
    };

    // Use both the is_moving flag and velocity to determine if actually moving
    let is_actually_moving = boss.is_moving && velocity.linvel.x.abs() > 0.1;

    for mut player in animation_players.iter_mut() {
        if boss.is_front_flipping {
            player.play(animations.front_flip.clone());
            if player.is_finished() {
                boss.is_front_flipping = false;
            }
        } else if boss.is_dive_rolling {
            player.play(animations.dive_roll.clone());
            if player.is_finished() {
                boss.is_dive_rolling = false;
            }
        } else if !boss.is_grounded {
            player.play(animations.air.clone()).repeat();
        } else if is_actually_moving {
            player.play(animations.walk.clone()).repeat();
        } else {
            player.play(animations.idle.clone()).repeat();
        }
    }
}

// Add this new system after the other systems
pub fn debug_animation_setup(
    animation_players: Query<Entity, Added<AnimationPlayer>>,
    names: Query<&Name>,
) {
    for entity in animation_players.iter() {
        if let Ok(name) = names.get(entity) {
            info!("Entity name: {}", name);
        }
    }
}