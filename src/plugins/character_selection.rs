use bevy::prelude::*;
use crate::{
    resources::{SelectedCharacter, CharacterType},
    states::*,
};

pub struct CharacterSelectionPlugin;

impl Plugin for CharacterSelectionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::CharacterSelection), setup_character_selection_ui)
            .add_systems(OnExit(GameState::CharacterSelection), cleanup_character_selection_ui)
            .add_systems(Update, (
                handle_character_selection_input,
                update_button_interactions,
            ).run_if(in_state(GameState::CharacterSelection)));
    }
}

// Component to mark the character selection UI root
#[derive(Component)]
struct CharacterSelectionUI;

// Component to mark character selection buttons
#[derive(Component)]
struct CharacterButton {
    character_type: CharacterType,
}

// Component to mark the character preview
#[derive(Component)]
struct CharacterPreview;

fn setup_character_selection_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    selected_character: Res<SelectedCharacter>,
) {
    // Create the main UI container
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            background_color: Color::rgba(0.1, 0.1, 0.1, 0.9).into(),
            ..default()
        },
        CharacterSelectionUI,
        Name::new("CharacterSelectionUI"),
    )).with_children(|parent| {
        // Title
        parent.spawn((
            TextBundle::from_section(
                "Choose Your Character",
                TextStyle {
                    font_size: 48.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            Name::new("Title"),
        ));

        // Character options container
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(80.0),
                    height: Val::Px(400.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceAround,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(Val::Px(40.0)),
                    ..default()
                },
                ..default()
            },
            Name::new("CharacterOptions"),
        )).with_children(|parent| {
            // Boss3 Character Option
            create_character_option(
                parent,
                CharacterType::Boss3,
                selected_character.character_type == CharacterType::Boss3,
                &asset_server,
            );

            // SwordHero Character Option
            create_character_option(
                parent,
                CharacterType::SwordHero,
                selected_character.character_type == CharacterType::SwordHero,
                &asset_server,
            );
        });

        // Instructions
        parent.spawn((
            TextBundle::from_section(
                "Use A/D to navigate, SPACE to select",
                TextStyle {
                    font_size: 24.0,
                    color: Color::rgba(0.8, 0.8, 0.8, 1.0),
                    ..default()
                },
            ),
            Name::new("Instructions"),
        ));
    });

    info!("Character selection UI setup complete");
}

fn create_character_option(
    parent: &mut ChildBuilder,
    character_type: CharacterType,
    is_selected: bool,
    _asset_server: &Res<AssetServer>,
) {
    let border_color = if is_selected {
        Color::rgba(0.2, 0.8, 0.2, 1.0) // Green for selected
    } else {
        Color::rgba(0.3, 0.3, 0.3, 1.0) // Gray for unselected
    };

    let background_color = if is_selected {
        Color::rgba(0.1, 0.4, 0.1, 0.8) // Dark green background for selected
    } else {
        Color::rgba(0.2, 0.2, 0.2, 0.8) // Dark gray background for unselected
    };

    parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(300.0),
                height: Val::Px(350.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(20.0)),
                border: UiRect::all(Val::Px(4.0)),
                ..default()
            },
            border_color: border_color.into(),
            background_color: background_color.into(),
            ..default()
        },
        CharacterButton { character_type },
        Name::new(format!("{:?}Button", character_type)),
    )).with_children(|parent| {
        // Character name
        parent.spawn((
            TextBundle::from_section(
                character_type.display_name(),
                TextStyle {
                    font_size: 28.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            Name::new("CharacterName"),
        ));

        // Placeholder for character preview (you can add a 3D model here later)
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(200.0),
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                background_color: Color::rgba(0.4, 0.4, 0.4, 0.6).into(),
                ..default()
            },
            CharacterPreview,
            Name::new("CharacterPreview"),
        ));

        // Character description
        let description = match character_type {
            CharacterType::Boss3 => "A powerful warrior with\nstrong combat abilities",
            CharacterType::SwordHero => "A skilled swordsman with\nelegant fighting style",
        };

        parent.spawn((
            TextBundle::from_section(
                description,
                TextStyle {
                    font_size: 16.0,
                    color: Color::rgba(0.9, 0.9, 0.9, 1.0),
                    ..default()
                },
            ),
            Name::new("CharacterDescription"),
        ));
    });
}

fn handle_character_selection_input(
    keyboard: Res<Input<KeyCode>>,
    mut selected_character: ResMut<SelectedCharacter>,
    mut next_state: ResMut<NextState<GameState>>,
    mut button_query: Query<(&CharacterButton, &mut BorderColor, &mut BackgroundColor)>,
) {
    let mut changed = false;

    // Navigate between characters
    if keyboard.just_pressed(KeyCode::A) {
        let old_character = selected_character.character_type;
        selected_character.character_type = match selected_character.character_type {
            CharacterType::Boss3 => CharacterType::SwordHero,
            CharacterType::SwordHero => CharacterType::Boss3,
        };
        changed = true;
        info!("Character selection changed from {:?} to {:?}", old_character, selected_character.character_type);
    }

    if keyboard.just_pressed(KeyCode::D) {
        let old_character = selected_character.character_type;
        selected_character.character_type = match selected_character.character_type {
            CharacterType::Boss3 => CharacterType::SwordHero,
            CharacterType::SwordHero => CharacterType::Boss3,
        };
        changed = true;
        info!("Character selection changed from {:?} to {:?}", old_character, selected_character.character_type);
    }

    // Confirm selection
    if keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::Return) {
        info!("Character selection CONFIRMED: {:?}", selected_character.character_type);
        info!("Transitioning to Playing state...");
        next_state.set(GameState::Playing);
        return;
    }

    // Update button visuals if selection changed
    if changed {
        for (button, mut border_color, mut background_color) in button_query.iter_mut() {
            let is_selected = button.character_type == selected_character.character_type;

            if is_selected {
                border_color.0 = Color::rgba(0.2, 0.8, 0.2, 1.0);
                background_color.0 = Color::rgba(0.1, 0.4, 0.1, 0.8);
            } else {
                border_color.0 = Color::rgba(0.3, 0.3, 0.3, 1.0);
                background_color.0 = Color::rgba(0.2, 0.2, 0.2, 0.8);
            }
        }
    }
}

fn update_button_interactions(
    mut interaction_query: Query<
        (&Interaction, &CharacterButton, &mut BorderColor, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut selected_character: ResMut<SelectedCharacter>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, button, mut border_color, mut background_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // Select this character and proceed to game
                selected_character.character_type = button.character_type;
                info!("Character selected via click: {:?}", selected_character.character_type);
                next_state.set(GameState::Playing);
            }
            Interaction::Hovered => {
                // Highlight on hover
                border_color.0 = Color::rgba(0.8, 0.8, 0.2, 1.0);
                background_color.0 = Color::rgba(0.3, 0.3, 0.1, 0.8);
            }
            Interaction::None => {
                // Reset to normal state
                let is_selected = button.character_type == selected_character.character_type;

                if is_selected {
                    border_color.0 = Color::rgba(0.2, 0.8, 0.2, 1.0);
                    background_color.0 = Color::rgba(0.1, 0.4, 0.1, 0.8);
                } else {
                    border_color.0 = Color::rgba(0.3, 0.3, 0.3, 1.0);
                    background_color.0 = Color::rgba(0.2, 0.2, 0.2, 0.8);
                }
            }
        }
    }
}

fn cleanup_character_selection_ui(
    mut commands: Commands,
    ui_query: Query<Entity, With<CharacterSelectionUI>>,
) {
    for entity in ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    info!("Character selection UI cleaned up");
}