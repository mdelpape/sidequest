use bevy::prelude::*;

// Game Events
#[derive(Event)]
pub struct GameStartEvent;

#[derive(Event)]
pub struct GamePauseEvent;

#[derive(Event)]
pub struct GameResumeEvent;

#[derive(Event)]
pub struct GameOverEvent;

// Player Events
#[derive(Event)]
pub struct PlayerSpawnEvent {
    pub position: Vec3,
}

#[derive(Event)]
pub struct PlayerJumpEvent {
    pub entity: Entity,
}

#[derive(Event)]
pub struct PlayerLandEvent {
    pub entity: Entity,
}

#[derive(Event)]
pub struct PlayerFlipEvent {
    pub entity: Entity,
    pub flip_type: FlipType,
}

#[derive(Event)]
pub struct PlayerMoveEvent {
    pub entity: Entity,
    pub direction: Vec3,
}

#[derive(Event)]
pub struct PlayerFallEvent {
    pub entity: Entity,
    pub position: Vec3,
}

#[derive(Debug, Clone)]
pub enum FlipType {
    Front,
    Dive,
}

// Animation Events
#[derive(Event)]
pub struct AnimationStartEvent {
    pub entity: Entity,
    pub animation: String,
}

#[derive(Event)]
pub struct AnimationEndEvent {
    pub entity: Entity,
    pub animation: String,
}

// System Events
#[derive(Event)]
pub struct SystemErrorEvent {
    pub system: String,
    pub error: String,
}

#[derive(Event)]
pub struct DebugEvent {
    pub message: String,
}

// Coin Events
#[derive(Event)]
pub struct CoinCollectedEvent {
    pub coin_entity: Entity,
    pub player_entity: Entity,
    pub position: Vec3,
}

// Trampoline Events
#[derive(Event)]
pub struct TrampolineBounceEvent {
    pub player_entity: Entity,
    pub bounce_force: f32,
    pub platform_entity: Entity,
}

// Plugin to register all events
pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<GameStartEvent>()
            .add_event::<GamePauseEvent>()
            .add_event::<GameResumeEvent>()
            .add_event::<GameOverEvent>()
            .add_event::<PlayerSpawnEvent>()
            .add_event::<PlayerJumpEvent>()
            .add_event::<PlayerLandEvent>()
            .add_event::<PlayerFlipEvent>()
            .add_event::<PlayerMoveEvent>()
            .add_event::<PlayerFallEvent>()
            .add_event::<AnimationStartEvent>()
            .add_event::<AnimationEndEvent>()
            .add_event::<SystemErrorEvent>()
            .add_event::<DebugEvent>()
            .add_event::<CoinCollectedEvent>()
            .add_event::<TrampolineBounceEvent>();
    }
}