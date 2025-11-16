use egui::Context as EguiContext;
use crate::{
    simulation::orbital_simulation::astralBody::AstralBody,
    ui::manager::Widget,
};

#[derive(Clone)]
pub struct AstralBodyInfoWidget {
    pub name: String,
    pub mass: f32,
    pub position: String,
    pub velocity: String,
    pub angular_velocity: String,
    pub acc: String,
    pub moment_of_inertia: f32,
    pub euler_angles: String,
    pub radius: f32,
}

impl AstralBodyInfoWidget {
    pub fn new(astral_body: &AstralBody, name: String) -> Self {
        Self {
            name,
            mass: astral_body.mass,
            position: astral_body.position_str(),
            velocity: astral_body.velocity_str(),
            angular_velocity: format!(
                "({}, {}, {}) rad/s",
                astral_body.angular_velocity.0,
                astral_body.angular_velocity.1,
                astral_body.angular_velocity.2
            ),
            acc: format!(
                "({}, {}, {}) m/s²",
                astral_body.acceleration.0,
                astral_body.acceleration.1,
                astral_body.acceleration.2
            ),
            moment_of_inertia: astral_body.moment_of_inertia,
            euler_angles: format!(
                "Pitch: {:.2}°, Yaw: {:.2}°, Roll: {:.2}°",
                astral_body.euler_angles.pitch.to_degrees(),
                astral_body.euler_angles.yaw.to_degrees(),
                astral_body.euler_angles.roll.to_degrees()
            ),
            radius: astral_body.r,
        }
    }
}

impl Widget for AstralBodyInfoWidget {
    fn show_widget_in_ui(&self, _ctx: &EguiContext, ui: &mut egui::Ui) {
        ui.collapsing(&self.name, |ui| {
            ui.label(format!("Name: {}", self.name));
            
            ui.horizontal(|ui| {
                ui.label(format!("Mass: {:.2} kg", self.mass));
                if ui.small_button("ℹ").on_hover_text("Total mass of the celestial body in kilograms").clicked() {}
            });
            
            ui.horizontal(|ui| {
                ui.label(format!("Position: {}", self.position));
                if ui.small_button("ℹ").on_hover_text("Current position in 3D space (x, y, z) meters").clicked() {}
            });
            
            ui.horizontal(|ui| {
                ui.label(format!("Velocity: {}", self.velocity));
                if ui.small_button("ℹ").on_hover_text("Linear velocity vector in meters per second").clicked() {}
            });
            
            ui.horizontal(|ui| {
                ui.label(format!("Angular Velocity: {}", self.angular_velocity));
                if ui.small_button("ℹ").on_hover_text("Rotation velocity in radians per second").clicked() {}
            });
            
            ui.horizontal(|ui| {
                ui.label(format!("Acceleration: {}", self.acc));
                if ui.small_button("ℹ").on_hover_text("Current acceleration vector from all forces").clicked() {}
            });
            
            ui.horizontal(|ui| {
                ui.label(format!("Moment of Inertia: {:.2} kg·m²", self.moment_of_inertia));
                if ui.small_button("ℹ").on_hover_text("Resistance to rotational acceleration").clicked() {}
            });
            
            ui.horizontal(|ui| {
                ui.label(format!("Euler Angles: {}", self.euler_angles));
                if ui.small_button("ℹ").on_hover_text("Rotation angles: Pitch (X), Yaw (Y), Roll (Z) in degrees").clicked() {}
            });
            
            ui.horizontal(|ui| {
                ui.label(format!("Radius: {:.2} m", self.radius));
                if ui.small_button("ℹ").on_hover_text("Physical radius of the sphere").clicked() {}
            });
        });
    }
}
