use std::vec;

use egui::{Context as EguiContext, Ui, ViewportId};
use egui_glium::EguiGlium;
use glium::{glutin::surface::WindowSurface, winit::{event_loop::EventLoop, window::Window}, Display};
use crate::ui::control_requests::*;

use crate::ui::widgets::*;

// Describes certain actions that can be taken 
#[derive(Debug, Clone)]
pub enum WidgetResponse {
    None,
    CreateObject(ObjectCreationRequest),
    TimeControl(TimeControlCommand),
    ForceCommand(ForceCommand),
    VisualCommand(VisualCommand),
}

// Implement such that if empty do nothing and remove it
pub struct WidgetManager {
    pub egui: EguiGlium,
    pub widgets: Vec<WidgetEnum>,
    pub control_bar: ControlBarWidget,
    pub object_creator: ObjectCreatorWidget,
    pub time_controller: TimeControllerWidget,
    pub force_manager: ForceManagerWidget,
    pub orbit_visualizer: OrbitVisualizerWidget,
}

// 
// Defining two different types of widgets
// 
pub trait Widget: Clone {
    fn show(&self, ctx: &EguiContext);
}

pub trait ControlWidget {
    fn show(&mut self, ctx: &EguiContext) -> WidgetResponse;
}

#[allow(dead_code)]
pub enum WidgetEnum {
    AstralBodyInfo(AstralBodyInfoWidget),
    SimulationInfo(SimulationInfoWidget),
    ControlBar(ControlBarWidget),
    ObjectCreator(ObjectCreatorWidget),
    TimeController(TimeControllerWidget),
    ForceManager(ForceManagerWidget),
    OrbitVisualizer(OrbitVisualizerWidget),
}


impl WidgetEnum {
    pub fn show(&self, ctx: &EguiContext) {
        match self {
            WidgetEnum::AstralBodyInfo(widget) => widget.show(ctx),
            WidgetEnum::SimulationInfo(widget) => widget.show(ctx),
            WidgetEnum::ControlBar(widget) => widget.show(ctx),
            WidgetEnum::ObjectCreator(widget) => widget.show(ctx),
            WidgetEnum::TimeController(widget) => widget.show(ctx),
            WidgetEnum::ForceManager(widget) => widget.show(ctx),
            WidgetEnum::OrbitVisualizer(widget) => widget.show(ctx),
        }
    }
}

impl WidgetManager {
    pub fn new( display: &Display<WindowSurface>, window: &Window, event_loop: &EventLoop<()>)  -> Self{
        let egui_glium: EguiGlium = EguiGlium::new(ViewportId::ROOT, display, window, event_loop);
        Self {
            egui: egui_glium,
            widgets: vec![],
            control_bar: ControlBarWidget::new(),
            object_creator: ObjectCreatorWidget::new(),
            time_controller: TimeControllerWidget::new(),
            force_manager: ForceManagerWidget::new(),
            orbit_visualizer: OrbitVisualizerWidget::new(),
        }
    }
    pub fn render_ui<F>(&mut self, window: &Window, display: &Display<WindowSurface>, frame: &mut glium::Frame, mut render_widgets: F) -> Vec<WidgetResponse>
    where
        F: FnMut(&EguiContext),
    {
        let mut responses: Vec<WidgetResponse> = Vec::new();

        self.egui.run(window, |egui_context| {
            // Run the provided widget rendering function
            render_widgets(egui_context);

                        // Render control widgets and collect responses
            responses.push(ControlWidget::show(&mut self.control_bar, egui_context));
            responses.push(ControlWidget::show(&mut self.object_creator, egui_context));
            // responses.push(ControlWidget::show(&mut self.time_controller, egui_context));
            // responses.push(ControlWidget::show(&mut self.force_manager, egui_context));
            // responses.push(ControlWidget::show(&mut self.orbit_visualizer, egui_context));

                        // Render additional widgets
            for widget in &self.widgets {
                widget.show(egui_context);
            }

        });

        self.egui.paint(display, frame);
        return responses.into_iter().filter(|r| !matches!(r, WidgetResponse::None)).collect::<Vec<WidgetResponse>>();
    }
}