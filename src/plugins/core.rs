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
            .add_systems(OnEnter(GameState::Loading), enter_loading_state)
            .add_systems(OnExit(GameState::Loading), exit_loading_state)
            .add_systems(OnEnter(GameState::Playing), enter_playing_state)
            .add_systems(OnExit(GameState::Playing), exit_playing_state);
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
                next_state.set(GameState::Playing);
            }
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

fn exit_playing_state(mut commands: Commands, query: Query<Entity, With<StateCleanup>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    info!("Exited playing state");
}