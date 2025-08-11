use glium::winit::event::{self, ElementState, WindowEvent};

use crate::render::{Camera, CameraMat};
use crate::simulation::timeutil::SimulationTime;
use crate::ui::WidgetManager;

use super::{app::Action};
#[macro_export]
macro_rules! handle_input {
    ($self:ident, $event:ident, $( ($key:ident, $state:ident) ),* ) => {
        use glium::winit::{event::ElementState, keyboard};
        let pressed = $event.state == ElementState::Pressed;
        let key = $event.physical_key;

        match key {
            $( glium::winit::keyboard::PhysicalKey::Code(keyboard::KeyCode::$key) => {$self.$state = pressed
        })*
            _ => (),
        }
    };
}

pub fn handle_events(windows: &glium::winit::window::Window, events: &Vec<event::Event<()>>, _camera: &mut Camera,camera_mat: &mut CameraMat, time: &mut SimulationTime, widget_manager: &mut WidgetManager) -> Action {
    let mut action = Action::Continue;
    for event in events {

        match event {
            // Handle device events (e.g., mouse, keyboard) and update camera view
            event::Event::DeviceEvent { event, .. } => {
                _camera.look_at(&event); // Handle Device Events
                _camera.update();

            }
            // Handle window events
            event::Event::WindowEvent { event, .. } => {
                widget_manager.egui.on_event(windows, event);
                match event {
                    WindowEvent::Resized(size) => {
                        // Update the projection matrix only
                        camera_mat.pers_mat = _camera.get_perspective(
                            size.width as f32 / size.height as f32,
                            45.0,
                            0.1,
                            100.0);
                    }
                    WindowEvent::CloseRequested => {
                        action = Action::Stop; // Stop the application
                    }
                    WindowEvent::KeyboardInput { event, .. } => {
                        time.input(event);
                        _camera.input(event); // Pass keyboard input to camera
                        if event.state == ElementState::Pressed 
                           && event.logical_key == glium::winit::keyboard::Key::Named(glium::winit::keyboard::NamedKey::Escape) {
                            action = Action::Stop; // Stop on Escape key press
                        }
                    }
                    _ => {
                    }
                }
            }
            _ => (),
        }
    }
    action
}