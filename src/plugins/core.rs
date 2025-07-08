use bevy::prelude::*;
use crate::{
    resources::*,
    states::*,
};

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                init_game_config,
                setup_loading_screen,
            ))
            .add_systems(Update, (
                handle_game_state_transitions,
                update_game_stats,
                update_performance_metrics,
                update_loading_progress,
            ))
            .add_systems(OnEnter(GameState::Loading), (enter_loading_state, preload_animations))
            .add_systems(OnExit(GameState::Loading), exit_loading_state)
            .add_systems(OnEnter(GameState::Playing), enter_playing_state)
            .add_systems(OnExit(GameState::Playing), exit_playing_state)
            .add_systems(OnEnter(GameState::CharacterSelection), reset_game_on_restart);
    }
}

fn init_game_config(mut commands: Commands) {
    commands.insert_resource(GameConfig::default());
    commands.insert_resource(InputConfig::default());
    info!("Game configuration initialized");
}

fn setup_loading_screen(mut commands: Commands) {
    // Create loading screen UI
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::rgba(0.1, 0.1, 0.1, 0.9).into(),
            ..default()
        },
        StateCleanup,
        Name::new("LoadingScreen"),
    )).with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                "Loading...",
                TextStyle {
                    font_size: 40.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            Name::new("LoadingText"),
        ));
    });
}

fn update_loading_progress(
    mut loading_progress: ResMut<LoadingProgress>,
    time: Res<Time>,
    current_state: Res<State<GameState>>,
) {
    if *current_state.get() == GameState::Loading {
        // Simulate loading progress over 2 seconds
        loading_progress.loaded_assets = (time.elapsed_seconds() * 50.0).min(100.0) as usize;
        loading_progress.total_assets = 100;

        // Update loading stage based on progress
        let progress = loading_progress.progress();
        loading_progress.loading_stage = if progress < 0.3 {
            "Loading assets...".to_string()
        } else if progress < 0.6 {
            "Initializing systems...".to_string()
        } else if progress < 0.9 {
            "Setting up world...".to_string()
        } else {
            "Ready!".to_string()
        };
    }
}

fn handle_game_state_transitions(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard: Res<Input<KeyCode>>,
    current_state: Res<State<GameState>>,
    loading_progress: Res<LoadingProgress>,
) {
    match current_state.get() {
        GameState::Loading => {
            if loading_progress.progress() >= 1.0 {
                next_state.set(GameState::Authentication);
            }
        }
        GameState::CharacterSelection => {
            // Character selection handles its own transition to Playing
        }
        GameState::Playing => {
            if keyboard.just_pressed(KeyCode::Escape) {
                next_state.set(GameState::Paused);
            }
        }
        GameState::Paused => {
            if keyboard.just_pressed(KeyCode::Escape) {
                next_state.set(GameState::Playing);
            }
        }
        _ => {}
    }
}

fn update_game_stats(
    mut stats: ResMut<GameStats>,
    time: Res<Time>,
    current_state: Res<State<GameState>>,
) {
    if *current_state.get() == GameState::Playing {
        stats.play_time += time.delta_seconds();
    }
}

fn update_performance_metrics(
    mut metrics: ResMut<PerformanceMetrics>,
    diagnostics: Res<bevy::diagnostic::DiagnosticsStore>,
) {
    if let Some(fps) = diagnostics.get(bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            metrics.fps = value as f32;
        }
    }

    if let Some(frame_time) = diagnostics.get(bevy::diagnostic::FrameTimeDiagnosticsPlugin::FRAME_TIME) {
        if let Some(value) = frame_time.smoothed() {
            metrics.frame_time = value as f32;
        }
    }
}

fn enter_loading_state(mut loading_progress: ResMut<LoadingProgress>) {
    loading_progress.loading_stage = "Initializing...".to_string();
    loading_progress.loaded_assets = 0;
    loading_progress.total_assets = 100;
    info!("Entered loading state");
}

fn exit_loading_state(mut commands: Commands, query: Query<Entity, With<StateCleanup>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    info!("Exited loading state");
}

fn enter_playing_state(mut play_state: ResMut<NextState<PlayState>>) {
    play_state.set(PlayState::Setup);
    info!("Entered playing state");
}

fn exit_playing_state(
    mut commands: Commands,
    query: Query<Entity, With<StateCleanup>>,
    player_query: Query<Entity, With<crate::Player>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Despawn player when exiting playing state
    for player_entity in player_query.iter() {
        commands.entity(player_entity).despawn_recursive();
        info!("Player despawned when exiting playing state");
    }

    info!("Exited playing state");
}

fn preload_animations(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    info!("=== PRELOADING ASSETS ===");

    // Preload character models
    let boss3_scene = asset_server.load("boss3.glb#Scene0");
    let sword_hero_scene = asset_server.load("swordHero.glb#Scene0");

    commands.insert_resource(PreloadedCharacterModels {
        boss3: boss3_scene,
        sword_hero: sword_hero_scene,
    });

    // Preload animations for Boss3
    let boss3_animations = PlayerAnimations {
        walk: asset_server.load("boss3.glb#Animation9"),
        air: asset_server.load("boss3.glb#Animation0"),
        idle: asset_server.load("boss3.glb#Animation6"),
        front_flip: asset_server.load("boss3.glb#Animation3"),
        dive_roll: asset_server.load("boss3.glb#Animation4"),
    };

    // Preload animations for SwordHero
    let sword_hero_animations = PlayerAnimations {
        walk: asset_server.load("swordHero.glb#Animation8"),
        air: asset_server.load("swordHero.glb#Animation1"),
        idle: asset_server.load("swordHero.glb#Animation5"),
        front_flip: asset_server.load("swordHero.glb#Animation2"),
        dive_roll: asset_server.load("swordHero.glb#Animation4"),
    };

    commands.insert_resource(PreloadedAnimations {
        boss3: boss3_animations,
        sword_hero: sword_hero_animations,
    });

    info!("Boss3 and SwordHero models and animations preloaded during Loading state");
    info!("=== PRELOADING COMPLETE ===");
}

fn reset_game_on_restart(
    mut stats: ResMut<GameStats>,
    coin_query: Query<Entity, With<crate::plugins::physics::Coin>>,
    _current_state: Res<State<GameState>>,
) {
    // Only reset if we're coming from a restart (not the normal loading flow)
    // We can detect this by checking if there are coins in the world
    if !coin_query.is_empty() {
        info!("=== RESETTING GAME FOR RESTART ===");

        // Reset game stats but keep coins as they are
        *stats = GameStats::default();

        info!("Game reset complete - stats reset, coins preserved");
    }
}