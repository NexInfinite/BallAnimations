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
struct GlobalBallsConfig {
    balls_move: bool,
}

#[derive(Default)]
struct BallStyle {
    size: f32,
    color: Color,
}

#[allow(dead_code)]
impl BallStyle {
    const DEFAULT_SIZE: f32 = 15.0;
    const DEFAULT_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);

    fn new(size: f32, color: Color) -> BallStyle {
        BallStyle {
            size: size,
            color: color,
        }
    }

    fn new_size_only(size: f32) -> BallStyle {
        BallStyle {
            size: size,
            color: BallStyle::DEFAULT_COLOR,
        }
    }

    fn new_color_only(color: Color) -> BallStyle {
        BallStyle {
            size: BallStyle::DEFAULT_SIZE,
            color: color,
        }
    }

    fn default() -> BallStyle {
        BallStyle {
            size: BallStyle::DEFAULT_SIZE,
            color: BallStyle::DEFAULT_COLOR,
        }
    }
}

pub struct DrawBalls;
impl Plugin for DrawBalls {
    fn build(&self, app: &mut App) {
        let mut config = GlobalBallsConfig { balls_move: true };

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

    let mut balls = Vec::<(Velocity, Transform, BallStyle)>::new();
    balls.push((
        Velocity { x: 10.0, y: -10.0 },
        Transform::from_xyz(-500.0, 0.0, 0.0),
        BallStyle::default(),
    ));
    balls.push((
        Velocity { x: 5.0, y: -10.0 },
        Transform::from_xyz(0.0, 0.0, 0.0),
        BallStyle::new_color_only(Color::srgb(0.4, 0.1, 0.1)),
    ));

    for ball in balls {
        let mesh = meshes.add(Circle::new(ball.2.size));
        let material = materials.add(ball.2.color);

        commands.spawn((Ball, Mesh2d(mesh), MeshMaterial2d(material), ball.1, ball.0));
    }
}

fn move_balls(
    mut query: Query<(&mut Transform, &Velocity), With<Ball>>,
    config: &mut GlobalBallsConfig,
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
        Text::new("Press space to debug"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

fn on_window_resize(mut resize_reader: EventReader<WindowResized>, config: &mut GlobalBallsConfig) {
    if resize_reader.read().len() == 0 {
        config.balls_move = true;
    }

    for _ in resize_reader.read() {
        config.balls_move = false;
    }
}
