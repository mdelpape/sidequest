use bevy::prelude::*;
use crate::{
    components::*,
    states::*,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(PlayState::Setup), setup_camera)
            .add_systems(Update, (
                update_camera_follow,
                handle_camera_shake,
            ).run_if(in_state(GameState::Playing)));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.0, 8.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        },
        FollowCamera {
            offset: Vec3::new(0.0, 2.0, 8.0),
            lerp_speed: 10.0,
        },
        CameraShake::default(),
        Name::new("MainCamera"),
    ));

    info!("Camera setup complete");
}

fn update_camera_follow(
    player_query: Query<&Transform, (With<Player>, Without<FollowCamera>)>,
    mut camera_query: Query<(&mut Transform, &FollowCamera, &mut CameraShake), (Without<Player>, With<FollowCamera>)>,
    time: Res<Time>,
) {
    let player_transform = if let Ok(transform) = player_query.get_single() {
        transform
    } else {
        return;
    };

    for (mut camera_transform, follow_camera, mut shake) in camera_query.iter_mut() {
        let target_position = Vec3::new(
            player_transform.translation.x + follow_camera.offset.x,
            player_transform.translation.y + follow_camera.offset.y,
            camera_transform.translation.z,
        );

        // Apply shake offset
        let shake_offset = shake.get_offset(time.elapsed_seconds());
        let final_target = target_position + shake_offset;

        camera_transform.translation = camera_transform.translation.lerp(
            final_target,
            follow_camera.lerp_speed * time.delta_seconds(),
        );

        // Update shake
        shake.update(time.delta_seconds());
    }
}

fn handle_camera_shake(
    mut camera_query: Query<&mut CameraShake>,
    mut flip_events: EventReader<crate::events::PlayerFlipEvent>,
    mut jump_events: EventReader<crate::events::PlayerJumpEvent>,
) {
    let has_flip = !flip_events.is_empty();
    let has_jump = !jump_events.is_empty();

    // Clear the events
    flip_events.clear();
    jump_events.clear();

    for mut shake in camera_query.iter_mut() {
        if has_flip {
            shake.add_trauma(0.3);
        }
        if has_jump {
            shake.add_trauma(0.1);
        }
    }
}

#[derive(Component)]
pub struct CameraShake {
    pub trauma: f32,
    pub max_offset: f32,
    pub max_roll: f32,
    pub frequency: f32,
    pub decay: f32,
}

impl Default for CameraShake {
    fn default() -> Self {
        Self {
            trauma: 0.0,
            max_offset: 0.5,
            max_roll: 0.1,
            frequency: 30.0,
            decay: 3.0,
        }
    }
}

impl CameraShake {
    pub fn add_trauma(&mut self, amount: f32) {
        self.trauma = (self.trauma + amount).min(1.0);
    }

    pub fn get_offset(&self, time: f32) -> Vec3 {
        let shake = self.trauma * self.trauma;
        let offset_x = self.max_offset * shake * (self.frequency * time).sin();
        let offset_y = self.max_offset * shake * (self.frequency * time * 0.9).cos();
        Vec3::new(offset_x, offset_y, 0.0)
    }

    pub fn update(&mut self, delta_time: f32) {
        self.trauma = (self.trauma - self.decay * delta_time).max(0.0);
    }
}