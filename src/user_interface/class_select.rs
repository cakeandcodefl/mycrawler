use crate::classes::class::*;
use bevy::prelude::*;
use std::str::FromStr;

//TODO: look how we handle player character
#[derive(Component)]
struct PlayerCharacter;

#[derive(Component)]
pub struct ClassSelectButton;

pub static mut CLASS_SELECTED: bool = false;

pub fn handle_character_selection(
    mut interaction_query: Query<
        (&Interaction, &Children, &Parent),
        (Changed<Interaction>, With<ClassSelectButton>),
    >,
    mut text_query: Query<&Text>,
    mut query_visibility: Query<&mut Visibility>,
) {
    unsafe {
        if CLASS_SELECTED {
            return;
        }
    }

    for (interaction, child, parent) in &mut interaction_query {
        let text = text_query.get_mut(child[0]).unwrap();
        let mut class_selection_visibility = query_visibility.get_mut(parent.get()).unwrap();
        match *interaction {
            Interaction::Pressed => {
                match Character::from_str(&text.sections[0].value) {
                    Ok(_created_character) => {
                        println!("you selected {}", text.sections[0].value);
                        //TODO: do sth with the character
                        //selected_character = created_character;
                        //TODO: change the unsafe and remove static to know if a class was selected
                        unsafe {
                            CLASS_SELECTED = true;
                        }
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

//TODO: initialize after class select
pub fn initialize_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PlayerCharacter)
        .insert(PbrBundle {
            mesh: meshes.add(Cuboid::new(2f32, 2f32, 2f32)),
            material: materials.add(Color::rgb(1f32, 1f32, 5f32)),
            transform: Transform::from_xyz(0f32, 1f32, 5f32),
            ..default()
        })
        .insert(Character::new(CharacterType::Warrior));
}
