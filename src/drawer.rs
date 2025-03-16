use bevy::{
    prelude::*, render::view::window, utils::tracing::instrument::WithSubscriber,
    window::WindowResized,
};
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
    last_window_pos: IVec2,
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
            last_window_pos: IVec2::new(0, 0),
        };

        let mut last_window_pos = IVec2::new(0, 0);

        app.add_systems(Startup, (draw_balls, draw_text));
        app.add_systems(
            Update,
            move |move_ball_query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
                  resize_reader: EventReader<WindowResized>,
                  window: Query<&Window>,
                  time: Res<Time>| {
                on_window_resize(resize_reader, &mut config);
                move_balls(move_ball_query, &mut config, window, time);
            },
        );
        app.add_systems(
            Update,
            move |query: Query<&mut Velocity, With<Ball>>,
                  window_move_reader: EventReader<WindowMoved>| {
                on_window_move(window_move_reader, query, &mut last_window_pos);
            },
        );
        app.add_systems(Update, (handle_wall_collision, keyboard_new_ball));
    }
}

fn draw_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let mut balls = Vec::<(Velocity, Transform, BallStyle)>::new();
    for i in 0..5 {
        balls.push(random_ball(i as f32));
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
}

fn draw_text(mut commands: Commands) {
    commands.spawn((
        Text::new("Press space to debug or n for another ball."),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

fn random_ball(z: f32) -> (Velocity, Transform, BallStyle) {
    let mut rng = rand::rng();
    return (
        Velocity {
            x: rng.random_range(-10.0..10.0),
            y: rng.random_range(0.0..15.0),
        },
        Transform::from_xyz(
            rng.random_range(-200.0..200.0),
            rng.random_range(-200.0..200.0),
            z,
        ),
        BallStyle::new_color_only(Color::srgb(
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
        )),
    );
}

fn keyboard_new_ball(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    query: Query<Entity, With<Ball>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if (keyboard.just_pressed(KeyCode::KeyN)) {
        let mut count = 0;
        let _ = query.iter().inspect(|_| count += 1).collect::<Vec<_>>();
        let new_ball = random_ball(count as f32 + 1.0);
        spawn_new_ball(commands, meshes, materials, new_ball);
    }
}

fn spawn_new_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    new_ball: (Velocity, Transform, BallStyle),
) {
    let mesh = meshes.add(Circle::new(new_ball.2.size));
    let material = materials.add(new_ball.2.color);

    commands.spawn((
        Ball,
        Mesh2d(mesh),
        MeshMaterial2d(material),
        new_ball.1,
        new_ball.0,
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

fn on_window_move(
    mut move_reader: EventReader<WindowMoved>,
    mut query: Query<&mut Velocity, With<Ball>>,
    last_window_pos: &mut IVec2,
) {
    if move_reader.read().len() == 0 {
        return;
    }

    // Handle first time move
    let window_event = move_reader.read().last().unwrap();
    if last_window_pos.x == 0 && last_window_pos.y == 0 {
        last_window_pos.x = window_event.position.x;
        last_window_pos.y = window_event.position.y;
    }

    let impulse = 0.05;
    let diff_x = window_event.position.x - last_window_pos.x;
    let diff_y = window_event.position.y - last_window_pos.y;

    if diff_x != 0 || diff_y != 0 {
        for mut velocity in &mut query {
            velocity.x += diff_x as f32 * impulse;
            velocity.y -= diff_y as f32 * impulse;
        }
    }

    last_window_pos.x = window_event.position.x;
    last_window_pos.y = window_event.position.y;
}
