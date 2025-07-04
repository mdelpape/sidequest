use bevy::prelude::*;

// Core plugins
pub mod core;
pub mod player;
pub mod camera;
pub mod physics;
pub mod rendering;
pub mod input;
pub mod audio;
pub mod debug;

// Re-export plugins
pub use core::*;
pub use player::*;
pub use camera::*;
pub use physics::*;
pub use rendering::*;
pub use input::*;
pub use audio::*;
pub use debug::*;

// Main game plugin that orchestrates everything
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Add state management
            .add_state::<crate::states::GameState>()
            .add_state::<crate::states::PlayState>()

            // Add core plugins
            .add_plugins((
                crate::events::EventsPlugin,
                crate::resources::ResourcesPlugin,
                CorePlugin,
                InputPlugin,
                PlayerPlugin,
                CameraPlugin,
                PhysicsPlugin,
                RenderingPlugin,
                AudioPlugin,
                DebugPlugin,
            ));
    }
}