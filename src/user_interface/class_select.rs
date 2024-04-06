use crate::classes::class::*;
use bevy::prelude::*;
use std::str::FromStr;

//TODO: look how we handle player character
#[derive(Component)]
pub struct PlayerCharacter; //TODO: rename to chosen character or selected

#[derive(Component)]
pub struct ClassSelectButton;

#[derive(Component)]
pub struct AbilityButton;

#[derive(Component)]
pub struct AbilityButtonList;

#[derive(Event)]
pub struct ClassSelectedEvent {
    pub selected_character: Character,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClassState {
    #[default]
    NotSelected,
    Selected,
    Spawned,
}

pub fn handle_and_fire_class_selection(
    mut evw_class_selected: EventWriter<ClassSelectedEvent>,
    mut interaction_query: Query<
        (&Interaction, &Children, &Parent),
        (Changed<Interaction>, With<ClassSelectButton>),
    >,
    mut text_query: Query<&Text>,
    mut query_visibility: Query<&mut Visibility>,
    mut state: ResMut<NextState<ClassState>>,
) {
    for (interaction, child, parent) in &mut interaction_query {
        let text = text_query.get_mut(child[0]).unwrap();
        let mut class_selection_visibility = query_visibility.get_mut(parent.get()).unwrap();
        match *interaction {
            Interaction::Pressed => {
                match Character::from_str(&text.sections[0].value) {
                    Ok(created_character) => {
                        println!("you selected {}", text.sections[0].value);
                        *class_selection_visibility = Visibility::Hidden;
                        evw_class_selected.send(ClassSelectedEvent {
                            //TODO: make a constructor for ClassSelectedEvent
                            selected_character: created_character,
                        });
                        state.set(ClassState::Selected);
                    }
                    Err(_e) => println!("no character selected, Should not happen"), //TODO: Proper error handling
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn on_character_selected(
    mut evr_character_selected: EventReader<ClassSelectedEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    for ev in evr_character_selected.read() {
        commands
            .spawn(PlayerCharacter)
            .insert(PbrBundle {
                mesh: meshes.add(Cuboid::new(2f32, 2f32, 2f32)),
                material: materials.add(Color::rgb(1f32, 1f32, 5f32)),
                transform: Transform::from_xyz(0f32, 1f32, 5f32),
                ..default()
            })
            .insert(ev.selected_character.clone()); //TODO: make this a custom spawn command for the character remove the insert dont clone this

        //TODO: Make this a custom spawn command |maybe make a root with 2 node bundles which each have 2 abilities 
        commands
        .spawn(AbilityButtonList)
        .insert(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_self: AlignSelf::Center,
                    align_items: AlignItems::End,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Row,
                    position_type: PositionType::Absolute,
                    row_gap: Val::Px(10.0),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn(AbilityButton)
                    .insert(ButtonBundle {
                        style: Style {
                            width: Val::Px(100.0),
                            height: Val::Px(100.0),
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
                            "1",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 20.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ));
                    });
            })
            .with_children(|parent| {
                parent
                    .spawn(AbilityButton)
                    .insert(ButtonBundle {
                        style: Style {
                            width: Val::Px(100.0),
                            height: Val::Px(100.0),
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
                            "2",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 20.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ));
                    });
            })
            .with_children(|parent| {
                parent
                    .spawn(AbilityButton)
                    .insert(ButtonBundle {
                        style: Style {
                            width: Val::Px(100.0),
                            height: Val::Px(100.0),
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
                            "3",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 20.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ));
                    });
            })
            .with_children(|parent| {
                parent
                    .spawn(AbilityButton)
                    .insert(ButtonBundle {
                        style: Style {
                            width: Val::Px(100.0),
                            height: Val::Px(100.0),
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
                            "4",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 20.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ));
                    });
            });

        println!("character spawned");
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
                position_type: PositionType::Absolute,
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
