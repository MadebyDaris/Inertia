use egui::{Context as EguiContext, RichText};
use crate::{
    simulation::{orbital_simulation::astralBody::AstralBody, simulation::Simulation, timeutil::SimulationTime},
    ui::manager::Widget,
    utils::vector::Vector,
};

#[derive(Clone)]
pub struct SimulationInfoWidget {
    pub elapsed_time: f32,
    pub time: SimulationTime,
    pub body_count: usize,
    pub average_velocity: f32,
    pub total_mass: f32,
    pub total_kinetic_energy: f32,
    pub center_of_mass: Vector,
}

impl SimulationInfoWidget {
    pub fn new(simulation: &dyn Simulation<Object = AstralBody>, time_control: SimulationTime) -> Self {
        let objects = simulation.get_objects();
        let count = objects.len();
        
        let mut total_velocity = Vector(0.0, 0.0, 0.0);
        let mut total_mass = 0.0;
        let mut total_ke = 0.0;
        let mut com = Vector(0.0, 0.0, 0.0);
        
        for body in objects {
            total_velocity += body.velocity;
            total_mass += body.mass;
            total_ke += 0.5 * body.mass * body.velocity.magnitude().powi(2);
            com = com + body.position() * body.mass;
        }
        
        if total_mass > 0.0 {
            com = com / total_mass;
        }
        
        let average_velocity = if count > 0 {
            (total_velocity / count as f32).magnitude()
        } else {
            0.0
        };

        Self {
            elapsed_time: 0.0,
            time: time_control,
            body_count: count,
            average_velocity,
            total_mass,
            total_kinetic_energy: total_ke,
            center_of_mass: com,
        }
    }
    
    pub fn update(&mut self, simulation: &dyn Simulation<Object = AstralBody>, time_control: SimulationTime, delta_time: f32) {
        self.elapsed_time += delta_time;
        self.time = time_control;
        
        let objects = simulation.get_objects();
        self.body_count = objects.len();
        
        let mut total_velocity = Vector(0.0, 0.0, 0.0);
        let mut total_mass = 0.0;
        let mut total_ke = 0.0;
        let mut com = Vector(0.0, 0.0, 0.0);
        
        for body in objects {
            total_velocity += body.velocity;
            total_mass += body.mass;
            total_ke += 0.5 * body.mass * body.velocity.magnitude().powi(2);
            com = com + body.position() * body.mass;
        }
        
        if total_mass > 0.0 {
            com = com / total_mass;
        }
        
        self.average_velocity = if self.body_count > 0 {
            (total_velocity / self.body_count as f32).magnitude()
        } else {
            0.0
        };
        
        self.total_mass = total_mass;
        self.total_kinetic_energy = total_ke;
        self.center_of_mass = com;
    }
}

impl Widget for SimulationInfoWidget {
    fn show_widget_in_ui(&self, _ctx: &EguiContext, ui: &mut egui::Ui) {
        ui.heading(RichText::new("Simulation Info").size(16.0));
        ui.separator();
        ui.add_space(4.0);

        ui.label(RichText::new("Status & Time").color(egui::Color32::LIGHT_BLUE).strong());
        ui.add_space(2.0);
        
        let time_status = if self.time.paused {
            RichText::new("⏸ PAUSED").color(egui::Color32::YELLOW)
        } else if self.time.reverse_time {
            RichText::new(format!("REVERSE {:.2}x", self.time.accelerated_time_factor)).color(egui::Color32::from_rgb(255, 150, 150))
        } else {
            RichText::new(format!("RUNNING {:.2}x", self.time.accelerated_time_factor)).color(egui::Color32::from_rgb(0, 255, 100))
        };
        ui.label(time_status);
        ui.label(format!("Elapsed: {:.1} seconds", self.elapsed_time));
        
        ui.add_space(6.0);
        ui.separator();
        ui.add_space(6.0);
        
        ui.label(RichText::new("System Statistics").color(egui::Color32::LIGHT_BLUE).strong());
        ui.add_space(2.0);
        
        ui.horizontal(|ui| {
            ui.label("Bodies:");
            ui.label(RichText::new(format!("{}", self.body_count)).strong());
        }).response.on_hover_text("Total number of celestial bodies in simulation");
        
        ui.horizontal(|ui| {
            ui.label("Total Mass:");
            ui.label(RichText::new(format!("{:.2e} kg", self.total_mass)).monospace());
        }).response.on_hover_text("Sum of all body masses");
        
        ui.horizontal(|ui| {
            ui.label("Avg Velocity:");
            ui.label(RichText::new(format!("{:.2} m/s", self.average_velocity)).monospace());
        }).response.on_hover_text("Average velocity magnitude of all bodies");
        
        ui.horizontal(|ui| {
            ui.label("Total KE:");
            ui.label(RichText::new(format!("{:.2e} J", self.total_kinetic_energy)).monospace());
        }).response.on_hover_text("Total kinetic energy of the system");
        
        ui.horizontal(|ui| {
            ui.label("Center of Mass:");
        }).response.on_hover_text("System center of mass coordinates");
        ui.label(RichText::new(format!("  ({:.1}, {:.1}, {:.1})", 
            self.center_of_mass.0, self.center_of_mass.1, self.center_of_mass.2)).monospace().size(10.0));

        ui.add_space(6.0);
        ui.separator();
        ui.add_space(4.0);
        
        ui.label(RichText::new("⌨ Keyboard Controls").color(egui::Color32::GRAY).size(11.0));
        ui.label(RichText::new("  SPACE - Pause/Resume").color(egui::Color32::GRAY).size(10.0));
        ui.label(RichText::new("  R - Reverse Time").color(egui::Color32::GRAY).size(10.0));
        ui.label(RichText::new("  ↑/↓ - Speed Up/Down").color(egui::Color32::GRAY).size(10.0));
        ui.label(RichText::new("  WASD - Move Camera").color(egui::Color32::GRAY).size(10.0));
        ui.label(RichText::new("  Mouse - Look Around").color(egui::Color32::GRAY).size(10.0));
    }
}
