use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Loading,
    Authentication,
    CharacterSelection,
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

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AuthState {
    #[default]
    Login,
    SignUp,
    Authenticating,
    Authenticated,
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