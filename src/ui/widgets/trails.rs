use egui::Context as EguiContext;
use crate::ui::manager::Widget;

#[derive(Clone)]
pub struct TrailsWidget {
    pub enabled: bool,
    pub max_trail_length: usize,
}

impl TrailsWidget {
    pub fn new() -> Self {
        Self {
            enabled: true,
            max_trail_length: 1000,
        }
    }
}

impl Widget for TrailsWidget {
    fn show_widget_in_ui(&self, _ctx: &EguiContext, ui: &mut egui::Ui) {
        ui.heading("Orbital Trails");
        ui.separator();
        
        ui.label(format!("Trails: {}", if self.enabled { "Enabled" } else { "Disabled" }));
        ui.label(format!("Max Length: {} points", self.max_trail_length));
        
        ui.separator();
        ui.label("Trail colors are randomized");
        ui.label("per object with transparency");
    }
}
