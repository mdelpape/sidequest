use bevy::prelude::*;
use serde::{Deserialize, Serialize};

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
    pub coins_collected: u32,
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

#[derive(Resource, Default, Clone)]
pub struct PlayerAnimations {
    pub walk: Handle<AnimationClip>,
    pub air: Handle<AnimationClip>,
    pub idle: Handle<AnimationClip>,
    pub front_flip: Handle<AnimationClip>,
    pub dive_roll: Handle<AnimationClip>,
}

// Preloaded animations for both characters
#[derive(Resource)]
pub struct PreloadedAnimations {
    pub boss3: PlayerAnimations,
    pub sword_hero: PlayerAnimations,
}

impl Default for PreloadedAnimations {
    fn default() -> Self {
        Self {
            boss3: PlayerAnimations::default(),
            sword_hero: PlayerAnimations::default(),
        }
    }
}

// Preloaded character models
#[derive(Resource)]
pub struct PreloadedCharacterModels {
    pub boss3: Handle<Scene>,
    pub sword_hero: Handle<Scene>,
}

impl Default for PreloadedCharacterModels {
    fn default() -> Self {
        Self {
            boss3: Handle::default(),
            sword_hero: Handle::default(),
        }
    }
}

// Input Configuration
#[derive(Resource, Clone)]
pub struct InputConfig {
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub jump: KeyCode,
    pub front_flip: KeyCode,
    pub dive_roll: KeyCode,
    pub pause: KeyCode,
    pub debug_toggle: KeyCode,
    pub camera_toggle: KeyCode,
    pub camera_forward: KeyCode,
    pub camera_backward: KeyCode,
    pub camera_left: KeyCode,
    pub camera_right: KeyCode,
    pub camera_up: KeyCode,
    pub camera_down: KeyCode,
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
            camera_toggle: KeyCode::C,
            camera_forward: KeyCode::I,
            camera_backward: KeyCode::K,
            camera_left: KeyCode::J,
            camera_right: KeyCode::L,
            camera_up: KeyCode::U,
            camera_down: KeyCode::O,
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

// Character Selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterType {
    Boss3,
    SwordHero,
}

impl CharacterType {
    pub fn model_path(&self) -> &'static str {
        match self {
            CharacterType::Boss3 => "boss3.glb#Scene0",
            CharacterType::SwordHero => "swordHero.glb#Scene0",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            CharacterType::Boss3 => "Boss Character",
            CharacterType::SwordHero => "Sword Hero",
        }
    }
}

#[derive(Resource)]
pub struct SelectedCharacter {
    pub character_type: CharacterType,
}

impl Default for SelectedCharacter {
    fn default() -> Self {
        Self {
            character_type: CharacterType::Boss3,
        }
    }
}

// Authentication Resources
#[derive(Resource, Default, Clone, Serialize, Deserialize)]
pub struct UserData {
    pub user_id: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
    pub player_stats: PlayerStats,
    pub preferences: UserPreferences,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PlayerStats {
    pub level: u32,
    pub total_play_time: f32,
    pub high_score: u32,
    pub achievements: Vec<String>,
    pub unlocked_characters: Vec<String>,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            level: 1,
            total_play_time: 0.0,
            high_score: 0,
            achievements: Vec::new(),
            unlocked_characters: vec!["Boss3".to_string()], // Default character
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub master_volume: f32,
    pub sfx_volume: f32,
    pub music_volume: f32,
    pub camera_sensitivity: f32,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            sfx_volume: 0.8,
            music_volume: 0.6,
            camera_sensitivity: 2.0,
        }
    }
}

#[derive(Resource, Default)]
pub struct AuthSession {
    pub is_authenticated: bool,
    pub session_token: Option<String>,
    pub expires_at: Option<std::time::SystemTime>,
}

impl AuthSession {
    pub fn is_valid(&self) -> bool {
        self.is_authenticated &&
        self.session_token.is_some() &&
        self.expires_at.map_or(false, |exp| exp > std::time::SystemTime::now())
    }
}

#[derive(Resource)]
pub struct AuthConfig {
    pub api_base_url: String,
    pub session_duration: std::time::Duration,
    pub auto_save_interval: std::time::Duration,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            api_base_url: "https://api.yourgame.com".to_string(), // Replace with your actual API
            session_duration: std::time::Duration::from_secs(24 * 60 * 60), // 24 hours
            auto_save_interval: std::time::Duration::from_secs(30), // 30 seconds
        }
    }
}

#[derive(Resource, Default)]
pub struct AuthFormData {
    pub email: String,
    pub password: String,
    pub confirm_password: String,
    pub username: String,
    pub error_message: Option<String>,
    pub is_loading: bool,
}

impl AuthFormData {
    pub fn clear(&mut self) {
        self.email.clear();
        self.password.clear();
        self.confirm_password.clear();
        self.username.clear();
        self.error_message = None;
        self.is_loading = false;
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
            .init_resource::<LoadingProgress>()
            .init_resource::<SelectedCharacter>()
            .init_resource::<PreloadedAnimations>()
            .init_resource::<PreloadedCharacterModels>()
            // Authentication resources
            .init_resource::<UserData>()
            .init_resource::<AuthSession>()
            .init_resource::<AuthConfig>()
            .init_resource::<AuthFormData>();
    }
}