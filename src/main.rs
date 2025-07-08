use bevy::prelude::*;
use bevy::log::LogPlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_rapier3d::prelude::*;
use bevy_egui::EguiPlugin;
use sidequest::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                filter: "wgpu=error,bevy_render=info,bevy_gltf=error".to_string(),
                level: bevy::log::Level::INFO,
                ..default()
            }),
            EguiPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            // RapierDebugRenderPlugin::default(),
            FrameTimeDiagnosticsPlugin,
            GamePlugin,
        ))
        .add_systems(OnEnter(GameState::Playing), init_animations)
        .run();
}

fn init_animations(
    mut commands: Commands,
    selected_character: Res<SelectedCharacter>,
    preloaded_animations: Res<PreloadedAnimations>,
) {
    info!("=== SETTING UP ANIMATIONS ===");
    info!("Using preloaded animations for character: {:?}", selected_character.character_type);

    let animations = match selected_character.character_type {
        CharacterType::Boss3 => {
            info!("Using preloaded Boss3 animations");
            preloaded_animations.boss3.clone()
        },
        CharacterType::SwordHero => {
            info!("Using preloaded SwordHero animations");
            preloaded_animations.sword_hero.clone()
        },
    };

    commands.insert_resource(animations);
    info!("=== ANIMATIONS READY ===");
}
