use bevy::prelude::*;
use crate::components::{Player, FollowCamera};

pub fn follow_camera(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<(&mut Transform, &FollowCamera), Without<Player>>,
    time: Res<Time>,
) {
    // Get the player position
    let player_transform = if let Ok(transform) = player_query.get_single() {
        transform
    } else {
        return;
    };

    // Update camera position
    for (mut camera_transform, follow_camera) in camera_query.iter_mut() {
        // Calculate target position
        let target_position = Vec3::new(
            player_transform.translation.x + follow_camera.offset.x,
            player_transform.translation.y + follow_camera.offset.y,
            camera_transform.translation.z, // Keep z constant
        );

        // Smoothly interpolate to target position
        camera_transform.translation = camera_transform.translation.lerp(
            target_position,
            follow_camera.lerp_speed * time.delta_seconds()
        );
    }
}