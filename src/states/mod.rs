use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    Playing,
    Paused,
    GameOver,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum PlayState {
    #[default]
    Setup,
    Playing,
    Transitioning,
}

#[derive(Component)]
pub struct StateCleanup;

pub fn cleanup_state<T: Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}