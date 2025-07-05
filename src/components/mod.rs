mod camera;
mod player;
mod light;
mod skybox;

pub use camera::*;
pub use player::*;
pub use light::*;
pub use skybox::*;

use bevy::prelude::*;

// Death vignette effect component
#[derive(Component)]
pub struct DeathVignette {
    pub start_time: f32,
    pub duration: f32,
    pub max_intensity: f32,
}

impl Default for DeathVignette {
    fn default() -> Self {
        Self {
            start_time: 0.0,
            duration: 1.5, // 1.5 seconds
            max_intensity: 0.8,
        }
    }
}
