# SideQuest - 3D Parkour Game

A 3D parkour game built with Bevy Engine featuring character selection, physics-based movement, and user authentication.

## Features

### Core Gameplay
- **3D Parkour Movement**: Jump, flip, and dive-roll through challenging levels
- **Character Selection**: Choose between Boss3 and SwordHero characters
- **Physics-Based Gameplay**: Realistic physics using Rapier3D
- **Dynamic Camera**: Follow the player with smooth camera movement

### Authentication System
- **Email/Password Authentication**: Simple signup and login system
- **User Data Persistence**: Save player progress, stats, and preferences
- **Session Management**: Secure session handling with automatic expiration
- **Demo Mode**: Skip authentication for quick testing

### Player Data
- **Player Statistics**: Track level, play time, high score, and achievements
- **Character Unlocks**: Manage unlocked characters and progression
- **User Preferences**: Save volume settings and camera sensitivity
- **Auto-save**: Automatic saving of player data every 30 seconds

## Getting Started

### Prerequisites
- Rust 1.70+
- Cargo

### Installation
1. Clone the repository:
```bash
git clone <repository-url>
cd sidequest
```

2. Build the project:
```bash
cargo build --release
```

3. Run the game:
```bash
cargo run --release
```

## Architecture

### Plugin System
The game uses a modular plugin architecture:

- **AuthPlugin**: Handles user authentication and session management
- **CorePlugin**: Core game systems and state management
- **CharacterSelectionPlugin**: Character selection UI and logic
- **PlayerPlugin**: Player movement and controls
- **PhysicsPlugin**: Physics simulation and collision detection
- **RenderingPlugin**: 3D rendering and graphics
- **InputPlugin**: Input handling and key mapping

### State Management
The game uses Bevy's state system for different game phases:

- **Loading**: Asset loading and initialization
- **Authentication**: User login/signup
- **CharacterSelection**: Choose your character
- **Playing**: Main gameplay
- **Paused**: Game pause state

### Authentication Flow
1. **Loading**: Game assets are loaded
2. **Authentication**: User presented with login/signup UI
3. **Login/Signup**: User enters credentials
4. **Session Creation**: Valid credentials create a session
5. **Character Selection**: User chooses character
6. **Gameplay**: Main game begins

## Configuration

### API Integration
To connect to your backend API, update the `AuthConfig` in `src/resources/mod.rs`:

```rust
impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            api_base_url: "https://your-api.com".to_string(),
            session_duration: std::time::Duration::from_secs(24 * 60 * 60), // 24 hours
            auto_save_interval: std::time::Duration::from_secs(30), // 30 seconds
        }
    }
}
```

### Backend API Endpoints
Your backend should implement these endpoints:

- `POST /auth/login`: User login
  - Request: `{ "email": "user@example.com", "password": "password" }`
  - Response: `{ "success": true, "user_data": {...}, "session_token": "..." }`

- `POST /auth/signup`: User registration
  - Request: `{ "email": "user@example.com", "password": "password", "username": "player" }`
  - Response: `{ "success": true, "user_data": {...}, "session_token": "..." }`

## Development

### Building for Development
```bash
cargo build
```

### Running Tests
```bash
cargo test
```

### Debug Mode
The game includes debug features when running in development mode:
- Physics debug visualization
- Performance metrics
- Console logging

## Controls

- **WASD**: Move left/right, front flip, dive roll
- **Space**: Jump
- **C**: Toggle camera mode
- **F3**: Toggle debug overlay
- **Escape**: Pause/unpause

## Dependencies

### Core Game Engine
- **Bevy**: 3D game engine
- **Bevy Rapier3D**: Physics simulation
- **Bevy Egui**: UI framework

### Authentication
- **Reqwest**: HTTP client for API calls
- **Serde**: JSON serialization
- **Tokio**: Async runtime
- **BCrypt**: Password hashing (for future use)

### Utilities
- **UUID**: Unique identifiers
- **Bevy Mod Rounded Box**: UI styling

## Future Enhancements

- **Real API Integration**: Connect to actual backend services
- **Password Security**: Implement proper password hashing
- **Social Features**: Leaderboards and achievements
- **Level Editor**: Create and share custom levels
- **Multiplayer**: Race against other players
- **Mobile Support**: iOS and Android versions

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.