pub mod constant;
pub mod physical;
pub use constant::basic_const::*;
pub use physical::*;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct Velocity(Vec2);

#[derive(Component)]
pub struct Mouse(Vec2);
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Spawn a ball whose initial Velocity is 0
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(BALL_COLOR)),
            transform: Transform::from_translation(BALL_STARTING_POSITION).with_scale(BALL_SIZE),
            ..default()
        },
        Ball,
        Collider,
        Velocity(Vec2::new(20.0, 0.0)),
    ));
}

pub fn gravity_effect(
    mut ball_query: Query<&mut Velocity, With<Ball>>,
    physic_const: Res<PhysicConst>,
) {
    let mut ball_velocity = ball_query.single_mut();
    ball_velocity.0.y -= physic_const.gravitional_acceleration;
}

pub fn spring_effect(
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    windows: Res<Windows>,
    physic_const: Res<PhysicConst>,
) {
    let window = windows.get_primary().unwrap();
    if let Some(position) = window.cursor_position() {
        let (mut ball_velocity, ball_transform) = ball_query.single_mut();
        let direction_vector = (position
            - Vec2::new(window.width() / 2.0, window.height() / 2.0)
            - ball_transform.translation.truncate())
            * physic_const.stiffness;
        ball_velocity.0 += direction_vector;
    }
}

pub fn check_collision(
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    physic_const: Res<PhysicConst>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();
    let ball_x = ball_transform.translation.x;
    let ball_y = ball_transform.translation.y;
    let collision_coefficient = physic_const.collision_coefficient;
    if ball_x + BALL_RADIUS > RIGHT_BORDER && ball_velocity.0.x.is_sign_positive() // Collide With
                                                                                   // right/left border
                                                                                   // and x axis
                                                                                   // Velocity is
                                                                                   // positive/negative
        || ball_x - BALL_RADIUS < LEFT_BORDER && ball_velocity.0.x.is_sign_negative()
    {
        ball_velocity.0.x = -ball_velocity.0.x * collision_coefficient;
    }
    if ball_y + BALL_RADIUS > TOP_BORDER && ball_velocity.0.y.is_sign_positive()    //Same for
                                                                                    //the y axis

        || ball_y - BALL_RADIUS < BOTTOM_BORDER && ball_velocity.0.y.is_sign_negative()
    {
        ball_velocity.0.y = -ball_velocity.0.y * collision_coefficient;
    }
}

pub fn apply_velocity(mut ball_query: Query<(&Velocity, &mut Transform), With<Ball>>) {
    let (ball_velocity, mut ball_transform) = ball_query.single_mut();
    ball_transform.translation += Vec3::from((TIME_STEP * ball_velocity.0, 0.0));
}
