use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy_rapier3d::prelude::{Collider, Friction};
use crate::{
    events::*,
    resources::*,
    states::*,
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Playing), setup_fps_ui)
            .add_systems(Update, (
                handle_debug_events,
                update_debug_display,
                handle_debug_commands,
                update_fps_display,
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
    player_query: Query<&Transform, With<crate::Player>>,
    collider_query: Query<(&GlobalTransform, &Collider, &Friction), Without<crate::Player>>,
) {
    if !config.debug_mode && !config.show_colliders {
        return;
    }

    // Draw player position
    if let Ok(player_transform) = player_query.get_single() {
        gizmos.sphere(player_transform.translation, Quat::IDENTITY, 0.5, Color::RED);
    }

    // Draw collider boxes with different colors based on friction
    if config.show_colliders {
        for (global_transform, _collider, friction) in collider_query.iter() {
            let color = if friction.coefficient > 0.5 {
                Color::GREEN // High friction (top surfaces)
            } else {
                Color::ORANGE // Low/no friction (side surfaces)
            };

            let world_transform = Transform {
                translation: global_transform.translation(),
                rotation: global_transform.to_scale_rotation_translation().1,
                scale: global_transform.to_scale_rotation_translation().0,
            };

            // Draw collider wireframe - simplified approach
            gizmos.cuboid(world_transform, color);
        }
    }
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

// Component for the FPS text entity
#[derive(Component)]
struct FpsText;

fn setup_fps_ui(mut commands: Commands) {
    // Create FPS counter UI
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(15.0),
                top: Val::Px(15.0),
                ..default()
            },
            ..default()
        },
    )).with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                "FPS: --",
                TextStyle {
                    font: default(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ).with_style(Style {
                ..default()
            }),
            FpsText,
        ));
    });
}

fn update_fps_display(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            for mut text in query.iter_mut() {
                text.sections[0].value = format!("FPS: {:.0}", value);
            }
        }
    }
}

