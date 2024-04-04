use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin, WindowResolution};
mod abilities;
mod classes;
mod stats;
mod user_interface;
use crate::classes::class::*;
use crate::user_interface::class_select::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum MyAppState {
    LoadingScreen,
    MainMenu,
    InGame,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum PlayerState {
    #[default]
    NotInGame,
}

#[derive(Component)]
struct Enemy;

#[derive(Component)]
pub struct Scene;

#[derive(Component)]
pub struct MyCamera;

#[derive(Event)]
struct CharacterSelectedEvent;

pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2f32, 0.2f32, 0.2f32)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WIDTH, HEIGHT),
                title: "fancy_crawler".to_string(),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, initialize_scene)
        .add_systems(Startup, initialize_camera)
        .init_state::<PlayerState>()
        .init_state::<CharacterState>()
        .add_systems(Startup, initialize_class_select_buttons) //TODO:swap this to event too
        .add_systems(Update, handle_character_selection)
        .add_event::<CharacterSelectedEvent>()
        .add_systems(
            Update,
            (fire_character_selected, on_character_selected)
                .run_if(in_state(CharacterState::Selected)),
        )
        .run();
}

fn fire_character_selected(
    mut evw_character_selected: EventWriter<CharacterSelectedEvent>,
    query: Query<&Character, With<PlayerCharacter>>,
) {
    for character in query.iter() {
        println!("sending");
        evw_character_selected.send(CharacterSelectedEvent);
    }
}

fn on_character_selected(mut evr_character_selected: EventReader<CharacterSelectedEvent>) {
    for ev in evr_character_selected.read() {
        println!("dude");
    }
}

fn initialize_camera(mut commands: Commands) {
    commands.spawn(MyCamera).insert(Camera3dBundle {
        transform: Transform::from_xyz(-2f32, 5f32, 10f32).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn initialize_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Scene).insert(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(12f32, 12f32)),
        material: materials.add(Color::rgb(1f32, 5f32, 1f32)),
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });

    commands
        .spawn(Enemy)
        .insert(PbrBundle {
            mesh: meshes.add(Cuboid::new(2f32, 2f32, 2f32)),
            material: materials.add(Color::rgb(5f32, 1f32, 1f32)),
            transform: Transform::from_xyz(0f32, 1f32, -5f32),
            ..default()
        })
        .insert(stats::base_stats::Health { hp_value: 100 });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-2f32, 5f32, 10f32),
        ..default()
    });
}
