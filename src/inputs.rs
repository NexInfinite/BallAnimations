use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};

pub struct CustomInputPlugin;

impl Plugin for CustomInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Wireframe2dPlugin);
        app.add_systems(Update, (toggle_wireframes, close_handler));
    }
}

fn toggle_wireframes(
    mut wireframe_config: ResMut<Wireframe2dConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}

fn close_handler(mut exit: EventWriter<AppExit>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::KeyW)
        && (keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight))
    {
        exit.send(AppExit::Success);
    } else if keyboard.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}
