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
    pub mass: f32,
    pub position: String,
    pub velocity: String,
    pub name: String,
}

impl AstralBodyInfoWidget {
    pub fn new(astral_body: &AstralBody, name: String) -> Self {
        Self {
            name,
            mass: astral_body.mass,
            position: astral_body.position_str(),
            velocity: astral_body.velocity_str(),
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