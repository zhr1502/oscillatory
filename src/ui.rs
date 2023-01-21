use crate::emulator::physical::*;
use bevy::{prelude::ResMut, prelude::Windows};
use bevy_egui::{egui, EguiContext};
use egui::Vec2;

pub fn configure_visuals(mut egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}

pub fn set_windows(mut windows: ResMut<Windows>){
    let window = windows.get_primary_mut().unwrap();
    window.set_title(String::from("Spring-Gravity emulator"));
}

pub fn ui_update(mut egui_context: ResMut<EguiContext>, mut physic_const: ResMut<PhysicConst>) {

    egui::Window::new("TweakPanel")
        .default_size(Vec2::new(200.0,150.0))
        .show(egui_context.ctx_mut(), |ui| {
            ui.label("Tweak");

            ui.add(
                egui::Slider::new(&mut physic_const.gravitional_acceleration, 0.0..=10.0)
                    .text("Gravitional Acceleration"),
            );

            ui.add(
                egui::Slider::new(&mut physic_const.stiffness, 0.0..=0.3)
                    .text("Stiffness of Sping"),
            );

            ui.add(
                egui::Slider::new(&mut physic_const.collision_coefficient, 0.0..=1.0)
                    .text("Collision Coefficient"),
            );
        });
}
