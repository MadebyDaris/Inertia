use egui::Context as EguiContext;
use crate::ui::manager::Widget;

#[derive(Clone)]
pub struct PlanetInfoPanel {
    pub planets: Vec<PlanetInfo>,
}

#[derive(Clone)]
pub struct PlanetInfo {
    pub name: String,
    pub mass: f32,
    pub velocity: f32,
    pub distance_from_sun: f32,
    pub color: egui::Color32,
}

impl PlanetInfoPanel {
    pub fn new(planets: Vec<PlanetInfo>) -> Self {
        Self { planets }
    }
}

impl Widget for PlanetInfoPanel {
    fn show_widget_in_ui(&self, _ctx: &EguiContext, ui: &mut egui::Ui) {
        ui.heading("Celestial Bodies");
        ui.separator();
        
        for planet in &self.planets {
            ui.horizontal(|ui| {
                ui.colored_label(planet.color, "●");
                ui.label(&planet.name);
            });
            ui.indent(planet.name.clone(), |ui| {
                ui.label(format!("Mass: {:.2} kg", planet.mass));
                ui.label(format!("Speed: {:.2} m/s", planet.velocity));
                ui.label(format!("Distance: {:.2} m", planet.distance_from_sun));
            });
            ui.separator();
        }
    }
}
