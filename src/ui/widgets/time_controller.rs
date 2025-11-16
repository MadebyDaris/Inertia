use egui::{Context as EguiContext, RichText};
use crate::ui::{manager::{ControlWidget, Widget, WidgetResponse}, TimeControlCommand};

#[derive(Clone)]
pub struct TimeControllerWidget {
    pub visible: bool,
    pub is_paused: bool,
    pub speed_multiplier: f32,
    pub time_travel_seconds: f32,
}

impl TimeControllerWidget {
    pub fn new() -> Self {
        Self {
            visible: false,
            is_paused: false,
            speed_multiplier: 1.0,
            time_travel_seconds: 5.0,
        }
    }
}

impl Widget for TimeControllerWidget {
    fn show_widget_in_ui(&self, _ctx: &EguiContext, _ui: &mut egui::Ui) {}
}

impl ControlWidget for TimeControllerWidget {
    fn show_control_widget(&mut self, ctx: &EguiContext) -> WidgetResponse {
        if !self.visible {
            return WidgetResponse::None;
        }

        let mut response = WidgetResponse::None;
        
        egui::Window::new("⏱ Time Control")
            .open(&mut self.visible)
            .resizable(true)
            .show(ctx, |ui| {
                ui.label(RichText::new("Control simulation time flow").color(egui::Color32::GRAY).size(11.0));
                ui.add_space(8.0);
                
                ui.horizontal(|ui| {
                    if ui.button(if self.is_paused { "▶ Play" } else { "⏸ Pause" })
                        .on_hover_text("Toggle simulation pause")
                        .clicked() {
                        self.is_paused = !self.is_paused;
                        response = if self.is_paused { 
                            WidgetResponse::TimeControl(TimeControlCommand::Pause)
                        } else { 
                            WidgetResponse::TimeControl(TimeControlCommand::Play)
                        };
                    }
                    
                    if ui.button("⟲ Reset")
                        .on_hover_text("Reset simulation to initial state")
                        .clicked() {
                        response = WidgetResponse::TimeControl(TimeControlCommand::Reset);
                    }
                });
                
                ui.separator();
                ui.label(RichText::new("Time Speed").strong());
                
                ui.horizontal(|ui| {
                    if ui.button("0.25x").clicked() {
                        self.speed_multiplier = 0.25;
                        response = WidgetResponse::TimeControl(TimeControlCommand::SetSpeed(0.25));
                    }
                    if ui.button("0.5x").clicked() {
                        self.speed_multiplier = 0.5;
                        response = WidgetResponse::TimeControl(TimeControlCommand::SetSpeed(0.5));
                    }
                    if ui.button("1x").clicked() {
                        self.speed_multiplier = 1.0;
                        response = WidgetResponse::TimeControl(TimeControlCommand::SetSpeed(1.0));
                    }
                });
                
                ui.horizontal(|ui| {
                    if ui.button("2x").clicked() {
                        self.speed_multiplier = 2.0;
                        response = WidgetResponse::TimeControl(TimeControlCommand::SetSpeed(2.0));
                    }
                    if ui.button("5x").clicked() {
                        self.speed_multiplier = 5.0;
                        response = WidgetResponse::TimeControl(TimeControlCommand::SetSpeed(5.0));
                    }
                    if ui.button("10x").clicked() {
                        self.speed_multiplier = 10.0;
                        response = WidgetResponse::TimeControl(TimeControlCommand::SetSpeed(10.0));
                    }
                });
                
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label("Current Speed:");
                    ui.label(RichText::new(format!("{:.2}x", self.speed_multiplier)).color(egui::Color32::LIGHT_GREEN));
                });
                
                ui.separator();
                ui.label(RichText::new("⚠ Experimental Features").color(egui::Color32::YELLOW));
                
                ui.horizontal(|ui| {
                    ui.label("Seconds to rewind:");
                    ui.add(egui::DragValue::new(&mut self.time_travel_seconds).speed(0.1).range(0.1..=60.0));
                });
                
                ui.horizontal(|ui| {
                    if ui.button("⏪ Rewind")
                        .on_hover_text("WARNING: Experimental time travel feature")
                        .clicked() {
                        response = WidgetResponse::TimeControl(TimeControlCommand::GoBackward(self.time_travel_seconds));
                    }
                });
                
                ui.add_space(4.0);
                ui.label(RichText::new("💡 Keyboard shortcuts:").color(egui::Color32::GRAY).size(10.0));
                ui.label(RichText::new("  SPACE - Pause/Play").color(egui::Color32::GRAY).size(10.0));
                ui.label(RichText::new("  ↑/↓ - Speed up/down").color(egui::Color32::GRAY).size(10.0));
                ui.label(RichText::new("  R - Reverse time").color(egui::Color32::GRAY).size(10.0));
            });
        
        response
    }
}
