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
                            Vec3::new(-0.5, 0.0, 0.0)
                        } else {
                            Vec3::new(0.5, 0.0, 0.0)
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