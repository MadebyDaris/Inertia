use std::vec;

use egui::{Context as EguiContext, Ui, ViewportId};
use egui_glium::EguiGlium;
use glium::{glutin::surface::WindowSurface, winit::{event_loop::EventLoop, window::Window}, Display};
use crate::ui::control_requests::*;

use crate::ui::widgets::*;

// Describes certain actions that can be taken 
// These are all the commands
#[derive(Debug, Clone)]
pub enum WidgetResponse {
    None,
    CreateObject(ObjectCreationRequest),
    TimeControl(TimeControlCommand),
    ForceCommand(ForceCommand),
    VisualCommand(VisualCommand),
}

// The manager object that runs each of the wigets
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
// This is to seperate some widgets that dont have a set Widget response
pub trait Widget: Clone {
    fn show_widget_in_ui(&self, ctx: &EguiContext, ui: &mut egui::Ui);
}

pub trait ControlWidget {
    fn show_control_widget(&mut self, ctx: &EguiContext) -> WidgetResponse;
}

// These are all the widgets may be used for some implemenations by user
// And help organize da widgets
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
    pub fn show(&self, ctx: &EguiContext, ui: &mut  egui::Ui) {
        match self {
            WidgetEnum::AstralBodyInfo(widget) => widget.show_widget_in_ui(ctx, ui),
            WidgetEnum::SimulationInfo(widget) => widget.show_widget_in_ui(ctx, ui),
            WidgetEnum::ControlBar(widget) => widget.show_widget_in_ui(ctx, ui),
            WidgetEnum::ObjectCreator(widget) => widget.show_widget_in_ui(ctx, ui),
            WidgetEnum::TimeController(widget) => widget.show_widget_in_ui(ctx, ui),
            WidgetEnum::ForceManager(widget) => widget.show_widget_in_ui(ctx, ui),
            WidgetEnum::OrbitVisualizer(widget) => widget.show_widget_in_ui(ctx, ui),
        }
    }
}

// 
// WIDGET MANAGER
// 
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
        F: FnMut(&EguiContext, &mut Ui),
    {
        let mut responses: Vec<WidgetResponse> = Vec::new();
        self.egui.run(window, |egui_context| {


            // Render control widgets and collect responses
            let control_response = ControlWidget::show_control_widget(&mut self.control_bar, egui_context);
            responses.push(control_response.clone());
            
            // Update widget visibility based on control bar state
            self.object_creator.visible = self.control_bar.show_object_creator;
            self.time_controller.visible = self.control_bar.show_time_controller;
            self.force_manager.visible = self.control_bar.show_force_manager;
            // self.orbit_visualizer.visible = self.control_bar.show_orbit_visualizer;
            

            if self.object_creator.visible {
                let creator_response = self.object_creator.show_control_widget(egui_context);
                responses.push(creator_response);
            }

            // if self.time_controller.visible {
            //     responses.push(ControlWidget::show_control_widget(&mut self.time_controller, egui_context));
            // }
            
            // if self.force_manager.visible {
            //     responses.push(ControlWidget::show_control_widget(&mut self.force_manager, egui_context));
            // }
            
            // if self.orbit_visualizer.visible {
            //     responses.push(ControlWidget::show_control_widget(&mut self.orbit_visualizer, egui_context));
            // }

            // Render additional widgets
            egui::SidePanel::left("main_side_panel").show(egui_context, |ui| {
                    // Run the provided widget rendering function
                    render_widgets(egui_context, ui);

                    for widget in &self.widgets {
                        widget.show(egui_context, ui);
                    }
                });
            });

        self.egui.paint(display, frame);
        return responses.into_iter().filter(|r| !matches!(r, WidgetResponse::None)).collect::<Vec<WidgetResponse>>();
    }
}