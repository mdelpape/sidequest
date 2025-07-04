use bevy::prelude::*;
use crate::{
    events::*,
    resources::*,
    states::*,
};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, load_audio_assets)
            .add_systems(Update, (
                handle_audio_events,
                update_audio_settings,
            ).run_if(in_state(GameState::Playing)));
    }
}

fn load_audio_assets(
    _asset_server: Res<AssetServer>,
) {
    // Load audio files (placeholder for now)
    info!("Audio assets loaded");
}

fn handle_audio_events(
    mut jump_events: EventReader<PlayerJumpEvent>,
    mut flip_events: EventReader<PlayerFlipEvent>,
    config: Res<GameConfig>,
) {
    for _event in jump_events.read() {
        // Play jump sound
        if config.sfx_volume > 0.0 {
            info!("Playing jump sound");
        }
    }

    for _event in flip_events.read() {
        // Play flip sound
        if config.sfx_volume > 0.0 {
            info!("Playing flip sound");
        }
    }
}

fn update_audio_settings(
    config: Res<GameConfig>,
) {
    // Update audio settings based on config
    if config.is_changed() {
        info!("Audio settings updated");
    }
}