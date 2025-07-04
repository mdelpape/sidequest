# Sidequest - Bevy ECS Game Architecture

A comprehensive Bevy ECS game built with a modular plugin architecture for easy development and maintenance.

## 🏗️ Architecture Overview

This project uses a sophisticated ECS (Entity Component System) architecture with the following structure:

### 📁 Project Structure

```
src/
├── lib.rs              # Main library entry point
├── main.rs             # Application entry point
├── components/         # ECS Components
│   ├── boss.rs         # Player/Boss components
│   ├── camera.rs       # Camera components
│   ├── light.rs        # Lighting components
│   ├── platform.rs     # Platform components
│   ├── skybox.rs       # Skybox components
│   └── mod.rs          # Component module exports
├── systems/            # Legacy systems (being phased out)
├── resources/          # ECS Resources
│   └── mod.rs          # Global game resources
├── events/             # ECS Events
│   └── mod.rs          # Event definitions
├── states/             # Game States
│   └── mod.rs          # State management
└── plugins/            # Plugin Architecture
    ├── core.rs         # Core game systems
    ├── player.rs       # Player/Boss management
    ├── camera.rs       # Camera systems
    ├── physics.rs      # Physics and platforms
    ├── rendering.rs    # Rendering and lighting
    ├── input.rs        # Input handling
    ├── audio.rs        # Audio systems
    ├── debug.rs        # Debug tools
    └── mod.rs          # Plugin orchestration
```

## 🎮 Game Features

### Player System
- **Movement**: WASD keys for movement
- **Jumping**: Space bar for jumping
- **Flips**: W for front flip, S for dive roll
- **Physics**: Rapier3D physics with collisions
- **Animations**: Multiple character animations

### Camera System
- **Follow Camera**: Smooth camera following
- **Camera Shake**: Dynamic shake effects on actions
- **Skybox**: Beautiful cube-mapped skybox

### Platform System
- **Multiple Types**: Ground, Floating, Small, Stepping Stones, Bridges
- **Physics Integration**: Full collision detection
- **Visual Variety**: Different colors for platform types

### Debug System
- **Debug Mode**: F3 to toggle debug information
- **Visual Debugging**: F4 for collider visualization
- **Physics Debug**: F5 for physics debug rendering
- **Stats Reset**: F6 to reset game statistics

## 🔧 Development Features

### Plugin Architecture
Each major system is organized as a plugin:

```rust
// Easy to add new features
app.add_plugins((
    CorePlugin,
    PlayerPlugin,
    CameraPlugin,
    PhysicsPlugin,
    RenderingPlugin,
    AudioPlugin,
    DebugPlugin,
));
```

### Event System
Decoupled communication between systems:

```rust
// Events for clean system communication
PlayerJumpEvent, PlayerFlipEvent, PlayerMoveEvent
AnimationStartEvent, AnimationEndEvent
DebugEvent, SystemErrorEvent
```

### Resource Management
Centralized configuration and state:

```rust
// Game configuration
GameConfig { debug_mode, show_colliders, volumes, ... }

// Statistics tracking
GameStats { play_time, jump_count, flip_count, ... }

// Performance monitoring
PerformanceMetrics { fps, frame_time, entity_count, ... }
```

### State Management
Proper game state handling:

```rust
#[derive(States)]
enum GameState {
    Loading,
    Playing,
    Paused,
    GameOver,
}

#[derive(States)]
enum PlayState {
    Setup,
    Playing,
    Transitioning,
}
```

## 🚀 Getting Started

### Prerequisites
- Rust 1.70+
- Cargo

### Running the Game
```bash
# Development mode
cargo run

# Release mode (better performance)
cargo run --release

# Check for errors
cargo check
```

### Controls
- **A/D**: Move left/right
- **Space**: Jump
- **W**: Front flip
- **S**: Dive roll
- **Escape**: Pause/Resume
- **F3**: Toggle debug mode
- **F4**: Toggle collider visualization
- **F5**: Toggle physics debug
- **F6**: Reset statistics

## 🛠️ Development

### Adding New Features

1. **Create a Component**:
```rust
// src/components/my_feature.rs
#[derive(Component)]
pub struct MyFeature {
    pub some_data: f32,
}
```

2. **Create a Plugin**:
```rust
// src/plugins/my_feature.rs
pub struct MyFeaturePlugin;

impl Plugin for MyFeaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, my_feature_system);
    }
}
```

3. **Add to GamePlugin**:
```rust
// src/plugins/mod.rs
.add_plugins((
    // ... existing plugins ...
    MyFeaturePlugin,
))
```

### Event-Driven Development

Create events for system communication:

```rust
#[derive(Event)]
pub struct MyEvent {
    pub entity: Entity,
    pub data: MyData,
}

// Send events
event_writer.send(MyEvent { ... });

// Receive events
for event in event_reader.read() {
    // Handle event
}
```

### Debug Tools

The debug system provides comprehensive development tools:

- **Real-time Statistics**: Performance metrics and game stats
- **Visual Debugging**: Collider and physics visualization
- **Entity Inspector**: Runtime entity debugging
- **Console Commands**: F-key shortcuts for quick debugging

## 📊 Performance

- **Plugin Architecture**: Modular systems for better performance
- **Event System**: Efficient decoupled communication
- **State Management**: Proper resource cleanup
- **Debug Controls**: Easy performance monitoring

## 🎯 Future Enhancements

The architecture supports easy addition of:
- **UI System**: Menu and HUD management
- **Audio System**: Sound effects and music
- **Save System**: Game state persistence
- **Networking**: Multiplayer support
- **Asset Management**: Advanced resource loading
- **Scripting**: Lua/WASM integration

## 🤝 Contributing

The modular architecture makes contributing easy:

1. Choose a plugin/system to work on
2. Create components, resources, and events as needed
3. Implement systems with proper state management
4. Add debug tools for development
5. Test with the comprehensive debug system

## 📝 License

This project is open source and available under the MIT License.