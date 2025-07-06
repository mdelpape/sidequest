use bevy::prelude::*;
use crate::{
    components::*,
    states::*,
    resources::*,
    events::*,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(PlayState::Setup), setup_camera)
            .add_systems(Update, (
                handle_camera_toggle,
                update_camera_follow,
                update_free_camera,
                handle_camera_shake,
            ).run_if(in_state(GameState::Playing)));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.0, 12.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        },
        FollowCamera {
            offset: Vec3::new(0.0, 2.0, 8.0),
            lerp_speed: 10.0,
        },
        FreeCamera::default(),
        CameraController::default(),
        CameraShake::default(),
        Name::new("MainCamera"),
    ));

    info!("Camera setup complete");
}

fn handle_camera_toggle(
    keyboard: Res<Input<KeyCode>>,
    input_config: Res<InputConfig>,
    mut camera_query: Query<(&mut FreeCamera, &mut CameraController, &Transform)>,
    player_query: Query<&Transform, (With<Player>, Without<FreeCamera>)>,
    mut debug_events: EventWriter<DebugEvent>,
) {
    if keyboard.just_pressed(input_config.camera_toggle) {
        for (mut free_camera, mut controller, camera_transform) in camera_query.iter_mut() {
            free_camera.is_active = !free_camera.is_active;

            if free_camera.is_active {
                // Store current camera position when entering free camera mode
                controller.target_position = camera_transform.translation;
                debug_events.send(DebugEvent {
                    message: "Free camera mode ON - Use IJKL to move, UO to zoom".to_string(),
                });
            } else {
                // Reset to follow mode
                if let Ok(player_transform) = player_query.get_single() {
                    controller.target_position = player_transform.translation;
                }
                debug_events.send(DebugEvent {
                    message: "Free camera mode OFF - Following player".to_string(),
                });
            }
        }
    }
}

fn update_free_camera(
    keyboard: Res<Input<KeyCode>>,
    input_config: Res<InputConfig>,
    mut camera_query: Query<(&mut Transform, &mut FreeCamera, &mut CameraController)>,
    time: Res<Time>,
    mut scroll_events: EventReader<bevy::input::mouse::MouseWheel>,
) {
    for (mut camera_transform, mut free_camera, mut controller) in camera_query.iter_mut() {
        if !free_camera.is_active {
            continue;
        }

        let delta_time = time.delta_seconds();
        let movement_speed = free_camera.movement_speed * delta_time;
        let mut movement = Vec3::ZERO;

        // Handle keyboard movement
        if keyboard.pressed(input_config.camera_forward) {
            movement.z -= movement_speed;
        }
        if keyboard.pressed(input_config.camera_backward) {
            movement.z += movement_speed;
        }
        if keyboard.pressed(input_config.camera_left) {
            movement.x -= movement_speed;
        }
        if keyboard.pressed(input_config.camera_right) {
            movement.x += movement_speed;
        }
        if keyboard.pressed(input_config.camera_up) {
            movement.y += movement_speed;
        }
        if keyboard.pressed(input_config.camera_down) {
            movement.y -= movement_speed;
        }

        // Apply movement relative to camera's rotation
        let forward = camera_transform.forward();
        let right = camera_transform.right();
        let up = camera_transform.up();

        controller.target_position += forward * -movement.z + right * movement.x + up * movement.y;

        // Handle zoom (mouse wheel or keyboard)
        let mut zoom_delta = 0.0;
        for scroll in scroll_events.read() {
            zoom_delta -= scroll.y * free_camera.zoom_speed * delta_time;
        }

        if zoom_delta != 0.0 {
            free_camera.current_zoom = (free_camera.current_zoom + zoom_delta)
                .clamp(free_camera.min_zoom, free_camera.max_zoom);

            // Move camera forward/backward based on zoom
            let forward = camera_transform.forward();
            controller.target_position += forward * zoom_delta;
        }

        // Smoothly move camera to target position
        camera_transform.translation = camera_transform.translation.lerp(
            controller.target_position,
            10.0 * delta_time,
        );
    }
}

fn update_camera_follow(
    player_query: Query<&Transform, (With<Player>, Without<FollowCamera>)>,
    mut camera_query: Query<(&mut Transform, &FollowCamera, &mut CameraShake, &FreeCamera), (Without<Player>, With<FollowCamera>)>,
    time: Res<Time>,
) {
    let player_transform = if let Ok(transform) = player_query.get_single() {
        transform
    } else {
        return;
    };

    for (mut camera_transform, follow_camera, mut shake, free_camera) in camera_query.iter_mut() {
        // Skip following if in free camera mode
        if free_camera.is_active {
            continue;
        }

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