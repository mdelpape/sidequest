use bevy::prelude::*;
use crate::components::{Boss, FollowCamera};

pub fn follow_camera(
    boss_query: Query<&Transform, With<Boss>>,
    mut camera_query: Query<(&mut Transform, &FollowCamera), Without<Boss>>,
    time: Res<Time>,
) {
    // Get the boss position
    let boss_transform = if let Ok(transform) = boss_query.get_single() {
        transform
    } else {
        return;
    };

    // Update camera position
    for (mut camera_transform, follow_camera) in camera_query.iter_mut() {
        // Calculate target position
        let target_position = Vec3::new(
            boss_transform.translation.x + follow_camera.offset.x,
            boss_transform.translation.y + follow_camera.offset.y,
            camera_transform.translation.z, // Keep z constant
        );

        // Smoothly interpolate to target position
        camera_transform.translation = camera_transform.translation.lerp(
            target_position,
            follow_camera.lerp_speed * time.delta_seconds()
        );
    }
}