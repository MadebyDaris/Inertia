use egui::{Context as EguiContext, RichText};
use crate::{
    physics::Force,
    ui::{manager::{ControlWidget, Widget, WidgetResponse}, ForceCommand},
    utils::vector::Vector,
};

#[derive(Clone)]
pub struct ForceManagerWidget {
    pub visible: bool,
    pub selected_object: String,
    pub force_direction: [f32; 3],
    pub force_magnitude: f32,
    pub gravity_constant: f32,
    pub damping_enabled: bool,
    pub available_objects: Vec<String>,
}

impl ForceManagerWidget {
    pub fn new() -> Self {
        Self {
            visible: false,
            selected_object: String::new(),
            force_direction: [1.0, 0.0, 0.0],
            force_magnitude: 1.0,
            gravity_constant: 5.0,
            damping_enabled: true,
            available_objects: Vec::new(),
        }
    }

    pub fn update_available_objects(&mut self, objects: Vec<String>) {
        self.available_objects = objects;
        if self.selected_object.is_empty() && !self.available_objects.is_empty() {
            self.selected_object = self.available_objects[0].clone();
        }
    }
}

impl Widget for ForceManagerWidget {
    fn show_widget_in_ui(&self, _ctx: &EguiContext, _ui: &mut egui::Ui) {}
}

impl ControlWidget for ForceManagerWidget {
    fn show_control_widget(&mut self, ctx: &EguiContext) -> WidgetResponse {
        if !self.visible {
            return WidgetResponse::None;
        }

        let mut response = WidgetResponse::None;
        
        egui::Window::new("⚡ Force Manager")
            .open(&mut self.visible)
            .resizable(true)
            .show(ctx, |ui| {
                ui.label(RichText::new("Apply forces and configure physics").color(egui::Color32::GRAY).size(11.0));
                ui.add_space(8.0);
                
                ui.horizontal(|ui| {
                    ui.label("Target Object:");
                    egui::ComboBox::from_label("")
                        .selected_text(&self.selected_object)
                        .show_ui(ui, |ui| {
                            for object in &self.available_objects {
                                ui.selectable_value(&mut self.selected_object, object.clone(), object);
                            }
                        });
                });
                
                ui.separator();
                ui.label(RichText::new("🌍 Global Physics Settings").strong());
                
                ui.horizontal(|ui| {
                    ui.label("Gravity Constant (G):");
                    if ui.add(egui::DragValue::new(&mut self.gravity_constant).speed(0.1).range(0.1..=20.0))
                        .on_hover_text("Universal gravitational constant. Higher = stronger gravity")
                        .changed() {
                        response = WidgetResponse::ForceCommand(ForceCommand::SetGravity(self.gravity_constant));
                    }
                });
                
                ui.separator();
                ui.label(RichText::new("💨 Apply Custom Force").strong());
                
                ui.horizontal(|ui| {
                    ui.label("Damping:");
                    if ui.checkbox(&mut self.damping_enabled, "Enable atmospheric drag")
                        .on_hover_text("Simulates air resistance")
                        .changed() {
                        response = WidgetResponse::ForceCommand(ForceCommand::ToggleDamping {
                            target_object: self.selected_object.clone(),
                            enabled: self.damping_enabled,
                        });
                    }
                });
                
                ui.add_space(4.0);
                ui.label(RichText::new("Force Direction").color(egui::Color32::LIGHT_BLUE));
                
                ui.horizontal(|ui| {
                    ui.label("X:");
                    ui.add(egui::DragValue::new(&mut self.force_direction[0]).speed(0.1));
                    ui.label("Y:");
                    ui.add(egui::DragValue::new(&mut self.force_direction[1]).speed(0.1));
                    ui.label("Z:");
                    ui.add(egui::DragValue::new(&mut self.force_direction[2]).speed(0.1));
                });
                
                ui.horizontal(|ui| {
                    ui.label("Force Magnitude:");
                    ui.add(egui::DragValue::new(&mut self.force_magnitude).speed(0.1).range(0.0..=100.0));
                });
                
                ui.add_space(4.0);
                if ui.button("⚡ Apply Force").clicked() && !self.selected_object.is_empty() {
                    let force = Force {
                        direction: Vector(self.force_direction[0], self.force_direction[1], self.force_direction[2]).normalized(),
                        magnitude: self.force_magnitude,
                    };
                    response = WidgetResponse::ForceCommand(ForceCommand::AddForce {
                        target_object: self.selected_object.clone(),
                        force,
                    });
                }
                
                ui.add_space(8.0);
                ui.label(RichText::new("💡 Quick Presets:").color(egui::Color32::GRAY).size(10.0));
                
                ui.horizontal(|ui| {
                    if ui.small_button("⬆ Up").clicked() {
                        self.force_direction = [0.0, 1.0, 0.0];
                    }
                    if ui.small_button("⬇ Down").clicked() {
                        self.force_direction = [0.0, -1.0, 0.0];
                    }
                    if ui.small_button("➡ Right").clicked() {
                        self.force_direction = [1.0, 0.0, 0.0];
                    }
                    if ui.small_button("⬅ Left").clicked() {
                        self.force_direction = [-1.0, 0.0, 0.0];
                    }
                });
            });
        
        response
    }
}
