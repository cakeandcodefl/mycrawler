use bevy::prelude::*;
use bevy::window::{Window, WindowResolution, WindowPlugin};
//use std::f32::consts::TAU;
//use std::f32::consts::PI;
mod classes;
mod stats;
use crate::classes::class::*;

#[derive(Component)]
struct PlayerCharacter;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
pub struct Scene {
    title: String,
}

#[derive(Component)]
pub struct MyCamera;

pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn main() {
   App::new()
   .insert_resource(ClearColor(Color::rgb(0.2f32, 0.2f32, 0.2f32)))
   .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
        resolution: WindowResolution::new(WIDTH, HEIGHT),
        title: "fancy_crawler".to_string(),
        resizable:false,
        ..Default::default()
        }),
        ..Default::default()
    }))
   .add_systems(Startup, initialize_scene)
   .add_systems(Startup, initialize_camera)
   .add_systems(Startup, initialize_player)
   .add_systems(Startup, initialize_class_select_buttons)
   //.add_systems(Startup, rotate_camera)
   .run();
}

fn initialize_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PlayerCharacter).insert(PbrBundle {
        mesh: meshes.add(Cuboid::new(2f32, 2f32,2f32)),
        material: materials.add(Color::rgb(1f32, 1f32, 5f32)),
        transform: Transform::from_xyz(0f32, 1f32, 5f32),
        ..default()
    },).insert(Character::new(CharacterType::Warrior));
}

fn handle_choose_character_button_pressed() {
    
}

//TODO: Create multiple buttons for class selections
fn initialize_class_select_buttons(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands
    .spawn(NodeBundle {
        style: Style {
            width: Val::Percent(20.0),
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
            .spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(65.0),
                    border: UiRect::all(Val::Px(5.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
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
            .spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(65.0),
                    border: UiRect::all(Val::Px(5.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
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


fn initialize_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(MyCamera).insert(Camera3dBundle {
        transform: Transform::from_xyz(-2f32, 5f32, 10f32).looking_at(Vec3::ZERO, Vec3::Y),//.with_rotation(Quat::from_rotation_y(-PI / 2f32)),
        ..default()
    });
}


fn initialize_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    commands.spawn(Scene {
        title: "first".to_string(),
    }).insert(PbrBundle {
        mesh: meshes.add( Plane3d::default().mesh().size(12f32, 12f32)),
        material: materials.add(Color::rgb(1f32, 5f32, 1f32)),
        transform: Transform::from_xyz(0., 0.,0.),
        ..default()
    },);

    commands.spawn(Enemy)
    .insert(PbrBundle {
        mesh: meshes.add(Cuboid::new(2f32, 2f32,2f32)),
        material: materials.add(Color::rgb(5f32, 1f32, 1f32)),
        transform: Transform::from_xyz(0f32, 1f32, -5f32),
        ..default()
    },).insert(stats::base_stats::Health {
        hp_value: 100,
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-2f32, 5f32,10f32),
        ..default()
    });

}


/* pub fn rotate_camera(mut query: Query<&mut Transform, With<MyCamera>>) {

    println!("rotating cam");
    for mut transform in &mut query {
        transform.rotate_y(PI / 4.0)
    }
}

pub fn rotate_scene(mut query: Query<&mut Transform, With<Scene>>, timer: Res<Time>) {
    // get all matching entities
    for mut transform in &mut query {
        transform.rotate_y(-(PI / 2.0))
    }

} */