use bevy::prelude::*;

// Game Configuration
#[derive(Resource)]
pub struct GameConfig {
    pub debug_mode: bool,
    pub show_colliders: bool,
    pub physics_debug: bool,
    pub camera_sensitivity: f32,
    pub master_volume: f32,
    pub sfx_volume: f32,
    pub music_volume: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            debug_mode: cfg!(debug_assertions),
            show_colliders: false,
            physics_debug: false,
            camera_sensitivity: 2.0,
            master_volume: 1.0,
            sfx_volume: 0.8,
            music_volume: 0.6,
        }
    }
}

// Game Statistics
#[derive(Resource, Default)]
pub struct GameStats {
    pub play_time: f32,
    pub jump_count: u32,
    pub flip_count: u32,
    pub fall_count: u32,
    pub platform_touches: u32,
    pub max_height: f32,
    pub total_distance: f32,
}

// Performance Metrics
#[derive(Resource, Default)]
pub struct PerformanceMetrics {
    pub fps: f32,
    pub frame_time: f32,
    pub entity_count: u32,
    pub system_count: u32,
}

// Asset Management
#[derive(Resource)]
pub struct GameAssets {
    pub player_scene: Handle<Scene>,
    pub player_animations: PlayerAnimations,
    pub skybox_texture: Handle<Image>,
    pub audio_jump: Handle<AudioSource>,
    pub audio_land: Handle<AudioSource>,
    pub audio_flip: Handle<AudioSource>,
}

impl Default for GameAssets {
    fn default() -> Self {
        Self {
            player_scene: Handle::default(),
            player_animations: PlayerAnimations::default(),
            skybox_texture: Handle::default(),
            audio_jump: Handle::default(),
            audio_land: Handle::default(),
            audio_flip: Handle::default(),
        }
    }
}



#[derive(Resource, Default)]
pub struct PlayerAnimations {
    pub walk: Handle<AnimationClip>,
    pub air: Handle<AnimationClip>,
    pub idle: Handle<AnimationClip>,
    pub front_flip: Handle<AnimationClip>,
    pub dive_roll: Handle<AnimationClip>,
}

// Input Configuration
#[derive(Resource)]
pub struct InputConfig {
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub jump: KeyCode,
    pub front_flip: KeyCode,
    pub dive_roll: KeyCode,
    pub pause: KeyCode,
    pub debug_toggle: KeyCode,
}

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            move_left: KeyCode::A,
            move_right: KeyCode::D,
            jump: KeyCode::Space,
            front_flip: KeyCode::W,
            dive_roll: KeyCode::S,
            pause: KeyCode::Escape,
            debug_toggle: KeyCode::F3,
        }
    }
}

// Loading Progress
#[derive(Resource, Default)]
pub struct LoadingProgress {
    pub total_assets: usize,
    pub loaded_assets: usize,
    pub loading_stage: String,
}

impl LoadingProgress {
    pub fn progress(&self) -> f32 {
        if self.total_assets == 0 {
            0.0
        } else {
            self.loaded_assets as f32 / self.total_assets as f32
        }
    }
}

// Plugin to register all resources
pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameConfig>()
            .init_resource::<GameStats>()
            .init_resource::<PerformanceMetrics>()
            .init_resource::<InputConfig>()
            .init_resource::<LoadingProgress>();
    }
}