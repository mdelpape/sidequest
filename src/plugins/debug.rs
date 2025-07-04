use bevy::prelude::*;
use crate::{
    events::*,
    resources::*,
    states::*,
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                handle_debug_events,
                update_debug_display,
                handle_debug_commands,
            ).run_if(in_state(GameState::Playing)));
    }
}

fn handle_debug_events(
    mut debug_events: EventReader<DebugEvent>,
) {
    for event in debug_events.read() {
        info!("Debug: {}", event.message);
    }
}

fn update_debug_display(
    config: Res<GameConfig>,
    mut gizmos: Gizmos,
    player_query: Query<&Transform, With<crate::Boss>>,
) {
    if !config.debug_mode {
        return;
    }

    // Draw player position
    if let Ok(player_transform) = player_query.get_single() {
        gizmos.sphere(player_transform.translation, Quat::IDENTITY, 0.5, Color::RED);
    }

    // You can add more debug visualizations here
}

fn handle_debug_commands(
    keyboard: Res<Input<KeyCode>>,
    mut debug_events: EventWriter<DebugEvent>,
    mut stats: ResMut<GameStats>,
) {
    if keyboard.just_pressed(KeyCode::F6) {
        stats.jump_count = 0;
        stats.flip_count = 0;
        stats.play_time = 0.0;
        debug_events.send(DebugEvent {
            message: "Stats reset".to_string(),
        });
    }
}