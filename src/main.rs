use bevy::prelude::*;
use bevy::log::LogPlugin;
use bevy_rapier3d::prelude::*;

mod components;
mod systems;

use systems::*;
use components::BossAnimations;

fn init_animations(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(BossAnimations {
        walk: asset_server.load("boss3.glb#Animation9"),
        air: asset_server.load("boss3.glb#Animation0"),
        idle: asset_server.load("boss3.glb#Animation6"),
        front_flip: asset_server.load("boss3.glb#Animation3"),
        dive_roll: asset_server.load("boss3.glb#Animation4"),
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "wgpu=error,bevy_render=info,bevy_gltf=error".to_string(),
            level: bevy::log::Level::INFO,
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(RapierDebugRenderPlugin::default()) // Removed debug rendering of colliders
        .add_systems(Startup, (
            init_animations,
            setup_camera,
            setup_platform,
            setup_lighting,
        ))
        .add_systems(Update, (
            spawn_boss,
            move_boss,
            control_animation,
            follow_camera,
            update_light_position,
            debug_animation_setup,
        ).chain())
        .run();
}
