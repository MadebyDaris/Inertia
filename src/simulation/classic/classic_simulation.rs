use crate::{simulation::{classic::rigidbody, simulation::Simulation}, utils::vector::Vector};

pub struct ClassicSimulation {
    pub owned_objects: Vec<rigidbody::Rigidbody>,
    pub object_names: Vec<String>,
    pub paused: bool,
    pub time_multiplier: f32,
    pub gravity_constant: f32,
    pub show_velocity_vectors: bool,
    pub show_force_vectors: bool,
    pub WorldOrigin: Vector,
}
impl Simulation for ClassicSimulation {
    type Object = rigidbody::Rigidbody;

    fn new() -> Self where Self: Sized {
        todo!()
    }

    fn update(&mut self, delta_time: f32) {
        todo!()
    }

    fn add_object(&mut self, object: Self::Object, name: String) {
        todo!()
    }

    fn remove_object(&mut self, name: &str) -> Option<Self::Object> {
        todo!()
    }

    fn get_object(&self, name: &str) -> Option<&Self::Object> {
        todo!()
    }

    fn get_object_mut(&mut self, name: &str) -> Option<&mut Self::Object> {
        todo!()
    }

    fn get_objects(&self) -> &Vec<Self::Object> {
        todo!()
    }

    fn get_object_names(&self) -> &[String] {
        todo!()
    }

    fn handle_ui_response(&mut self, _response: crate::ui::WidgetResponse, _display: &glium::Display<glium::glutin::surface::WindowSurface>, _shader_manager: &crate::render::shader_system::ShaderManager) {
        todo!()
    }

    fn is_paused(&self) -> bool {
        todo!()
    }

    fn set_paused(&mut self, paused: bool) {
        todo!()
    }

    fn reset(&mut self) {
        todo!()
    }
}