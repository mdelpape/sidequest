use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::{
    components::*,
    events::*,
    resources::{GameStats, PlayerAnimations},
    states::*,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(PlayState::Setup), spawn_player)
            .add_systems(Update, (
                handle_player_movement,
                handle_player_jump,
                handle_player_flip,
                update_player_state,
                handle_player_animation,
                check_player_grounded,
                manage_dive_roll_hitbox,
            ).run_if(in_state(GameState::Playing)))
            .add_systems(Update, (
                check_player_fall,
                handle_player_fall,
                update_death_vignette,
                handle_player_respawn,
            ).run_if(in_state(GameState::Playing)));
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut spawn_events: EventWriter<PlayerSpawnEvent>,
) {
    let scene = asset_server.load("boss3.glb#Scene0");
    let spawn_position = Vec3::new(0.0, 7.0, 0.0);

    commands.spawn((
        Transform::from_translation(spawn_position),
        GlobalTransform::default(),
        RigidBody::Dynamic,
        Collider::capsule_y(0.4, 0.4),
        MainCollider,
        Velocity::default(),
        GravityScale(1.0),
        LockedAxes::ROTATION_LOCKED,
        Friction {
            coefficient: 0.3,
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
        InheritedVisibility::default(),
        ViewVisibility::default(),
        Name::new("Player"),
    )).with_children(|parent| {
        parent.spawn((
            SceneBundle {
                scene,
                transform: Transform::from_xyz(0.0, -0.8, 0.0)
                    .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2))
                    .with_scale(Vec3::splat(1.0)),
                ..default()
            },
            Name::new("PlayerModel"),
        ));
    });

    spawn_events.send(PlayerSpawnEvent {
        position: spawn_position,
    });

    info!("Player spawned at {:?}", spawn_position);
}

fn manage_dive_roll_hitbox(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Player, &mut Collider), With<MainCollider>>,
    dive_roll_query: Query<Entity, With<DiveRollCollider>>,
) {
    if let Ok((player_entity, player, mut main_collider)) = player_query.get_single_mut() {
        let has_dive_roll_collider = !dive_roll_query.is_empty();

        if player.is_dive_rolling && !has_dive_roll_collider {
            // Start dive roll: disable main collider and spawn dive roll collider
            *main_collider = Collider::ball(0.0001); // Make main collider tiny but keep it

            // Spawn dive roll collider as a child entity at the player's feet
            commands.entity(player_entity).with_children(|parent| {
                parent.spawn((
                    TransformBundle::from_transform(Transform::from_xyz(0.0, -0.5, 0.0)),
                    Collider::capsule_y(0.15, 0.2), // Smaller capsule: radius 0.15, height 0.2
                    DiveRollCollider,
                    Name::new("DiveRollCollider"),
                ));
            });

            info!("Dive roll hitbox activated - smaller collider at feet");

        } else if !player.is_dive_rolling && has_dive_roll_collider {
            // End dive roll: remove dive roll collider and restore main collider
            for dive_roll_entity in dive_roll_query.iter() {
                commands.entity(dive_roll_entity).despawn();
            }

            // Restore main collider
            *main_collider = Collider::capsule_y(0.4, 0.4);

            info!("Dive roll hitbox deactivated - main collider restored");
        }
    }
}

fn handle_player_movement(
    mut move_events: EventReader<PlayerMoveEvent>,
    mut player_query: Query<(&mut Transform, &mut Velocity, &mut Player)>,
) {
    for event in move_events.read() {
        if let Ok((mut transform, mut velocity, mut player)) = player_query.get_mut(event.entity) {
            if player.is_front_flipping || player.is_dive_rolling {
                continue;
            }

            let direction = event.direction.x;
            player.is_moving = direction != 0.0;

            if direction < 0.0 {
                player.facing_left = true;
                transform.rotation = Quat::from_rotation_y(-std::f32::consts::PI);
            } else if direction > 0.0 {
                player.facing_left = false;
                transform.rotation = Quat::from_rotation_y(0.0);
            }

            velocity.linvel.x = direction * player.speed;

            if direction == 0.0 {
                // Apply strong friction to stop horizontal movement immediately
                if player.is_grounded {
                    velocity.linvel.x *= 0.07; // Much stronger friction when grounded
                } else {
                    velocity.linvel.x *= 0.95; // Lighter friction when in air
                }
            }
        }
    }
}

fn handle_player_jump(
    mut jump_events: EventReader<PlayerJumpEvent>,
    mut player_query: Query<(&mut Velocity, &Player)>,
    mut stats: ResMut<GameStats>,
) {
    for event in jump_events.read() {
        if let Ok((mut velocity, player)) = player_query.get_mut(event.entity) {
            if player.is_grounded {
                velocity.linvel.y = 8.0;
                stats.jump_count += 1;
                info!("Player jumped! Total jumps: {}", stats.jump_count);
            }
        }
    }
}

fn handle_player_flip(
    mut flip_events: EventReader<PlayerFlipEvent>,
    mut player_query: Query<(&mut Player, &Transform)>,
    mut stats: ResMut<GameStats>,
) {
    for event in flip_events.read() {
        if let Ok((mut player, _transform)) = player_query.get_mut(event.entity) {
            if !player.is_grounded {
                continue;
            }

            match event.flip_type {
                FlipType::Front => {
                    if !player.is_front_flipping {
                        player.is_front_flipping = true;
                        player.flip_direction = if player.facing_left {
                            Vec3::new(-0.5, 0.0, 0.0)
                        } else {
                            Vec3::new(0.5, 0.0, 0.0)
                        };
                        stats.flip_count += 1;
                        info!("Player front flipped! Total flips: {}", stats.flip_count);
                    }
                }
                FlipType::Dive => {
                    if !player.is_dive_rolling {
                        player.is_dive_rolling = true;
                        player.flip_direction = if player.facing_left {
                            Vec3::new(-0.4, 0.0, 0.0)
                        } else {
                            Vec3::new(0.4, 0.0, 0.0)
                        };
                        stats.flip_count += 1;
                        info!("Player dive rolled! Total flips: {}", stats.flip_count);
                    }
                }
            }
        }
    }
}

fn update_player_state(
    mut player_query: Query<(&mut Velocity, &Player)>,
) {
    for (mut velocity, player) in player_query.iter_mut() {
        if player.is_front_flipping {
            velocity.linvel.x = player.flip_direction.x * player.speed * 1.5;
        } else if player.is_dive_rolling {
            velocity.linvel.x = player.flip_direction.x * player.speed * 2.0;
        }
    }
}

fn handle_player_animation(
    mut player_query: Query<(&mut Player, &Velocity)>,
    mut animation_players: Query<&mut AnimationPlayer>,
    animations: Res<PlayerAnimations>,
) {
    for (mut player, velocity) in player_query.iter_mut() {
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
}

fn check_player_grounded(
    mut player_query: Query<(Entity, &Transform, &mut Player)>,
    rapier_context: Res<RapierContext>,
) {
    for (entity, transform, mut player) in player_query.iter_mut() {
        let ray_pos = transform.translation;
        let ray_dir = Vec3::new(0.0, -1.0, 0.0);
        let max_distance = 1.0;

        let ground_hit = rapier_context.cast_ray(
            ray_pos,
            ray_dir,
            max_distance,
            true,
            QueryFilter::default().exclude_rigid_body(entity),
        );

        let was_grounded = player.is_grounded;
        player.is_grounded = ground_hit.is_some();

        // Track landing events
        if !was_grounded && player.is_grounded {
            // Player just landed
            info!("Player landed!");
        }
    }
}

fn check_player_fall(
    mut player_query: Query<(Entity, &Transform, &mut Player)>,
    mut fall_events: EventWriter<PlayerFallEvent>,
    mut stats: ResMut<GameStats>,
) {
    const FALL_THRESHOLD: f32 = -10.0; // Y position below which player is considered fallen

    for (entity, transform, mut player) in player_query.iter_mut() {
        if transform.translation.y < FALL_THRESHOLD && !player.is_falling {
            player.is_falling = true;
            fall_events.send(PlayerFallEvent {
                entity,
                position: transform.translation,
            });
            stats.fall_count += 1;
            info!("Player fell off platform! Total falls: {}", stats.fall_count);
        }
    }
}

fn handle_player_fall(
    mut commands: Commands,
    mut fall_events: EventReader<PlayerFallEvent>,
    time: Res<Time>,
) {
    for _event in fall_events.read() {
        // Create the death vignette effect - overlay that covers the entire screen
        commands.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    border: UiRect::all(Val::Px(0.0)),
                    ..default()
                },
                background_color: Color::rgba(1.0, 0.0, 0.0, 0.0).into(),
                z_index: ZIndex::Global(1000),
                ..default()
            },
            DeathVignette {
                start_time: time.elapsed_seconds(),
                ..default()
            },
            Name::new("DeathVignette"),
        )).with_children(|parent| {
            // Create a radial gradient effect by layering multiple nested elements
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        position_type: PositionType::Absolute,
                        left: Val::Px(0.0),
                        top: Val::Px(0.0),
                        ..default()
                    },
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                    ..default()
                },
                Name::new("VignetteInner"),
            ));
        });

        info!("Death vignette effect started");
    }
}

fn update_death_vignette(
    mut commands: Commands,
    mut vignette_query: Query<(Entity, &mut BackgroundColor, &DeathVignette)>,
    time: Res<Time>,
) {
    for (entity, mut background_color, vignette) in vignette_query.iter_mut() {
        let elapsed = time.elapsed_seconds() - vignette.start_time;
        let progress = (elapsed / vignette.duration).min(1.0);

        if progress >= 1.0 {
            // Remove the vignette effect
            commands.entity(entity).despawn_recursive();
            info!("Death vignette effect ended");
        } else {
            // Create a dramatic pulsing red vignette effect
            // Start with rapid pulses, then fade to solid red
            let pulse_speed = 8.0 - (progress * 6.0); // Slower pulses over time
            let base_intensity = progress * 0.3; // Base red tint that increases over time
            let pulse_intensity = (elapsed * pulse_speed).sin().abs() * 0.5 * (1.0 - progress * 0.5);
            let total_intensity = (base_intensity + pulse_intensity).min(vignette.max_intensity);

            background_color.0 = Color::rgba(1.0, 0.0, 0.0, total_intensity);
        }
    }
}

fn handle_player_respawn(
    mut player_query: Query<(&mut Transform, &mut Velocity, &mut Player)>,
    vignette_query: Query<&DeathVignette>,
    mut spawn_events: EventWriter<PlayerSpawnEvent>,
    time: Res<Time>,
) {
    // Check if there's an active vignette effect that's near completion
    for vignette in vignette_query.iter() {
        let elapsed = time.elapsed_seconds() - vignette.start_time;
        let progress = elapsed / vignette.duration;

        if progress >= 0.8 { // Start respawn near end of vignette
            let spawn_position = Vec3::new(0.0, 7.0, 0.0);

            // Reset player position and velocity
            if let Ok((mut transform, mut velocity, mut player)) = player_query.get_single_mut() {
                transform.translation = spawn_position;
                velocity.linvel = Vec3::ZERO;
                velocity.angvel = Vec3::ZERO;
                player.is_falling = false; // Reset the falling flag

                spawn_events.send(PlayerSpawnEvent {
                    position: spawn_position,
                });

                info!("Player respawned at {:?}", spawn_position);
                break;
            }
        }
    }
}