use bevy::{prelude::*, window::WindowResized};
use rand::Rng;

#[derive(Component, Debug)]
#[require(Transform)]
struct Ball;

#[derive(Component, Debug, Clone, Copy)]
pub struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Default)]
struct GlobalBallsConfig {
    balls_move: bool,
    pixel_scaling: f32,
}

#[derive(Component)]
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
        let mut config = GlobalBallsConfig {
            balls_move: true,
            pixel_scaling: 100.0,
        };

        app.add_systems(Startup, (draw_balls, draw_text));
        app.add_systems(
            Update,
            move |query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
                  resize_reader: EventReader<WindowResized>,
                  window: Query<&Window>,
                  time: Res<Time>| {
                on_window_resize(resize_reader, &mut config);
                move_balls(query, &mut config, window, time);
            },
        );
        app.add_systems(Update, handle_wall_collision);
    }
}

fn draw_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let mut rng = rand::rng();
    let mut balls = Vec::<(Velocity, Transform, BallStyle)>::new();

    for i in 0..10 {
        balls.push((
            Velocity {
                x: rng.random_range(-10.0..10.0),
                y: rng.random_range(0.0..15.0),
            },
            Transform::from_xyz(
                rng.random_range(-200.0..200.0),
                rng.random_range(-200.0..200.0),
                i as f32,
            ),
            BallStyle::new_color_only(Color::srgb(
                rng.random_range(0.0..1.0),
                rng.random_range(0.0..1.0),
                rng.random_range(0.0..1.0),
            )),
        ));
    }

    for ball in balls {
        let mesh = meshes.add(Circle::new(ball.2.size));
        let material = materials.add(ball.2.color);

        commands.spawn((Ball, Mesh2d(mesh), MeshMaterial2d(material), ball.1, ball.0));
    }
}

fn move_balls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    config: &mut GlobalBallsConfig,
    window: Query<&Window>,
    time: Res<Time>,
) {
    let half_height = window.single().resolution.height() / 2.0;
    let half_width = window.single().resolution.width() / 2.0;

    for (mut transform, mut velocity) in &mut query {
        let mut translation = transform.translation;

        if config.balls_move == false {
            translation =
                keep_balls_bound(&mut translation, half_height, half_width, velocity.clone());
        } else {
            if transform.translation.y > -half_height + 15.0 {
                let (distance, final_velocity) =
                    calc_displacement_and_vel(velocity.clone(), time.delta_secs(), -9.8);

                velocity.y = final_velocity;
                translation.y += distance * config.pixel_scaling;
            }

            translation.x += velocity.x;
        }

        transform.translation = translation;
    }
}

fn keep_balls_bound(
    translation: &mut Vec3,
    half_height: f32,
    half_width: f32,
    velocity: Velocity,
) -> Vec3 {
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

    return *translation;
}

fn calc_displacement_and_vel(velocity: Velocity, delta: f32, acceleration: f32) -> (f32, f32) {
    let distance = velocity.y * delta + 0.5 * acceleration * delta * delta;
    let final_velocity = velocity.y + acceleration * delta;

    return (distance, final_velocity);
}

fn handle_wall_collision(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Velocity), With<Ball>>,
    window: Query<&Window>,
    time: Res<Time>,
) {
    let half_height = window.single().resolution.height() / 2.0;
    let half_width = window.single().resolution.width() / 2.0;
    let dampening = 0.8;

    for (entity, mut transform, mut velocity) in &mut query {
        if transform.translation.y < -half_height + 15.0 && velocity.y.abs() > 0.5 {
            let new_velocity = Velocity {
                x: velocity.x,
                y: -velocity.y * dampening,
            };
            let (distance, final_velocity) =
                calc_displacement_and_vel(new_velocity, time.delta_secs(), -9.8);
            velocity.y = final_velocity;
            transform.translation.y += distance * 200.0;
        } else if transform.translation.y <= -half_height + 15.0 {
            velocity.y = 0.0;
            transform.translation.y = -half_height + 15.0;

            // Handle friction for ball rubbing on floor
            velocity.x *= 0.995;
            if velocity.x.abs() < 0.005 {
                velocity.x = 0.0;

                commands.entity(entity).despawn();
                println!("Ball despawned!")
            }
        }

        if transform.translation.x < -half_width + 15.0
            || transform.translation.x > half_width - 15.0
        {
            velocity.x = -(velocity.x * dampening);
            transform.translation.x += velocity.x;
        }
    }

    // for (mut transform, mut velocity) in &mut query {
    //     if transform.translation.y >= half_height {
    //         velocity.y = -velocity.y;
    //     } else if transform.translation.y <= -half_height {
    //         velocity.y = -velocity.y - 2.0;
    //     } else if transform.translation.x <= -half_width || transform.translation.x >= half_width {
    //         velocity.x = -(velocity.x / dampening);
    //     }
    // }
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
