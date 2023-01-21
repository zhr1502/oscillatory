pub mod basic_const {
    use bevy::prelude::{Color, Vec3};

    pub const BALL_COLOR: Color = Color::TEAL;
    pub const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, 0.0, 1.0);
    pub const BALL_RADIUS: f32 = 25.0;
    pub const BALL_SIZE: Vec3 = Vec3::new(BALL_RADIUS * 2.0, BALL_RADIUS * 2.0, 0.0);

    pub const RIGHT_BORDER: f32 = 450.0;
    pub const LEFT_BORDER: f32 = -450.0;
    pub const TOP_BORDER: f32 = 300.0;
    pub const BOTTOM_BORDER: f32 = -300.0;

    pub const TIME_STEP: f32 = 1.0 / 60.0;
}
