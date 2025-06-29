use bevy::prelude::*;

#[derive(Component)]
pub struct FollowCamera {
    pub offset: Vec3,
    pub lerp_speed: f32,
}