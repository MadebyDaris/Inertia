use egui::{Context as EguiContext, RichText};
use crate::{
    ui::{control_requests::ObjectCreationRequest, manager::{ControlWidget, Widget, WidgetResponse}},
    utils::vector::Vector,
};

#[derive(Clone)]
pub struct ObjectCreatorWidget {
    pub visible: bool,
    pub name: String,
    pub mass: f32,
    pub radius: f32,
    pub position: [f32; 3],
    pub velocity: [f32; 3],
    pub texture_selection: usize,
    pub available_textures: Vec<String>,
}

impl ObjectCreatorWidget {
    pub fn new() -> Self {
        Self {
            visible: false,
            name: "New Object".to_string(),
            mass: 1.0,
            radius: 2.0,
            position: [0.0, 0.0, 0.0],
            velocity: [0.0, 0.0, 0.0],
            texture_selection: 0,
            available_textures: vec![
                "./data/tex/mercury.jpg".to_string(),
                "./data/tex/mars.jpg".to_string(),
                "./data/tex/earth.jpg".to_string(),
                "./data/tex/jupiter.jpg".to_string(),
            ],
        }
    }
}

impl Widget for ObjectCreatorWidget {
    fn show_widget_in_ui(&self, _ctx: &EguiContext, _ui: &mut egui::Ui) {}
}

impl ControlWidget for ObjectCreatorWidget {
    fn show_control_widget(&mut self, ctx: &EguiContext) -> WidgetResponse {
        if !self.visible {
            return WidgetResponse::None;
        }

        let mut open = self.visible;
        let mut close_now = false;
        let mut response = WidgetResponse::None;
        
        egui::Window::new("🪐 Object Creator")
            .open(&mut open)
            .resizable(true)
            .show(ctx, |ui| {
                ui.label(RichText::new("Create a new celestial body").color(egui::Color32::GRAY).size(11.0));
                ui.add_space(8.0);
                
                ui.horizontal(|ui| {
                    ui.label("Name:");
                    ui.text_edit_singleline(&mut self.name)
                        .on_hover_text("Give your celestial body a unique name");
                });
                
                ui.horizontal(|ui| {
                    ui.label("Mass (kg):");
                    ui.add(egui::DragValue::new(&mut self.mass).speed(1.0).range(0.1..=10000.0))
                        .on_hover_text("Mass affects gravitational pull");
                });
                
                ui.horizontal(|ui| {
                    ui.label("Radius (m):");
                    ui.add(egui::DragValue::new(&mut self.radius).speed(0.1).range(0.1..=20.0))
                        .on_hover_text("Visual and collision radius");
                });
                
                ui.separator();
                ui.label(RichText::new("Initial Position").color(egui::Color32::LIGHT_BLUE));
                
                ui.horizontal(|ui| {
                    ui.label("X:");
                    ui.add(egui::DragValue::new(&mut self.position[0]).speed(1.0));
                    ui.label("Y:");
                    ui.add(egui::DragValue::new(&mut self.position[1]).speed(1.0));
                    ui.label("Z:");
                    ui.add(egui::DragValue::new(&mut self.position[2]).speed(1.0));
                });
                
                ui.separator();
                ui.label(RichText::new("Initial Velocity").color(egui::Color32::LIGHT_GREEN));
                
                ui.horizontal(|ui| {
                    ui.label("Vx:");
                    ui.add(egui::DragValue::new(&mut self.velocity[0]).speed(0.1));
                    ui.label("Vy:");
                    ui.add(egui::DragValue::new(&mut self.velocity[1]).speed(0.1));
                    ui.label("Vz:");
                    ui.add(egui::DragValue::new(&mut self.velocity[2]).speed(0.1));
                });
                
                ui.add_space(4.0);
                ui.label(RichText::new("Tip: For circular orbit, use v = √(G*M/r)").color(egui::Color32::GRAY).size(10.0));
                
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Texture:");
                    egui::ComboBox::from_label("")
                        .selected_text(format!("Texture {}", self.texture_selection + 1))
                        .show_ui(ui, |ui| {
                            for (i, texture) in self.available_textures.iter().enumerate() {
                                let name = texture.split('/').last().unwrap_or("Unknown");
                                ui.selectable_value(&mut self.texture_selection, i, 
                                    format!("{} - {}", i + 1, name.replace(".jpg", "")));
                            }
                        });
                });
                
                ui.add_space(8.0);
                ui.separator();
                
                ui.horizontal(|ui| {
                    if ui.button("Create Object").clicked() {
                        response = WidgetResponse::CreateObject(ObjectCreationRequest {
                            name: self.name.clone(),
                            mass: self.mass,
                            radius: self.radius,
                            position: Vector(self.position[0], self.position[1], self.position[2]),
                            velocity: Vector(self.velocity[0], self.velocity[1], self.velocity[2]),
                            texture_path: self.available_textures[self.texture_selection].clone(),
                        });
                        close_now = true;
                    }
                    
                    if ui.button("Cancel").clicked() {
                        close_now = true;
                    }
                });
            });
        
        if close_now {
            open = false;
        }
        self.visible = open;
        response
    }
}
