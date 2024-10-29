use egui::{Context as EguiContext, Ui, ViewportId};
use egui_glium::EguiGlium;
use glium::{glutin::surface::WindowSurface, winit::{event_loop::EventLoop, window::Window}, Display};
use crate::physics::physicsobject::AstralBody;

pub struct MyUI {
    pub egui: EguiGlium,
}

pub trait Widget: Clone {
    fn show(&self, ctx: &EguiContext);
}

#[allow(dead_code)]
pub enum WidgetEnum {
    AstralBodyInfo(AstralBodyInfoWidget),
    SimulationInfo(SimulationInfoWidget),
}


impl WidgetEnum {
    #[allow(dead_code)]
    pub fn show(&self, ctx: &EguiContext) {
        match self {
            WidgetEnum::AstralBodyInfo(widget) => widget.show(ctx),
            WidgetEnum::SimulationInfo(widget) => widget.show(ctx),
        }
    }
}

impl MyUI {
    pub fn new( display: &Display<WindowSurface>, window: &Window, event_loop: &EventLoop<()>)  -> Self{
        let egui_glium: EguiGlium = EguiGlium::new(ViewportId::ROOT, display, window, event_loop);
        Self {
            egui: egui_glium,
        }
    }
    pub fn render_ui<F>(&mut self, window: &Window, display: &Display<WindowSurface>, frame: &mut glium::Frame, mut render_widgets: F)
    where
        F: FnMut(&EguiContext),
    {
        // Run egui if a repaint is needed
        self.egui.run(window, |egui_context| {
            // Run the provided widget rendering function
            render_widgets(egui_context);
        });

        self.egui.paint(display, frame);

    }
}


//
// Some Widget Examples
//

// Body info
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
    fn show(&self, ctx: &EguiContext) {
        egui::Window::new(format!("Information about {}", self.name)).show(ctx, |ui| {
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
// Simulation info
#[derive(Clone)]
pub struct SimulationInfoWidget {
    pub elapsed_time: f32,
}

impl Widget for SimulationInfoWidget {
    fn show(&self, ctx: &EguiContext) {
        egui::Window::new("Simulation information:").show(ctx, |ui: &mut Ui| {
            ui.heading("Simulation Information");
            ui.label(format!("Elapsed Time: {:.2} seconds", self.elapsed_time));
        });
    }
}