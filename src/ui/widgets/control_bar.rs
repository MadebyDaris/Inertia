use egui::{Context as EguiContext, FontId, RichText};
use crate::ui::manager::{ControlWidget, Widget, WidgetResponse};

#[derive(Clone)]
pub struct ControlBarWidget {
    pub show_object_creator: bool,
    pub show_time_controller: bool,
    pub show_force_manager: bool,
    pub show_orbit_visualizer: bool,
}

impl ControlBarWidget {
    pub fn new() -> Self {
        Self {
            show_object_creator: false,
            show_time_controller: false,
            show_force_manager: false,
            show_orbit_visualizer: false,
        }
    }
}

impl Widget for ControlBarWidget {
    fn show_widget_in_ui(&self, _ctx: &EguiContext, _ui: &mut egui::Ui) {}
}

impl ControlWidget for ControlBarWidget {
    fn show_control_widget(&mut self, ctx: &EguiContext) -> WidgetResponse {
        egui::TopBottomPanel::bottom("control_bar").show(ctx, |ui| {
            ui.horizontal(|hor_ui| {
                hor_ui.label(RichText::new("  Inertia  ").font(FontId::proportional(15.0)));
                hor_ui.separator();
                
                if hor_ui.button("Add Object").clicked() {
                    self.show_object_creator = !self.show_object_creator;
                }
                
                if hor_ui.button("Time Control").clicked() {
                    self.show_time_controller = !self.show_time_controller;
                }
                
                if hor_ui.button("Force Manager").clicked() {
                    self.show_force_manager = !self.show_force_manager;
                }
                
                if hor_ui.button("Orbit Trails").clicked() {
                    self.show_orbit_visualizer = !self.show_orbit_visualizer;
                }
                
                hor_ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("Use WASD to move camera, Mouse to look around");
                });
            });
        });
        WidgetResponse::None
    }
}
