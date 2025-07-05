use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::components::Player;
use crate::plugins::physics::Platform;
use crate::resources::PlayerAnimations;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut done: Local<bool>,
) {
    if !*done {
        // Load the player model
        let scene = asset_server.load("boss3.glb#Scene0");

        // Spawn the player with physics components
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
            Player {
                speed: 5.0,
                is_moving: false,
                is_grounded: false,
                is_front_flipping: false,
                is_dive_rolling: false,
                flip_direction: Vec3::ZERO,
                facing_left: false,
                is_falling: false,
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
                Name::new("Player Model"), // Add a name for debugging
            ));
        });

        *done = true;
    }
}

pub fn move_player(
    keyboard: Res<Input<KeyCode>>,
    mut player_query: Query<(Entity, &mut Transform, &mut Velocity, &mut Player), Without<Platform>>,
    rapier_context: Res<RapierContext>,
) {
    if let Ok((entity, mut transform, mut velocity, mut player)) = player_query.get_single_mut() {
        if player.is_front_flipping {
            // During front flip, only control horizontal movement
            velocity.linvel.x = player.flip_direction.x * player.speed * 1.5; // Keep horizontal movement
            // Let gravity handle vertical movement naturally
            return; // Skip other movement processing
        }

        if player.is_dive_rolling {
            // During dive roll, maintain horizontal momentum but allow gravity
            velocity.linvel.x = player.flip_direction.x * player.speed * 2.0; // Faster than front flip
            return; // Skip other movement processing
        }

        // First, do all collision detection
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
        player.is_grounded = ground_hit.is_some();

        // Now handle horizontal movement - let physics handle collisions naturally
        let mut direction = 0.0;

        // Get input direction
        if keyboard.pressed(KeyCode::A) {
            direction -= 1.0;
            player.is_moving = true;
            player.facing_left = true;
            transform.rotation = Quat::from_rotation_y(-std::f32::consts::PI);
        }
        if keyboard.pressed(KeyCode::D) {
            direction += 1.0;
            player.is_moving = true;
            player.facing_left = false;
            transform.rotation = Quat::from_rotation_y(0.0);
        }

        if direction == 0.0 {
            player.is_moving = false;
            // Apply strong friction to stop horizontal movement immediately
            if player.is_grounded {
                velocity.linvel.x *= 0.07; // Much stronger friction when grounded
            } else {
                velocity.linvel.x *= 0.95; // Lighter friction when in air
            }
        } else {
            // Apply movement directly - let physics engine handle collisions
            velocity.linvel.x = direction * player.speed;
        }

        // Handle jumping when on ground
        if player.is_grounded && keyboard.just_pressed(KeyCode::Space) {
            velocity.linvel.y = 10.0; // Jump velocity
        }

        // Handle front flip trigger
        if keyboard.just_pressed(KeyCode::W) && !player.is_front_flipping && player.is_grounded {
            player.is_front_flipping = true;

            player.flip_direction = if player.facing_left {
                Vec3::new(-0.5, 0.0, 0.0)
            } else {
                Vec3::new(0.5, 0.0, 0.0)
            };
            return;
        }

        // Handle dive roll trigger
        if keyboard.just_pressed(KeyCode::S) && !player.is_dive_rolling && player.is_grounded {
            player.is_dive_rolling = true;

            player.flip_direction = if player.facing_left {
                Vec3::new(-0.5, 0.0, 0.0) // Faster horizontal movement for dive roll
            } else {
                Vec3::new(0.5, 0.0, 0.0)
            };
            return;
        }
    }
}

// This system controls the animation state based on player movement
pub fn control_animation(
    mut player_query: Query<(&mut Player, &Velocity)>,
    mut animation_players: Query<&mut AnimationPlayer>,
    animations: Res<PlayerAnimations>,
) {
    let (mut player, velocity) = if let Ok(data) = player_query.get_single_mut() {
        data
    } else {
        return;
    };

    // Use both the is_moving flag and velocity to determine if actually moving
    let is_actually_moving = player.is_moving && velocity.linvel.x.abs() > 0.1;

    for mut animation_player in animation_players.iter_mut() {
        if player.is_front_flipping {
            animation_player.play(animations.front_flip.clone());
            if animation_player.is_finished() {
                player.is_front_flipping = false;
            }
        } else if player.is_dive_rolling {
            animation_player.play(animations.dive_roll.clone());
            if animation_player.is_finished() {
                player.is_dive_rolling = false;
            }
        } else if !player.is_grounded {
            animation_player.play(animations.air.clone()).repeat();
        } else if is_actually_moving {
            animation_player.play(animations.walk.clone()).repeat();
        } else {
            animation_player.play(animations.idle.clone()).repeat();
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