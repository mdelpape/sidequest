use bevy::prelude::*;

#[derive(Component)]
pub struct Boss {
    pub speed: f32,
    pub is_moving: bool,
    pub is_grounded: bool,
    pub is_front_flipping: bool,
    pub is_dive_rolling: bool,
    pub flip_direction: Vec3,
    pub facing_left: bool,
}