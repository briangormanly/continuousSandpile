use bevy::{prelude::*, transform};
use bevy::window::PrimaryWindow;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, print_names)
        .run();
}


pub fn spawn_grain(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>) {
    let window: &Window = window_query.get_single().unwrap();
    
    commands.spawn(bundle: (transform))
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Person {
        name: "Brian".to_string(),
    });
}

pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name: {}", person.name);
    }
}
#[derive(Component)]
pub struct Person {
    pub name: String,
}