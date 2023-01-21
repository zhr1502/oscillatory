use bevy::prelude::Resource;
#[derive(Resource)]
pub struct PhysicConst {
    pub stiffness: f32,
    pub gravitional_acceleration: f32,
    pub collision_coefficient: f32,
}

impl Default for PhysicConst {
    fn default() -> Self {
        return PhysicConst {
            stiffness: 0.05,
            gravitional_acceleration: 3.0,
            collision_coefficient: 0.95,
        };
    }
}
