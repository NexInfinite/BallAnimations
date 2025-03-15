use bevy::prelude::*;

#[derive(Component, Debug)]
#[require(Transform)]
struct Ball;

#[derive(Component, Debug)]
pub struct Velocity {
    x: f32,
    y: f32,
}

pub struct DrawBalls;

impl Plugin for DrawBalls {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (draw_balls, draw_text));
        app.add_systems(Update, (move_balls, handle_collision));
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
        Transform::from_xyz(0.0, 0.0, 0.0),
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

fn move_balls(mut query: Query<(&mut Transform, &Velocity), With<Ball>>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.y += velocity.y;
        transform.translation.x += velocity.x;
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
