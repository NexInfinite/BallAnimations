use bevy::prelude::*;
mod drawer;
mod inputs;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, inputs::CustomInputPlugin, drawer::DrawBalls));
    app.run();
}
