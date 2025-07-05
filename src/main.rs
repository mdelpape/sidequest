use bevy::prelude::*;
use bevy::log::LogPlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_rapier3d::prelude::*;
use sidequest::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                filter: "wgpu=error,bevy_render=info,bevy_gltf=error".to_string(),
                level: bevy::log::Level::INFO,
                ..default()
            }),
            RapierPhysicsPlugin::<NoUserData>::default(),
            // RapierDebugRenderPlugin::default(),
            FrameTimeDiagnosticsPlugin,
            GamePlugin,
        ))
        .add_systems(Startup, init_animations)
        .run();
}

fn init_animations(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PlayerAnimations {
        walk: asset_server.load("boss3.glb#Animation9"),
        air: asset_server.load("boss3.glb#Animation0"),
        idle: asset_server.load("boss3.glb#Animation6"),
        front_flip: asset_server.load("boss3.glb#Animation3"),
        dive_roll: asset_server.load("boss3.glb#Animation4"),
    });
}
