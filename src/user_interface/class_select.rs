use crate::classes::class::*;
use bevy::prelude::*;
use std::str::FromStr;

//TODO: look how we handle player character
#[derive(Component)]
pub struct PlayerCharacter;

#[derive(Component)]
pub struct ClassSelectButton;

#[derive(Component)]
pub struct AbilityButtons;

#[derive(Component)]
pub struct AbilityButtonList;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CharacterState {
    #[default]
    NotSelected,
    Selected,
    SelectedAndSpawned,
}

pub static mut CLASS_SELECTED: bool = false;

pub fn handle_character_selection(
    mut interaction_query: Query<
        (&Interaction, &Children, &Parent),
        (Changed<Interaction>, With<ClassSelectButton>),
    >,
    mut text_query: Query<&Text>,
    mut query_visibility: Query<&mut Visibility>,
    mut commands: Commands, //TODO: commands meshes materials give it to initialize player in another way
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut state: ResMut<NextState<CharacterState>>,
) {
    for (interaction, child, parent) in &mut interaction_query {
        let text = text_query.get_mut(child[0]).unwrap();
        let mut class_selection_visibility = query_visibility.get_mut(parent.get()).unwrap();
        match *interaction {
            Interaction::Pressed => {
                match Character::from_str(&text.sections[0].value) {
                    Ok(created_character) => {
                        println!("you selected {}", text.sections[0].value);
                        commands
                            .spawn(PlayerCharacter)
                            .insert(PbrBundle {
                                mesh: meshes.add(Cuboid::new(2f32, 2f32, 2f32)),
                                material: materials.add(Color::rgb(1f32, 1f32, 5f32)),
                                transform: Transform::from_xyz(0f32, 1f32, 5f32),
                                ..default()
                            })
                            .insert(created_character);
                        state.set(CharacterState::Selected);
                        *class_selection_visibility = Visibility::Hidden;
                    }
                    Err(_e) => println!("no character selected, Should not happen"), //TODO: Proper error handling
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

//TODO: Make generic buttons for character selection
pub fn initialize_class_select_buttons(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Choose your character",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .with_children(|parent| {
            parent
                .spawn(ClassSelectButton)
                .insert(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Mage",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn(ClassSelectButton)
                .insert(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Hunter",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn(ClassSelectButton)
                .insert(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Priest",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn(ClassSelectButton)
                .insert(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Rogue",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn(ClassSelectButton)
                .insert(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Warrior",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}
