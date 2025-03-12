use bevy::prelude::*;

pub fn draw_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let color = Color::srgb(0.1, 0.1, 0.1);
    let ball_size = 15.0;
    let shapes = [(
        meshes.add(Circle::new(ball_size)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    )];

    for (_, shape) in shapes.into_iter().enumerate() {
        commands.spawn((
            Mesh2d(shape.0),
            MeshMaterial2d(materials.add(color)),
            shape.1,
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
