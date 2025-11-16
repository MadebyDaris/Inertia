use egui::{Color32, RichText, Stroke};
use crate::{
    render::CameraMat,
    simulation::orbital_simulation::GalaxySimulation,
    ui::{hover_system::HoverSystem, physics_ui::PhysicsUI, projection::project_to_screen},
    utils::vector::Vector,
};
pub struct NameplateRenderer;

impl NameplateRenderer {
    /// Render nameplates for all bodies in the simulation
    pub fn render(
        ctx: &egui::Context,
        simulation: &GalaxySimulation,
        camera_mat: &CameraMat,
        hover_system: &HoverSystem,
        width: u32,
        height: u32,
    ) {
        for (idx, obj) in simulation.owned_objects.iter().enumerate() {
            let center_pos = obj.position();
            
            // Project the planet CENTER to screen space
            if let Some((screen_x, screen_y)) = project_to_screen(center_pos, camera_mat, width, height) {
                let painter = ctx.debug_painter();
                painter.circle_filled(
                    egui::pos2(screen_x, screen_y),
                    5.0,
                    Color32::from_rgba_premultiplied(255, 0, 0, 150), // Red dot at center
                );
                
                let top_pos = center_pos + Vector(0.0, obj.r + 1.5, 0.0);
                let label_screen_y = if let Some((_, top_y)) = project_to_screen(top_pos, camera_mat, width, height) {
                    top_y - 15.0 
                } else {
                    screen_y - 40.
                };
                
                let is_hovered = Some(idx) == hover_system.hovered_index;
                let color = PhysicsUI::get_body_color(idx);

                egui::Area::new(format!("label_{}", idx).into())
                    .fixed_pos(egui::Pos2::new(screen_x, label_screen_y))
                    .show(ctx, |ui| {
                        egui::Frame::none()
                            .fill(if is_hovered {
                                Color32::from_rgba_premultiplied(255, 220, 100, 220)
                            } else {
                                Color32::from_rgba_premultiplied(0, 0, 0, 180)
                            })
                            .inner_margin(egui::vec2(8.0, 4.0))
                            .rounding(4.0)
                            .stroke(if is_hovered {
                                Stroke::new(1.5, Color32::from_rgb(255, 240, 150))
                            } else {
                                Stroke::NONE
                            })
                            .show(ui, |ui| {
                                ui.label(
                                    RichText::new(&simulation.object_names[idx])
                                        .color(if is_hovered { Color32::BLACK } else { color })
                                        .size(if is_hovered { 14.0 } else { 13.0 })
                                        .strong(),
                                );
                            });
                    });
            }
        }
    }
}