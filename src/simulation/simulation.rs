use std::collections::HashMap;

use crate::{
    mesh::{sphere::SphereConstructor, MeshObject, ShaderData}, 
    physics::{physicsinterface::OrbitTrail, physicsobject::PhysicsObject, physicsworld::PhysicsWorld, EulerAngles, Force}, 
    simulation::orbital_simulation::astralBody::{AstralBody, AstralCollisionObject, AstralPhysicsObject}, 
    ui::{ForceCommand, ObjectCreationRequest, TimeControlCommand, VisualCommand, WidgetResponse}, 
    utils::vector::Vector
};


pub struct Simulation {
    pub owned_objects: Vec<AstralBody>,
    pub object_trails: HashMap<String, OrbitTrail>,
    pub object_names: Vec<String>,
    pub paused: bool,
    pub time_multiplier: f32,
    pub gravity_constant: f32,
    pub show_velocity_vectors: bool,
    pub show_force_vectors: bool,
}
impl<'a> Simulation {
    pub fn new() -> Self {
        return Self {
            owned_objects: Vec::new(),
            object_trails: HashMap::new(),
            object_names: Vec::new(),
            // Names seems to complicate stuff and lazy as hell to do thart but I'll do that next time
            // Sum kinda function that renames them all or sum shi.
            paused: false,
            time_multiplier: 1.,
            gravity_constant: 1.,
            show_velocity_vectors: true,
            show_force_vectors: true,
        }
    }
// Simplified ts, ts pmo fr
    pub fn add_object(&mut self, object: AstralBody, name: String) {
        self.owned_objects.push(object);
        self.object_names.push(name.clone());
        self.object_trails.insert(name, OrbitTrail::new(100, [1.0, 1.0, 1.0]));
    }

    pub fn update_trails(&mut self) {
        for (i, object) in self.owned_objects.iter().enumerate() {
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
                    self.owned_objects[index].add_force(force);
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
        // Add a helper method to create objects with transforms
    pub fn create_and_add_sphere(
        &mut self,
        name: String,
        constructor: &SphereConstructor,
        velocity: Vector,
        mass: f32,
        display: &glium::Display<glium::glutin::surface::WindowSurface>,
        shader_data: ShaderData,
        translation: (f32, f32, f32),
        rotation: Option<(f32, f32, f32)>,
        scale: Option<(f32, f32, f32)>,
    ) {
        let mut object = constructor.sphere_physics_object(velocity, mass, display, shader_data);
        
        object.mesh.translate(translation.0, translation.1, translation.2);
        
        if let Some((rx, ry, rz)) = rotation {
            object.mesh.rotate(rx, ry, rz);
        }
        
        if let Some((sx, sy, sz)) = scale {
            object.mesh.scale(sx, sy, sz);
        }
        
        self.add_object(object, name);
    }

    // Add a method to create a PhysicsWorld for rendering
    pub fn get_astral_bodies(&self) -> Vec<&AstralBody> {
        self.owned_objects.iter().collect()
    }
    pub fn get_physics_object_refs(&self) -> Vec<
        &dyn PhysicsObject<
            Mesh = MeshObject, 
            Velocity = Vector, 
            Acceleration = Vector, 
            AngularVelocity = Vector, 
            AngularAcceleration = Vector, 
            Mass = f32, 
            Forces = Vec<Force>, 
            Torques = Vec<Vector>, 
            MomentOfInertia = f32, 
            EulerAngles = EulerAngles, 
            CollisionObject = AstralCollisionObject>> {
        let object_refs= self
            .get_astral_bodies()
            .into_iter()
            .map(|obj| obj as &dyn PhysicsObject<
            Mesh = MeshObject, 
            Velocity = Vector, 
            Acceleration = Vector, 
            AngularVelocity = Vector, 
            AngularAcceleration = Vector, 
            Mass = f32, 
            Forces = Vec<Force>, 
            Torques = Vec<Vector>, 
            MomentOfInertia = f32, 
            EulerAngles = EulerAngles, 
            CollisionObject = AstralCollisionObject>)
            .collect();
        object_refs
    }
}