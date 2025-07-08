use bevy::prelude::*;
use crate::{
    events::*,
    resources::*,
    states::*,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                handle_debug_input,
                handle_player_input.run_if(in_state(GameState::Playing)),
            ));
    }
}

fn handle_debug_input(
    keyboard: Res<Input<KeyCode>>,
    mut config: ResMut<GameConfig>,
    input_config: Res<InputConfig>,
    mut debug_events: EventWriter<DebugEvent>,
) {
    if keyboard.just_pressed(input_config.debug_toggle) {
        config.debug_mode = !config.debug_mode;
        debug_events.send(DebugEvent {
            message: format!("Debug mode: {}", config.debug_mode),
        });
    }

    if keyboard.just_pressed(KeyCode::F4) {
        config.show_colliders = !config.show_colliders;
        if config.show_colliders {
            debug_events.send(DebugEvent {
                message: "Colliders ON: GREEN = High friction (tops), ORANGE = Low friction (sides)".to_string(),
            });
        } else {
            debug_events.send(DebugEvent {
                message: "Colliders OFF".to_string(),
            });
        }
    }

    if keyboard.just_pressed(KeyCode::F5) {
        config.physics_debug = !config.physics_debug;
        debug_events.send(DebugEvent {
            message: format!("Physics debug: {}", config.physics_debug),
        });
    }
}

fn handle_player_input(
    keyboard: Res<Input<KeyCode>>,
    input_config: Res<InputConfig>,
    mut player_events: EventWriter<PlayerMoveEvent>,
    mut jump_events: EventWriter<PlayerJumpEvent>,
    mut flip_events: EventWriter<PlayerFlipEvent>,
    mut next_state: ResMut<NextState<crate::states::GameState>>,
    player_query: Query<Entity, With<crate::Player>>,
) {
    // Check for restart key (i) - return to character selection
    if keyboard.just_pressed(KeyCode::I) {
        info!("Restart key pressed - returning to character selection");
        next_state.set(crate::states::GameState::CharacterSelection);
        return;
    }

    if let Ok(player_entity) = player_query.get_single() {
        // Movement input
        let mut direction = Vec3::ZERO;

        if keyboard.pressed(input_config.move_left) {
            direction.x -= 1.0;
        }
        if keyboard.pressed(input_config.move_right) {
            direction.x += 1.0;
        }

        if direction != Vec3::ZERO {
            player_events.send(PlayerMoveEvent {
                entity: player_entity,
                direction,
            });
        }

        // Jump input
        if keyboard.just_pressed(input_config.jump) {
            jump_events.send(PlayerJumpEvent {
                entity: player_entity,
            });
        }

        // Flip input
        if keyboard.just_pressed(input_config.front_flip) {
            flip_events.send(PlayerFlipEvent {
                entity: player_entity,
                flip_type: FlipType::Front,
            });
        }

        if keyboard.just_pressed(input_config.dive_roll) {
            flip_events.send(PlayerFlipEvent {
                entity: player_entity,
                flip_type: FlipType::Dive,
            });
        }
    }
}