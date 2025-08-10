use std::{collections::HashMap, hash::Hash, string};

use crate::{mesh::{sphere::SphereConstructor, ShaderData}, physics::{physicsobject::AstralBody, physicsworld::PhysicsWorld}, ui::{ForceCommand, ObjectCreationRequest, TimeControlCommand, VisualCommand, WidgetResponse}, utils::vector::Vector};

// Orbit trail system for visualizing object paths
#[derive(Clone)]
pub struct OrbitTrail {
    positions: Vec<Vector>,
    max_length: usize,
    color: [f32; 3],
    enabled: bool
}
impl OrbitTrail {
    pub fn new(max_length: usize, color: [f32; 3]) -> Self {
        Self {
            positions: Vec::new(),
            max_length,
            color,
            enabled: false,
        }
    }

    pub fn add_position(&mut self, position: Vector) {
        if !self.enabled { return; }
        
        self.positions.push(position);
        if self.positions.len() > self.max_length {
            self.positions.remove(0);
        }
    }

    pub fn clear(&mut self) {
        self.positions.clear();
    }
}

pub struct SimulationInterface<'a> {
    pub owned_objects: Vec<AstralBody>,
    pub world: PhysicsWorld<'a>,
    pub object_trails: HashMap<String, OrbitTrail>,
    pub object_names: Vec<String>,
    pub paused: bool,
    pub time_multiplier: f32,
    pub gravity_constant: f32,
    pub show_velocity_vectors: bool,
    pub show_force_vectors: bool,
}
impl<'a> SimulationInterface<'a> {
    pub fn new(world: PhysicsWorld<'a>) -> Self {
        let names =(0..world.children.len()).map(|i| format!("AstralBody n{}", i)).collect();
        return Self {
            owned_objects: Vec::new(),
            world,
            object_trails: HashMap::new(),
            object_names: names,
            paused: false,
            time_multiplier: 1.,
            gravity_constant: 1.,
            show_velocity_vectors: true,
            show_force_vectors: true,
        }
    }
    pub fn add_object(&mut self, object: AstralBody, name: String) {
        self.owned_objects.push(object);
        let idx = self.owned_objects.len() - 1;
        let obj_ref: &'a AstralBody = &self.owned_objects[self.owned_objects.len() - 1];
        self.world.children.push(obj_ref);
        self.object_names.push(name.clone());
        self.object_trails.insert(name, OrbitTrail::new(100, [1., 1., 1.]));
    }
    pub fn update_trails(&mut self) {
        for (i, object) in self.world.children.iter().enumerate() {
            if let Some(trail) = self.object_trails.get_mut(&self.object_names[i]) {
                trail.add_position(object.position());
            }
        }
    }
    pub fn handle_ui_response(&mut self, response: WidgetResponse, display: &glium::Display<glium::glutin::surface::WindowSurface>) {
        match response {
            WidgetResponse::CreateObject(request) => {
                self.create_object_from_request(request, display);
            },
            WidgetResponse::TimeControl(command) => {
                self.handle_time_control(command);
            },
            WidgetResponse::ForceCommand(command) => {
                self.handle_force_command(command);
            },
            WidgetResponse::VisualCommand(command) => {
                self.handle_visual_command(command);
            },
            _ => {}
        }
    }

    pub fn create_object_from_request(
        &mut self,
        request: ObjectCreationRequest,
        display: &glium::Display<glium::glutin::surface::WindowSurface>,
    ) {
        let sphere_constructor = SphereConstructor {
            radius: request.radius,
            longitude: 32,
            latitude: 16,
        };

        let shader_data = ShaderData {
            tex_filename: request.texture_path,
            vertex_shader: "data/glsl/vertex_shader.glsl".to_string(),
            fragment_shader: "data/glsl/fragment_shader.glsl".to_string(),
        };

        let mut new_object = sphere_constructor.sphere_physics_object(
            request.velocity,
            request.mass,
            display,
            shader_data,
        );

        new_object.mesh.translate(request.position.0, request.position.1, request.position.2);

        // Here we pass ownership — no lifetime issue
        self.add_object(new_object, request.name);
    }
    fn handle_time_control(&mut self, command: TimeControlCommand) {
        match command {
            TimeControlCommand::Play => self.paused = false,
            TimeControlCommand::Pause => self.paused = true,
            TimeControlCommand::SpeedUp(factor) => {
                self.time_multiplier *= factor;
            },
            TimeControlCommand::SlowDown(factor) => {
                self.time_multiplier /= factor;
            },
            TimeControlCommand::Reset => {
                self.time_multiplier = 1.0;
                // Reset object positions and velocities to initial state
                // This would require storing initial states
            },
            TimeControlCommand::GoBackward(_seconds) => {
                // Time travel implementation would require storing simulation history
                println!("Time travel not yet implemented - would need state history");
            },
        }
    }
    fn handle_force_command(&mut self, command: ForceCommand) {
        match command {
            ForceCommand::SetGravity(new_g) => {
                self.gravity_constant = new_g;
            },
            ForceCommand::AddForce { target_object, force } => {
                if let Some(index) = self.object_names.iter().position(|name| name == &target_object) {
                    self.world.children[index].add_force(force);
                }
            },
           ForceCommand::ToggleDamping { target_object, enabled } => {
                if let Some(index) = self.object_names.iter().position(|name| name == &target_object) {
                    println!("Toggle damping for {}: {}", target_object, enabled);
                }
            },
            ForceCommand::RemoveForce { target_object, force_id } => {
                println!("Remove force {} from {}", force_id, target_object);
            },
        }
    }

    fn handle_visual_command(&mut self, command: VisualCommand) {
        match command {
            VisualCommand::ToggleOrbitTrail { object_name, enabled } => {
                if let Some(trail) = self.object_trails.get_mut(&object_name) {
                    trail.enabled = enabled;
                }
            },
            VisualCommand::ClearAllTrails => {
                for trail in self.object_trails.values_mut() {
                    trail.clear();
                }
            },
            VisualCommand::SetTrailLength(length) => {
                for trail in self.object_trails.values_mut() {
                    trail.max_length = length;
                }
            },
            VisualCommand::ToggleVelocityVectors(enabled) => {
                self.show_velocity_vectors = enabled;
            },
            VisualCommand::ToggleForceVectors(enabled) => {
                self.show_force_vectors = enabled;
            },
        }
    }
}