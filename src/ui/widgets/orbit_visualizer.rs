use egui::{Context as EguiContext, RichText};
use crate::ui::{manager::{ControlWidget, Widget, WidgetResponse}, VisualCommand};

#[derive(Clone)]
pub struct OrbitVisualizerWidget {
    pub visible: bool,
    pub trails_enabled: bool,
    pub max_trail_length: usize,
    pub show_velocity_arrows: bool,
    pub show_acceleration_arrows: bool,
}

impl OrbitVisualizerWidget {
    pub fn new() -> Self {
        Self {
            visible: false,
            trails_enabled: true,
            max_trail_length: 1000,
            show_velocity_arrows: true,
            show_acceleration_arrows: false,
        }
    }
}

impl Widget for OrbitVisualizerWidget {
    fn show_widget_in_ui(&self, _ctx: &EguiContext, _ui: &mut egui::Ui) {}
}

impl ControlWidget for OrbitVisualizerWidget {
    fn show_control_widget(&mut self, ctx: &EguiContext) -> WidgetResponse {
        if !self.visible {
            return WidgetResponse::None;
        }

        let mut response = WidgetResponse::None;
        
        egui::Window::new("Orbit Trails & Visualization")
            .open(&mut self.visible)
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Orbital Trails");
                ui.separator();
                
                ui.horizontal(|ui| {
                    ui.label("Enable Trails:");
                    if ui.checkbox(&mut self.trails_enabled, "").changed() {
                        response = WidgetResponse::VisualCommand(VisualCommand::ToggleTrails(self.trails_enabled));
                    }
                });
                
                ui.horizontal(|ui| {
                    ui.label("Max Trail Length:");
                    ui.add(egui::Slider::new(&mut self.max_trail_length, 10..=5000).text("points"));
                });
                
                ui.separator();
                ui.heading("Vector Arrows");
                
                ui.horizontal(|ui| {
                    ui.label("Velocity Arrows:");
                    if ui.checkbox(&mut self.show_velocity_arrows, "").changed() {
                        response = WidgetResponse::VisualCommand(VisualCommand::ToggleVelocityArrows(self.show_velocity_arrows));
                    }
                });
                
                ui.horizontal(|ui| {
                    ui.label("Acceleration Arrows:");
                    if ui.checkbox(&mut self.show_acceleration_arrows, "").changed() {
                        response = WidgetResponse::VisualCommand(VisualCommand::ToggleAccelerationArrows(self.show_acceleration_arrows));
                    }
                });
                
                ui.separator();
                ui.label("ℹ Trail colors are randomized");
                ui.label("with transparency per object");
                
                ui.separator();
                if ui.button("Clear All Trails").clicked() {
                    response = WidgetResponse::VisualCommand(VisualCommand::ClearTrails);
                }
            });
        
        response
    }
}
