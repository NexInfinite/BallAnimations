use bevy::{prelude::*, window::WindowResized};

#[derive(Component, Debug)]
#[require(Transform)]
struct Ball;

#[derive(Component, Debug)]
pub struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Default)]
struct BallsConfig {
    balls_move: bool,
}

pub struct DrawBalls;
impl Plugin for DrawBalls {
    fn build(&self, app: &mut App) {
        let mut config = BallsConfig { balls_move: true };

        app.add_systems(Startup, (draw_balls, draw_text));
        app.add_systems(Update, handle_collision);
        app.add_systems(
            Update,
            move |query: Query<(&mut Transform, &Velocity), With<Ball>>,
                  resize_reader: EventReader<WindowResized>,
                  window: Query<&Window>| {
                on_window_resize(resize_reader, &mut config);
                move_balls(query, &mut config, window);
            },
        );
    }
}

fn draw_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let color = Color::srgb(0.1, 0.1, 0.1);
    let ball_size = 15.0;
    let balls = [(
        meshes.add(Circle::new(ball_size)),
        Transform::from_xyz(-500.0, 0.0, 0.0),
    )];

    for (_, ball) in balls.into_iter().enumerate() {
        commands.spawn((
            Ball,
            Mesh2d(ball.0),
            MeshMaterial2d(materials.add(color)),
            ball.1,
            Velocity { x: 10.0, y: -10.0 },
        ));
    }
}

fn move_balls(
    mut query: Query<(&mut Transform, &Velocity), With<Ball>>,
    config: &mut BallsConfig,
    window: Query<&Window>,
) {
    let half_height = window.single().resolution.height() / 2.0;
    let half_width = window.single().resolution.width() / 2.0;

    for (mut transform, velocity) in &mut query {
        let mut translation = transform.translation;

        if config.balls_move == false {
            // Handling ball being off screen on x
            if translation.x <= -half_width {
                translation.x = -half_width + velocity.x;
            } else if translation.x >= half_width {
                translation.x = half_width - velocity.x;
            }

            // Handling ball being off screen on y
            if translation.y <= -half_height {
                translation.y = -half_height + velocity.y;
            } else if translation.y >= half_height {
                translation.y = half_height - velocity.y
            }
        } else {
            translation.y += velocity.y;
            translation.x += velocity.x;
        }

        transform.translation = translation;
    }
}

fn handle_collision(
    mut query: Query<(&Transform, &mut Velocity), With<Ball>>,
    window: Query<&Window>,
) {
    let half_height = window.single().resolution.height() / 2.0;
    let half_width = window.single().resolution.width() / 2.0;

    for (transform, mut velocity) in &mut query {
        if transform.translation.y <= -half_height || transform.translation.y >= half_height {
            velocity.y = -velocity.y;
        } else if transform.translation.x <= -half_width || transform.translation.x >= half_width {
            velocity.x = -velocity.x;
        }
    }
}

fn draw_text(mut commands: Commands) {
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

fn on_window_resize(mut resize_reader: EventReader<WindowResized>, config: &mut BallsConfig) {
    if resize_reader.read().len() == 0 {
        config.balls_move = true;
    }

    for _ in resize_reader.read() {
        config.balls_move = false;
    }
}
