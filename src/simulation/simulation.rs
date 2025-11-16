use glium::{glutin::surface::WindowSurface, Display};

use crate::{physics::physicsobject::PhysicsObject, render::shader_system::ShaderManager, ui::WidgetResponse};


pub trait Simulation {
    type Object: PhysicsObject;
    
    fn new() -> Self where Self: Sized;
    fn update(&mut self, delta_time: f32);
    fn add_object(&mut self, object: Self::Object, name: String);
    fn remove_object(&mut self, name: &str) -> Option<Self::Object>;
    fn get_object(&self, name: &str) -> Option<&Self::Object>;
    fn get_object_mut(&mut self, name: &str) -> Option<&mut Self::Object>;
    fn get_objects(&self) -> &Vec<Self::Object>;
    fn get_object_names(&self) -> &[String];
    fn handle_ui_response(&mut self, response: WidgetResponse, display: &Display<WindowSurface>, shader_manager: &ShaderManager);
    fn is_paused(&self) -> bool;
    fn set_paused(&mut self, paused: bool);
    fn reset(&mut self);
}
