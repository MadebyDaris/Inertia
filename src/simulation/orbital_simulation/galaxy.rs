
use std::collections::HashMap;

use glium::{Display, glutin::surface::WindowSurface};

use crate::{
    mesh::{sphere::SphereConstructor, MeshObject, ShaderData}, 
    physics::{physicsinterface::OrbitTrail, physicsobject::PhysicsObject, EulerAngles, Force}, 
    simulation::{orbital_simulation::astralBody::{AstralBody, AstralCollisionObject}, simulation::Simulation}, 
    ui::{ForceCommand, ObjectCreationRequest, TimeControlCommand, VisualCommand, WidgetResponse}, 
    utils::vector::Vector
};

pub struct GalaxySimulation {
    pub owned_objects: Vec<AstralBody>,
    pub object_trails: HashMap<String, OrbitTrail>,
    pub object_names: Vec<String>,
    pub paused: bool,
    pub time_multiplier: f32,
    pub gravity_constant: f32,
    pub show_velocity_vectors: bool,
    pub show_force_vectors: bool,
    pub galaxy_center: Vector,
    pub galaxy_radius: f32,
}

impl GalaxySimulation {
    pub fn create_solar_system(&mut self, display: &Display<WindowSurface>) {
        let sphere_constructor = SphereConstructor {
            radius: 3.0,
            longitude: 64,
            latitude: 32,
        };

        // Sun
        let sun_shader = ShaderData {
            tex_filename: "./data/tex/sun.jpg".to_string(),
            vertex_shader: "data/glsl/vertex_shader.glsl".to_string(),
            fragment_shader: "data/glsl/fragment_shader.glsl".to_string(),
        };
        
        let mut sun = sphere_constructor.sphere_physics_object(
            Vector(0.0, 0.0, 0.0),
            8000.0,
            display,
            sun_shader,
        );
        let sunscale = 1.5;
        sun.mesh.translate(0.0, 0.0, 0.0);
        sun.mesh.scale(sunscale, sunscale, sunscale);
        sun.r *= sunscale;
        self.add_object(sun, "Sun".to_string());

        // Ngl use gpt to get values
        // Planet data: (name, distance_from_sun, mass, radius_scale, texture)
        let planet_data = [
            ("Mercury", 20.0, 13.0, 0.6, "./data/tex/mercury.jpg"),
            ("Venus", 30.0, 15.0, 0.7, "./data/tex/venus.jpg"),
            ("Earth", 45.0, 16.0, 0.7, "./data/tex/earth.jpg"),
            ("Mars", 65.0, 10.0, 0.4, "./data/tex/mars.jpg"),
            ("Jupiter", 80.0, 50.0, 1.2, "./data/tex/jupiter.jpg"),
            ("Saturn", 100.0, 40.0, 1.0, "./data/tex/saturn.jpg"),
        ];

        let sun_mass = 10000.0;
        let g = self.gravity_constant;

        for (name, distance, mass, scale, texture) in planet_data.iter() {

            let orbital_speed = (g * sun_mass / distance).sqrt() * 0.9;

            let planet_constructor = SphereConstructor {
                radius: 2.0 * scale,
                longitude: 32,
                latitude: 16,
            };

            let planet_shader = ShaderData {
                tex_filename: texture.to_string(),
                vertex_shader: "data/glsl/vertex_shader.glsl".to_string(),
                fragment_shader: "data/glsl/fragment_shader.glsl".to_string(),
            };

            let mut planet = planet_constructor.sphere_physics_object(
                Vector(0.0, 0.0, orbital_speed),
                *mass,
                display,
                planet_shader,
            );
            
            planet.mesh.translate(*distance, 0.0, 0.0);
            planet.mesh.scale(*scale, *scale, *scale);

            planet.r = 1.0 * scale;

            self.add_object(planet, name.to_string());
        }
    }

    pub fn update_trails(&mut self) {
        for (i, object) in self.owned_objects.iter().enumerate() {
            if let Some(trail) = self.object_trails.get_mut(&self.object_names[i]) {
                trail.add_position(object.position());
            }
        }
    }

    pub fn create_physics_world(&self) -> Vec<&AstralBody> {
        self.owned_objects.iter().collect()
    }
}

// Simplified ts, ts pmo fr
impl Simulation for GalaxySimulation {
    type Object = AstralBody;

    fn new() -> Self {
        return Self {
            owned_objects: Vec::new(),
            object_trails: HashMap::new(),
            object_names: Vec::new(),
            paused: false,
            time_multiplier: 1.,
            gravity_constant: 1.,
            show_velocity_vectors: true,
            show_force_vectors: true,
            galaxy_center: Vector(0.0, 0.0, 0.0),
            galaxy_radius: 1000.0,
        }
    }
    fn update(&mut self, delta_time: f32) {
        if self.paused {
            return;
        }
        let acc_delta = delta_time * self.time_multiplier;

        for i in 0..self.owned_objects.len() {
            self.owned_objects[i].forces.clear();
            self.owned_objects[i].torques.clear();
            
            // Apply damping
            let damping = self.owned_objects[i].damping_force(0.001);
            self.owned_objects[i].add_force(damping);

            // Gravity cuz shit tight
            for j in 0..self.owned_objects.len() {
                if i != j {
                    let other_position = self.owned_objects[j].position();
                    let other_mass = self.owned_objects[j].mass;

                    let distance = (other_position - self.owned_objects[i].position()).magnitude();
                    
                        if distance > (self.owned_objects[i].r + self.owned_objects[j].r) * 0.2 {
                            let g_force = self.owned_objects[i].universal_gravitation_force(
                                &self.owned_objects[j],
                                self.gravity_constant,
                            );
                            self.owned_objects[i].add_force(g_force);

                            // Reduced torque for stability
                            let torque = self.owned_objects[i].calculate_torque(g_force, other_position);
                            self.owned_objects[i].torques.push(torque * 0.1); // Scale down torque
                        }

                    let g_force = self.owned_objects[i].universal_gravitation_force(&self.owned_objects[j], self.gravity_constant);
                    self.owned_objects[i].add_force(g_force);
                    
                    let torque = self.owned_objects[i].calculate_torque(g_force, other_position);
                    self.owned_objects[i].torques.push(torque);
                    }
                }

                // Apply physics updates
                self.owned_objects[i].law_of_momentum();
                self.owned_objects[i].update_velocity(acc_delta);
                self.owned_objects[i].update_geometry(acc_delta);
                self.owned_objects[i].update_orientation(acc_delta);
            }

        for i in 0..self.owned_objects.len() {
            for j in (i + 1)..self.owned_objects.len() {
                if AstralBody::detect_collision(&self.owned_objects[i], &self.owned_objects[j]) {

                    let pos1 = self.owned_objects[i].position();
                    let pos2 = self.owned_objects[j].position();
                    let distance = (pos2 - pos1).magnitude();
                    let min_distance = self.owned_objects[i].r + self.owned_objects[j].r + 0.05;
                    
                    if distance < min_distance && distance > 0.0 {
                        let separation = (min_distance - distance) * 0.5;
                        let direction = (pos2 - pos1).normalized();
                        
                        self.owned_objects[i].mesh.translate(
                            -direction.0 * separation,
                            -direction.1 * separation,
                            -direction.2 * separation,
                        );
                        self.owned_objects[j].mesh.translate(
                            direction.0 * separation,
                            direction.1 * separation,
                            direction.2 * separation,
                        );
                    }

                    let (left, right) = self.owned_objects.split_at_mut(j);
                    AstralBody::handle_collision(&mut left[i], &mut right[0]);
                }
            }
        }
    }

    fn add_object(&mut self, object: AstralBody, name: String) {
        self.owned_objects.push(object);
        self.object_names.push(name.clone());
        self.object_trails.insert(name, OrbitTrail::new(100, [1.0, 1.0, 1.0]));
    }

    fn remove_object(&mut self, name: &str) -> Option<Self::Object>{
        let index = self.object_names.iter().position(|n| n == name);
        if let Some(index) = index {
            let removed_object = self.owned_objects.remove(index);
            self.object_names.remove(index);
            self.object_trails.remove(name);
            Some(removed_object)
        } else {
            None
        }
    }

    fn get_object(&self, name: &str) -> Option<&Self::Object> {
        self.object_names.iter().position(|n| n == name).map(|index| &self.owned_objects[index])
    }
    
    fn get_object_mut(&mut self, name: &str) -> Option<&mut Self::Object> {
      if let Some(index) = self.object_names.iter().position(|n| n == name) {
            Some(&mut self.owned_objects[index])
        } else {
            None
        }
    }
    fn get_objects(&self) -> &Vec<Self::Object> {
        &self.owned_objects
    }

    fn get_object_names(&self) -> &[String] {
        &self.object_names
    }

    fn handle_ui_response(&mut self, response: WidgetResponse, display: &glium::Display<glium::glutin::surface::WindowSurface>) {
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
    
    fn is_paused(&self) -> bool {
        self.paused
    }
    
    fn set_paused(&mut self, paused: bool) {
        self.paused = paused
    }
    
    fn reset(&mut self) {
        self.owned_objects.clear();
        self.object_names.clear();
        self.object_trails.clear();
        self.time_multiplier = 1.0;
        self.paused = false;
    }
}

impl GalaxySimulation {
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
    
    pub fn handle_time_control(&mut self, command: TimeControlCommand) {
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

    pub fn handle_force_command(&mut self, command: ForceCommand) {
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

    pub fn handle_visual_command(&mut self, command: VisualCommand) {
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