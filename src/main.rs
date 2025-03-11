use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, Wireframe2dPlugin));
    app.add_systems(Startup, setup);
    app.add_systems(Update, (toggle_wireframes, close_handler));
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let shapes = [meshes.add(Circle::new(50.0))];

    for shape in shapes.into_iter().enumerate() {
        let color = Color::srgb(0.1, 0.1, 0.1);

        commands.spawn((
            Mesh2d(shape.1),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));
    }

    commands.spawn((
        Text::new("What da sigma: Press space to debug"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
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
    if keyboard.just_pressed(KeyCode::KeyW) && keyboard.pressed(KeyCode::ControlLeft) {
        exit.send(AppExit::Success);
    } else if keyboard.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}
