use bevy::{prelude::*, time::FixedTimestep};
use bevy_egui::{egui, EguiContext, EguiPlugin};
mod emulator;
use emulator::*;
mod ui;
use ui::*;

fn _ui_example_system(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
    });
}

const BG_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .insert_resource(ClearColor(BG_COLOR))
        .init_resource::<PhysicConst>()
        .add_startup_system(setup)
        .add_startup_system(configure_visuals)
        .add_startup_system(set_windows)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::steps_per_second(60.0))
                .with_system(check_collision)
                .with_system(spring_effect.before(check_collision))
                .with_system(gravity_effect.before(check_collision))
                .with_system(apply_velocity.after(check_collision)),
        )
        .add_system(ui_update)
        .run();
}
