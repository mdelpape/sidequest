use bevy::prelude::*;

#[derive(Component)]
pub struct FollowCamera {
    pub offset: Vec3,
    pub lerp_speed: f32,
}

#[derive(Component)]
pub struct FreeCamera {
    pub is_active: bool,
    pub movement_speed: f32,
    pub rotation_speed: f32,
    pub zoom_speed: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub current_zoom: f32,
}

impl Default for FreeCamera {
    fn default() -> Self {
        Self {
            is_active: false,
            movement_speed: 10.0,
            rotation_speed: 1.0,
            zoom_speed: 5.0,
            min_zoom: 3.0,
            max_zoom: 50.0,
            current_zoom: 8.0,
        }
    }
}

#[derive(Component)]
pub struct CameraController {
    pub target_position: Vec3,
    pub rotation: Vec2, // pitch, yaw
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            target_position: Vec3::ZERO,
            rotation: Vec2::ZERO,
        }
    }
}