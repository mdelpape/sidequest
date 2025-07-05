use bevy::prelude::*;

#[derive(Component)]
pub struct FollowLight {
    pub offset: Vec3,
}

#[derive(Component)]
pub struct FloorLight {
    pub light_type: FloorLightType,
    pub intensity: f32,
}

#[derive(Component, Clone)]
pub enum FloorLightType {
    Spotlight,
    Point,
    Accent,
}