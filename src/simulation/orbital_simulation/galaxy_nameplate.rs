use egui::{Context as EguiContext, Align2, Color32, FontId, Pos2, RichText};
use crate::ui::{manager::Widget};

#[derive(Clone)]
pub struct GalaxyNameplate {
    pub galaxy_name: String,
    pub visible: bool,
    pub position: Pos2,
    pub font_size: f32,
    pub color: Color32,
}

impl GalaxyNameplate {
    pub fn new(galaxy_name: String) -> Self {
        Self {
            galaxy_name,
            visible: true,
            position: Pos2::new(20.0, 20.0),
            font_size: 28.0,
            color: Color32::WHITE,
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position = Pos2::new(x, y);
    }

    pub fn set_font_size(&mut self, size: f32) {
        self.font_size = size;
    }

    pub fn set_color(&mut self, color: Color32) {
        self.color = color;
    }
}

impl Widget for GalaxyNameplate {
    fn show_widget_in_ui(&self, ctx: &EguiContext, _ui: &mut egui::Ui) {
        if !self.visible {
            return;
        }

        // Show as overlay on the main viewport
        egui::Area::new("galaxy_nameplate".into())
            .fixed_pos(self.position)
            .show(ctx, |ui| {
                ui.label(
                    RichText::new(&self.galaxy_name)
                        .font(FontId::proportional(self.font_size))
                        .color(self.color)
                );
            });
    }
}