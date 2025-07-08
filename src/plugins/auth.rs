use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::states::{GameState, AuthState};
use crate::resources::{AuthSession, AuthFormData, UserData, AuthConfig, PlayerStats};
use crate::events::{AuthRequestEvent, AuthResponseEvent, AuthRequestType};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

pub struct AuthPlugin;

impl Plugin for AuthPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<AuthState>()
            .add_systems(OnEnter(GameState::Authentication), setup_auth_ui)
            .add_systems(OnExit(GameState::Authentication), cleanup_auth_ui)
            .add_systems(Update, (
                auth_ui_system,
                handle_auth_response,
                check_session_validity,
            ).run_if(in_state(GameState::Authentication)))
            .add_systems(Update, auto_save_player_data.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct AuthUiCleanup;

#[derive(Serialize, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct SignUpRequest {
    email: String,
    password: String,
    username: String,
}

#[derive(Serialize, Deserialize)]
struct AuthResponse {
    success: bool,
    message: String,
    user_data: Option<UserData>,
    session_token: Option<String>,
}

fn setup_auth_ui(mut commands: Commands) {
    info!("Setting up authentication UI");
    commands.insert_resource(AuthFormData::default());
}

fn cleanup_auth_ui(
    mut commands: Commands,
    query: Query<Entity, With<AuthUiCleanup>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    commands.remove_resource::<AuthFormData>();
}

fn auth_ui_system(
    mut contexts: EguiContexts,
    mut auth_form: ResMut<AuthFormData>,
    mut auth_state: ResMut<NextState<AuthState>>,
    mut game_state: ResMut<NextState<GameState>>,
    current_auth_state: Res<State<AuthState>>,
    mut auth_session: ResMut<AuthSession>,
    mut commands: Commands,
) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.add_space(100.0);

            // Title
            ui.heading("SideQuest Authentication");
            ui.add_space(40.0);

            // Auth state tabs
            ui.horizontal(|ui| {
                                if ui.selectable_label(
                    current_auth_state.get() == &AuthState::Login,
                    "Login"
                ).clicked() {
                    auth_state.set(AuthState::Login);
                    auth_form.clear();
                }

                if ui.selectable_label(
                    current_auth_state.get() == &AuthState::SignUp,
                    "Sign Up"
                ).clicked() {
                    auth_state.set(AuthState::SignUp);
                    auth_form.clear();
                }
            });

            ui.add_space(20.0);

            // Error message
            if let Some(error) = &auth_form.error_message {
                ui.colored_label(egui::Color32::RED, error);
                ui.add_space(10.0);
            }

            // Form fields
            ui.add_sized([300.0, 20.0], egui::TextEdit::singleline(&mut auth_form.email)
                .hint_text("Email"));
            ui.add_space(10.0);

            ui.add_sized([300.0, 20.0], egui::TextEdit::singleline(&mut auth_form.password)
                .password(true)
                .hint_text("Password"));
            ui.add_space(10.0);

            // Sign up specific fields
            if current_auth_state.get() == &AuthState::SignUp {
                ui.add_sized([300.0, 20.0], egui::TextEdit::singleline(&mut auth_form.confirm_password)
                    .password(true)
                    .hint_text("Confirm Password"));
                ui.add_space(10.0);

                ui.add_sized([300.0, 20.0], egui::TextEdit::singleline(&mut auth_form.username)
                    .hint_text("Username"));
                ui.add_space(10.0);
            }

            ui.add_space(20.0);

                        // Action buttons
            ui.horizontal(|ui| {
                let button_text = match current_auth_state.get() {
                    AuthState::Login => "Login",
                    AuthState::SignUp => "Sign Up",
                    AuthState::Authenticating => "Authenticating...",
                    AuthState::Authenticated => "Authenticated",
                };

                let button_enabled = !auth_form.is_loading &&
                    !auth_form.email.is_empty() &&
                    !auth_form.password.is_empty() &&
                    (current_auth_state.get() == &AuthState::Login ||
                     (!auth_form.username.is_empty() && auth_form.password == auth_form.confirm_password));

                if ui.add_enabled(button_enabled, egui::Button::new(button_text)).clicked() {
                    if current_auth_state.get() == &AuthState::Login {
                        handle_login(&mut auth_form, &mut auth_state, &mut commands);
                    } else if current_auth_state.get() == &AuthState::SignUp {
                        handle_signup(&mut auth_form, &mut auth_state, &mut commands);
                    }
                }

                if ui.button("Skip (Demo Mode)").clicked() {
                    // For demo purposes, allow skipping auth
                    auth_session.is_authenticated = true;
                    auth_session.session_token = Some("demo_token".to_string());
                    auth_session.expires_at = Some(SystemTime::now() + std::time::Duration::from_secs(3600));

                    // Create demo user data
                    let demo_user = UserData {
                        user_id: Some("demo_user".to_string()),
                        email: Some("demo@example.com".to_string()),
                        username: Some("DemoPlayer".to_string()),
                        player_stats: PlayerStats::default(),
                        preferences: crate::resources::UserPreferences::default(),
                    };
                    commands.insert_resource(demo_user);

                    game_state.set(GameState::CharacterSelection);
                }
            });
        });
    });
}

fn handle_login(
    auth_form: &mut AuthFormData,
    auth_state: &mut NextState<AuthState>,
    commands: &mut Commands,
) {
    if auth_form.email.is_empty() || auth_form.password.is_empty() {
        auth_form.error_message = Some("Please fill in all fields".to_string());
        return;
    }

    auth_form.is_loading = true;
    auth_form.error_message = None;
    auth_state.set(AuthState::Authenticating);

    // Send login request event
    commands.add(|world: &mut World| {
        let event = AuthRequestEvent {
            request_type: AuthRequestType::Login,
            email: world.resource::<AuthFormData>().email.clone(),
            password: world.resource::<AuthFormData>().password.clone(),
            username: None,
        };

        // For now, simulate a successful login
        let response = AuthResponseEvent {
            success: true,
            message: "Login successful!".to_string(),
            user_data: Some(UserData {
                user_id: Some("user_123".to_string()),
                email: Some(event.email.clone()),
                username: Some("Player".to_string()),
                player_stats: PlayerStats::default(),
                preferences: crate::resources::UserPreferences::default(),
            }),
            session_token: Some("session_token_123".to_string()),
        };

        world.send_event(response);
    });
}

fn handle_signup(
    auth_form: &mut AuthFormData,
    auth_state: &mut NextState<AuthState>,
    commands: &mut Commands,
) {
    if auth_form.email.is_empty() || auth_form.password.is_empty() || auth_form.username.is_empty() {
        auth_form.error_message = Some("Please fill in all fields".to_string());
        return;
    }

    if auth_form.password != auth_form.confirm_password {
        auth_form.error_message = Some("Passwords do not match".to_string());
        return;
    }

    if auth_form.password.len() < 6 {
        auth_form.error_message = Some("Password must be at least 6 characters".to_string());
        return;
    }

    auth_form.is_loading = true;
    auth_form.error_message = None;
    auth_state.set(AuthState::Authenticating);

    // Send signup request event
    commands.add(|world: &mut World| {
        let form_data = world.resource::<AuthFormData>();
        let event = AuthRequestEvent {
            request_type: AuthRequestType::SignUp,
            email: form_data.email.clone(),
            password: form_data.password.clone(),
            username: Some(form_data.username.clone()),
        };

        // For now, simulate a successful signup
        let response = AuthResponseEvent {
            success: true,
            message: "Account created successfully!".to_string(),
            user_data: Some(UserData {
                user_id: Some("user_123".to_string()),
                email: Some(event.email.clone()),
                username: event.username.clone(),
                player_stats: PlayerStats::default(),
                preferences: crate::resources::UserPreferences::default(),
            }),
            session_token: Some("session_token_123".to_string()),
        };

        world.send_event(response);
    });
}

fn handle_auth_response(
    mut auth_response_events: EventReader<AuthResponseEvent>,
    mut auth_form: ResMut<AuthFormData>,
    mut auth_state: ResMut<NextState<AuthState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut auth_session: ResMut<AuthSession>,
    mut commands: Commands,
) {
    for event in auth_response_events.read() {
        auth_form.is_loading = false;

        if event.success {
            auth_session.is_authenticated = true;
            auth_session.session_token = event.session_token.clone();
            auth_session.expires_at = Some(SystemTime::now() + std::time::Duration::from_secs(3600));

            if let Some(user_data) = &event.user_data {
                commands.insert_resource(user_data.clone());
            }

            auth_state.set(AuthState::Authenticated);
            game_state.set(GameState::CharacterSelection);

            info!("Authentication successful: {}", event.message);
        } else {
            auth_form.error_message = Some(event.message.clone());
            auth_state.set(AuthState::Login);
            warn!("Authentication failed: {}", event.message);
        }
    }
}

fn check_session_validity(
    auth_session: Res<AuthSession>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if !auth_session.is_valid() && auth_session.is_authenticated {
        warn!("Session expired, redirecting to authentication");
        game_state.set(GameState::Authentication);
    }
}

fn auto_save_player_data(
    time: Res<Time>,
    mut last_save: Local<f32>,
    user_data: Res<UserData>,
    auth_config: Res<AuthConfig>,
    auth_session: Res<AuthSession>,
) {
    if !auth_session.is_valid() {
        return;
    }

    *last_save += time.delta_seconds();

    if *last_save >= auth_config.auto_save_interval.as_secs_f32() {
        *last_save = 0.0;

        // In a real implementation, this would send the data to your backend
        info!("Auto-saving player data for user: {:?}", user_data.user_id);

        // Here you would serialize user_data and send it to your API
        // let serialized = serde_json::to_string(&*user_data).unwrap();
        // send_to_api(serialized);
    }
}

// Future implementation for real API calls
#[allow(dead_code)]
async fn send_login_request(config: &AuthConfig, email: String, password: String) -> Result<AuthResponse, String> {
    let client = reqwest::Client::new();
    let login_data = LoginRequest { email, password };

    let response = client
        .post(&format!("{}/auth/login", config.api_base_url))
        .json(&login_data)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if response.status().is_success() {
        response.json::<AuthResponse>().await
            .map_err(|e| format!("Failed to parse response: {}", e))
    } else {
        Err(format!("Login failed with status: {}", response.status()))
    }
}

#[allow(dead_code)]
async fn send_signup_request(config: &AuthConfig, email: String, password: String, username: String) -> Result<AuthResponse, String> {
    let client = reqwest::Client::new();
    let signup_data = SignUpRequest { email, password, username };

    let response = client
        .post(&format!("{}/auth/signup", config.api_base_url))
        .json(&signup_data)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if response.status().is_success() {
        response.json::<AuthResponse>().await
            .map_err(|e| format!("Failed to parse response: {}", e))
    } else {
        Err(format!("Signup failed with status: {}", response.status()))
    }
}