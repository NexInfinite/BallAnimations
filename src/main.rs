use bevy::prelude::*;
mod draw_balls;
mod inputs;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, inputs::CustomInputPlugin));
    app.add_systems(Startup, draw_balls::draw_balls);
    app.run();
}
