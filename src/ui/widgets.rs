use egui::{Context as EguiContext, FontId, RichText};
use crate::{
    physics::{physicsobject::AstralBody, Force}, 
    simulation::{simulation::Simulation, timeutil::SimulationTime}, 
    ui::{self, control_requests::ObjectCreationRequest, manager::{ControlWidget, Widget, WidgetResponse}, ForceCommand, TimeControlCommand}, 
    utils::vector::Vector,
};


//
// Some Widget Examples
//


// 
// Astral Body Info Widget
// 
#[derive(Clone)]
pub struct AstralBodyInfoWidget {
    pub name: String,
    pub mass: f32,                    // kg
    pub position: String,             // m
    pub velocity: String,             // m/s
    pub angular_velocity: String,     // rad/s
    pub acc: String,                  // m/s^2
    pub moment_of_inertia: f32,       // kg·m^2
    pub euler_angles: String,         // deg (or rad)
    pub radius: f32,                  // m
}


impl AstralBodyInfoWidget {
    pub fn new(astral_body: &AstralBody, name: String) -> Self {
        Self {
            name,
            mass: astral_body.mass,
            position: astral_body.position_str(),
            velocity: astral_body.velocity_str(),
            angular_velocity: format!(
                "({}, {}, {}) rad/s",
                astral_body.angular_velocity.0,
                astral_body.angular_velocity.1,
                astral_body.angular_velocity.2
            ),
            acc: format!(
                "({}, {}, {}) m/s²",
                astral_body.acceleration.0,
                astral_body.acceleration.1,
                astral_body.acceleration.2
            ),
            moment_of_inertia: astral_body.moment_of_inertia,
            euler_angles: format!(
                "Pitch: {:.2}°, Yaw: {:.2}°, Roll: {:.2}°",
                astral_body.euler_angles.pitch.to_degrees(),
                astral_body.euler_angles.yaw.to_degrees(),
                astral_body.euler_angles.roll.to_degrees()
            ),
            radius: astral_body.r,
        }
    }
}

impl Widget for AstralBodyInfoWidget {
    fn show_widget_in_ui(&self, ctx: &EguiContext, ui: &mut egui::Ui) {
        ui.collapsing(&self.name, |ui| {
            ui.label(format!("Name: {}", self.name));
            ui.label(format!("Mass: {:.2} kg", self.mass));
            ui.label(format!("Position: {}", self.position));
            ui.label(format!("Velocity: {}", self.velocity));
            ui.label(format!("Angular Velocity: {}", self.angular_velocity));
            ui.label(format!("Acceleration: {}", self.acc));
            ui.label(format!("Moment of Inertia: {:.2} kg·m²", self.moment_of_inertia));
            ui.label(format!("Euler Angles: {}", self.euler_angles));
            ui.label(format!("Radius: {:.2} m", self.radius));
        });
    }
}


// 
// Simulation Info Widget
// 
// TODO:
// Number of objects in simulation
// Simulation time acceleration (1x, 10x, etc.)
// Average velocity or kinetic energy of bodies
// Total gravitational potential energy of the system
// Frame time / FPS (helps debugging performance)
// Center of mass coordinates
// Largest body name & mass
// Smallest body name & mass
// Paused/Running status
// Collision count (since start)

#[derive(Clone)]
pub struct SimulationInfoWidget {
    pub elapsed_time: f32,
    pub time: SimulationTime,
    pub body_count: usize,
    pub average_velocity: f32
}
impl SimulationInfoWidget {
    pub fn new(simulation: &Simulation, time_control: SimulationTime) -> Self {
        let count = simulation.owned_objects.len();
        let mut total_velocity = Vector(0.0, 0.0, 0.0);
        for body in &simulation.owned_objects {
            total_velocity += body.velocity;
        }
        let average_velocity = total_velocity / count as f32;

        Self {
            elapsed_time: 0.0,
            time: time_control,
            body_count: count,
            average_velocity: average_velocity.to_scalar(),
        }
    }
}

impl Widget for SimulationInfoWidget {
    fn show_widget_in_ui(&self, ctx: &EguiContext, ui: &mut egui::Ui) {
        ui.heading("Simulation Info");
        ui.separator();

        ui.label(format!("Elapsed Time: {:.2} seconds", self.elapsed_time));
        ui.label(format!("Time Acceleration: {:.2}x", self.time.accelerated_time_factor));
        ui.label(format!("Number of Bodies: {}", self.body_count));
        ui.label(format!("Average Velocity: {:.2} m/s", self.average_velocity));

        ui.separator();
        if ui.button("Pause / Resume").clicked() {
            // Could emit a pause event to the simulation
        }
        if ui.button("Reset Simulation").clicked() {
            // Could emit a reset event
        }            
        ui.separator();
    }
}


// 
// Control Bar Widget
// 
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
    fn show_widget_in_ui(&self, ctx: &EguiContext, ui: &mut egui::Ui) {
    // Will be handled by ControlWidget
    }
}
impl ControlWidget for ControlBarWidget{
    fn show_control_widget(&mut self, ctx: &EguiContext) -> WidgetResponse {
        egui::TopBottomPanel::bottom("control_bar").show(ctx, |Ui| {
            Ui.horizontal(|hor_ui|{
                hor_ui.label(RichText::new("  Inertia !  ").font(FontId::proportional(25.0)));
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




// 
// Orbit Visualizer Widget
// 
#[derive(Clone)]
pub struct OrbitVisualizerWidget {
    show_object_creator: bool,
    show_time_controller: bool,
    show_force_manager: bool,
    show_orbit_visualizer: bool,
}
impl OrbitVisualizerWidget {
    pub fn new() -> Self {
        Self {
            show_object_creator: false,
            show_time_controller: false,
            show_force_manager: false,
            show_orbit_visualizer: false,
        }
    }
}
impl Widget for OrbitVisualizerWidget {
    fn show_widget_in_ui(&self, ctx: &EguiContext, ui: &mut egui::Ui) {
        // Will be handled by ControlWidget 
    }
}




// 
// Object Creator Widget
// 
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
                "./data/tex/2k_mercury.jpg".to_string(),
                "./data/tex/mars.jpg".to_string(),
                "./data/tex/earth.jpg".to_string(),
                "./data/tex/jupiter.jpg".to_string(),
            ],
        }
    }
}
impl Widget for ObjectCreatorWidget {
    fn show_widget_in_ui(&self, ctx: &EguiContext, ui: &mut egui::Ui) {
        // Will be handled by ControlWidget
    }
}
impl ControlWidget for ObjectCreatorWidget {
    fn show_control_widget(&mut self, ctx: &EguiContext) -> super::manager::WidgetResponse {
        if !self.visible { return WidgetResponse::None; }

        let mut open = self.visible;
        let mut close_now = false; // separate flag


        let mut response = WidgetResponse::None;
        let window = egui::Window::new("Object Creator")
        .open(&mut open)
        .resizable(true);
        window.show(ctx, |ui| {
            ui.horizontal(|ui| {
            ui.label("Name:");
            ui.text_edit_singleline(&mut self.name);
            });
            
            ui.horizontal(|ui| {
                ui.label("Mass (kg):");
                ui.add(egui::DragValue::new(&mut self.mass).speed(1.0).clamp_range(0.1..=1000.0));
            });
            
            ui.horizontal(|ui| {
                ui.label("Radius (m):");
                ui.add(egui::DragValue::new(&mut self.radius).speed(0.1).clamp_range(0.1..=10.0));
            });
            
            ui.horizontal(|ui| {
                ui.label("Position (x, y, z):");
                ui.add(egui::DragValue::new(&mut self.position[0]).speed(1.0));
                ui.add(egui::DragValue::new(&mut self.position[1]).speed(1.0));
                ui.add(egui::DragValue::new(&mut self.position[2]).speed(1.0));
            });
            
            ui.horizontal(|ui| {
                ui.label("Velocity (x, y, z):");
                ui.add(egui::DragValue::new(&mut self.velocity[0]).speed(0.1));
                ui.add(egui::DragValue::new(&mut self.velocity[1]).speed(0.1));
                ui.add(egui::DragValue::new(&mut self.velocity[2]).speed(0.1));
            });
            
            ui.horizontal(|ui| {
                ui.label("Texture:");
                egui::ComboBox::from_label("Select texture")
                    .selected_text(format!("Texture {}", self.texture_selection + 1))
                    .show_ui(ui, |ui| {
                        for (i, texture) in self.available_textures.iter().enumerate() {
                            ui.selectable_value(&mut self.texture_selection, i, 
                            format!("Texture {} ({})", i + 1, texture.split('/').last().unwrap_or("Unknown")));
                        }
                    });
            });
            
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


// 
// Time Controller Widget
// 
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
    fn show_widget_in_ui(&self, ctx: &EguiContext, ui: &mut egui::Ui) {
        // Will be handled by ControlWidget
    }
}
impl ControlWidget for TimeControllerWidget {
    fn show_control_widget(&mut self, ctx: &EguiContext) -> WidgetResponse {
        if !self.visible { return WidgetResponse::None; }

        let mut response = WidgetResponse::None;
        
        egui::Window::new("Time Ctrl")
            .open(&mut self.visible)
            .resizable(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button(if self.is_paused { "Play" } else { "Pause" }).clicked() {
                        self.is_paused = !self.is_paused;
                        response = if self.is_paused { 
                            WidgetResponse::TimeControl(TimeControlCommand::Pause)
                        } else { 
                            WidgetResponse::TimeControl(TimeControlCommand::Play)
                        };
                    }
                    
                    if ui.button("Reset").clicked() {
                        response = WidgetResponse::TimeControl(TimeControlCommand::Reset);
                    }
                });
                
                ui.horizontal(|ui| {
                    ui.label("Speed:");
                    if ui.button("0.5x").clicked() {
                        response = WidgetResponse::TimeControl(TimeControlCommand::SlowDown(0.5));
                    }
                    if ui.button("2x").clicked() {
                        response = WidgetResponse::TimeControl(TimeControlCommand::SpeedUp(2.0));
                    }
                    if ui.button("5x").clicked() {
                        response = WidgetResponse::TimeControl(TimeControlCommand::SpeedUp(5.0));
                    }
                });
                
                ui.horizontal(|ui| {
                    ui.label("Current Speed:");
                    ui.label(format!("{:.2}x", self.speed_multiplier));
                });
                
                ui.separator();
                ui.label("Time Travel (Experimental)");
                
                ui.horizontal(|ui| {
                    ui.label("Seconds to travel back:");
                    ui.add(egui::DragValue::new(&mut self.time_travel_seconds).speed(0.1).clamp_range(0.1..=60.0));
                });
                
                ui.horizontal(|ui| {
                    if ui.button("Back").clicked() {
                        response = WidgetResponse::TimeControl(TimeControlCommand::GoBackward(self.time_travel_seconds));
                    }
                    ui.label("(Warning: Experimental feature)");
                });
            });
        
        response
    }
}


// 
// Force Manager Widget
// 
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
    fn show_widget_in_ui(&self, ctx: &EguiContext, ui: &mut egui::Ui) {
        // Will be handled by ControlWidget
    }
}
impl ControlWidget for ForceManagerWidget {
    fn show_control_widget(&mut self, ctx: &EguiContext) -> WidgetResponse {
        if !self.visible { return WidgetResponse::None; }

        let mut response = WidgetResponse::None;
        
        egui::Window::new("Force Manager")
            .open(&mut self.visible)
            .resizable(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Target Object:");
                    egui::ComboBox::from_label("Select object")
                        .selected_text(&self.selected_object)
                        .show_ui(ui, |ui| {
                            for object in &self.available_objects {
                                ui.selectable_value(&mut self.selected_object, object.clone(), object);
                            }
                        });
                });
                
                ui.separator();
                ui.label("Global Settings");
                
                ui.horizontal(|ui| {
                    ui.label("Gravity Constant:");
                    if ui.add(egui::DragValue::new(&mut self.gravity_constant).speed(0.1)).changed() {
                        response = WidgetResponse::ForceCommand(ForceCommand::SetGravity(self.gravity_constant));
                    }
                });
                
                ui.separator();
                ui.label("💨 Object Forces");
                
                ui.horizontal(|ui| {
                    ui.label("Damping:");
                    if ui.checkbox(&mut self.damping_enabled, "Enable").changed() {
                        response = WidgetResponse::ForceCommand(ForceCommand::ToggleDamping {
                            target_object: self.selected_object.clone(),
                            enabled: self.damping_enabled,
                        });
                    }
                });
                
                ui.horizontal(|ui| {
                    ui.label("Force Direction (x, y, z):");
                    ui.add(egui::DragValue::new(&mut self.force_direction[0]).speed(0.1));
                    ui.add(egui::DragValue::new(&mut self.force_direction[1]).speed(0.1));
                    ui.add(egui::DragValue::new(&mut self.force_direction[2]).speed(0.1));
                });
                
                ui.horizontal(|ui| {
                    ui.label("Force Magnitude:");
                    ui.add(egui::DragValue::new(&mut self.force_magnitude).speed(0.1).clamp_range(0.0..=100.0));
                });
                
                if ui.button("Apply Force").clicked() && !self.selected_object.is_empty() {
                    let force = Force {
                        direction: Vector(self.force_direction[0], self.force_direction[1], self.force_direction[2]).normalized(),
                        magnitude: self.force_magnitude,
                    };
                    response = WidgetResponse::ForceCommand(ForceCommand::AddForce {
                        target_object: self.selected_object.clone(),
                        force,
                    });
                }
            });
        
        response
    }
}