use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::{
        component::Component,
        query::With,
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res, ResMut, Resource},
    },
    time::{Time, Timer},
    DefaultPlugins,
};

fn main() {
    App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Custard_Beard".to_owned())));
    commands.spawn((Person, Name("Stacking".to_owned())));
    commands.spawn((Person, Name("epicblaargh".to_owned())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for Name(name) in &query {
            println!("hello {name}!");
        }
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0.contains('_') {
            name.0 = name.0.replace('_', " _ ");
            break;
        }
    }
}

struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(
            2.0,
            bevy::time::TimerMode::Repeating,
        )))
        .add_systems(Startup, add_people)
        .add_systems(Update, (update_people, greet_people).chain());
    }
}

#[derive(Resource)]
struct GreetTimer(Timer);
