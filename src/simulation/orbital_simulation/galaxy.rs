
use std::collections::HashMap;

use glium::{Display, glutin::surface::WindowSurface};

use crate::{
    mesh::{sphere::SphereConstructor, MeshObject, ShaderData}, 
    physics::{physicsinterface::OrbitTrail, physicsobject::PhysicsObject, EulerAngles, Force}, 
    render::{scene_ui::SceneUIObject, shader_system::{ShaderType, MaterialProperties, ShaderManager}},
    simulation::{orbital_simulation::astralBody::{AstralBody, AstralCollisionObject}, simulation::Simulation}, 
    ui::{ForceCommand, ObjectCreationRequest, TimeControlCommand, VisualCommand, WidgetResponse}, 
    utils::vector::Vector
};

pub struct GalaxySimulation {
    pub owned_objects: Vec<AstralBody>,
    pub scene_ui_objects: Vec<SceneUIObject>,
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
    pub fn create_simple_system(&mut self, display: &Display<WindowSurface>, shader_manager: &mut ShaderManager) {
        let sphere_constructor = SphereConstructor {
            radius: 3.0,
            longitude: 32,
            latitude: 16,
        };


        self.gravity_constant = 6.0;

        shader_manager.initialize_shaders(display);
        let sun_shader = ShaderData {
            tex_filename: "./data/tex/sun.jpg".to_string(),
            shader_type: ShaderType::Emission,
            material: MaterialProperties {
                color: [1.0, 0.95, 0.8],           // Warm yellowish tint
                emission_strength: 2.5,             // Very bright emission
                ..Default::default()
            },
        };
        
        let sun_mass = 10000.0;
        let mut sun = sphere_constructor.sphere_physics_object(
            Vector(0.0, 0.0, 0.0),
            sun_mass,
            display,
            sun_shader,
            shader_manager,
        );
        sun.mesh.scale(3.0, 3.0, 3.0);
        sun.r *= 3.0;
        sun.velocity = Vector(0.0, 0.0, 0.0);
        self.add_object(sun, "Sun".to_string());

        // Planet 1 - Inner orbit (Earth-like) - Using Diffuse shader
        let planet1_shader = ShaderData {
            tex_filename: "./data/tex/earth.jpg".to_string(),
            shader_type: ShaderType::Diffuse,
            material: MaterialProperties {
                color: [0.95, 0.98, 1.0],          // Slight blue atmospheric tint
                ambient_occlusion: 1.0,
                ..Default::default()
            },
        };
        
        let distance1 = 40.0;
        let pos1 = Vector(distance1, 0.0, 0.0);
        
        // Calculate proper orbital velocity: v = sqrt(G * M / r)
        let orbital_speed1 = (self.gravity_constant * sun_mass / distance1).sqrt();
        
        let mut planet1 = sphere_constructor.sphere_physics_object(
            Vector(0.0, 0.0, 0.0), // velocity placeholder
            1.0,
            display,
            planet1_shader,
            shader_manager
        );
        // Set initial position
        planet1.mesh.translate(pos1.0, pos1.1, pos1.2);
        // Set orbital velocity
        planet1.velocity = Vector(0.0, 0.0, orbital_speed1);
        planet1.mesh.scale(1.0, 1.0, 1.0);
        planet1.r *= 1.0;
        self.add_object(planet1, "Earth".to_string());

        // Planet 2 - Outer orbit (Mars-like) - Using Diffuse shader with red tint
        let planet2_shader = ShaderData {
            tex_filename: "./data/tex/mars.jpg".to_string(),
            shader_type: ShaderType::Diffuse,
            material: MaterialProperties {
                color: [1.0, 0.85, 0.75],          // Reddish-orange tint
                ambient_occlusion: 0.9,
                ..Default::default()
            },
        };
        
        let distance2 = 70.0;
        let pos2 = Vector(distance2, 0.0, 0.0);
        
        // Calculate proper orbital velocity for outer planet
        let orbital_speed2 = (self.gravity_constant * sun_mass / distance2).sqrt();
        
        let mut planet2 = sphere_constructor.sphere_physics_object(
            Vector(0.0, 0.0, 0.0), // velocity placeholder
            0.6,
            display,
            planet2_shader,
            shader_manager
        );
        // Set initial position
        planet2.mesh.translate(pos2.0, pos2.1, pos2.2);
        // Set orbital velocity
        planet2.velocity = Vector(0.0, 0.0, orbital_speed2);
        planet2.mesh.scale(0.8, 0.8, 0.8);
        planet2.r *= 0.8;
        self.add_object(planet2, "Mars".to_string());
    }

    pub fn create_solar_system(&mut self, display: &Display<WindowSurface>, shader_manager: &ShaderManager) {
        let sphere_constructor = SphereConstructor {
            radius: 3.0,
            longitude: 64,
            latitude: 32,
        };

        // Sun - Using Emission shader
        let sun_shader = ShaderData {
            tex_filename: "./data/tex/sun.jpg".to_string(),
            shader_type: ShaderType::Emission,
            material: MaterialProperties {
                color: [1.0, 0.95, 0.8],
                emission_strength: 2.5,
                ..Default::default()
            },
        };
        let mut sun = sphere_constructor.sphere_physics_object(
            Vector(0.0, 0.0, 0.0),
            8000.0,
            display,
            sun_shader,
            shader_manager
        );
        let sunscale = 1.5;
        sun.mesh.translate(0.0, 0.0, 0.0);
        sun.mesh.scale(sunscale, sunscale, sunscale);
        sun.r *= sunscale;
        self.add_object(sun, "Sun".to_string());

        // Ngl used gpt to get values
        // Planet data: (name, distance_from_sun, mass, radius_scale, texture)
        let planet_data = [
            ("Mercury", 20.0, 13.0, 0.6, "./data/tex/mercury.jpg"),
            ("Venus", 30.0, 15.0, 0.7, "./data/tex/venus.jpg"),
            ("Earth", 45.0, 16.0, 0.7, "./data/tex/earth.jpg"),
            ("Mars", 65.0, 10.0, 0.4, "./data/tex/mars.jpg"),
            ("Jupiter", 80.0, 30.0, 1.0, "./data/tex/jupiter.jpg"),
            ("Saturn", 100.0, 50.0, 1.0, "./data/tex/saturn.jpg"),
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

            // Choose shader type based on planet characteristics
            let (shader_type, material) = match name.as_ref() {
                "Jupiter" | "Saturn" => (
                    ShaderType::Glossy,
                    MaterialProperties {
                        color: [1.0, 1.0, 1.0],
                        specular_intensity: 0.6,
                        shininess: 48.0,
                        ..Default::default()
                    }
                ),
                "Mars" => (
                    ShaderType::Diffuse,
                    MaterialProperties {
                        color: [1.0, 0.85, 0.75],
                        ambient_occlusion: 0.9,
                        ..Default::default()
                    }
                ),
                "Earth" => (
                    ShaderType::Glossy,
                    MaterialProperties {
                        color: [0.95, 0.98, 1.0],
                        specular_intensity: 0.4,
                        shininess: 32.0,
                        ..Default::default()
                    }
                ),
                _ => (
                    ShaderType::Diffuse,
                    MaterialProperties::default()
                )
            };

            let planet_shader = ShaderData {
                tex_filename: texture.to_string(),
                shader_type,
                material,
            };
            let mut planet = planet_constructor.sphere_physics_object(
                Vector(0.0, 0.0, orbital_speed),
                *mass,
                display,
                planet_shader,
                shader_manager,
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
            scene_ui_objects: Vec::new(),
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

            // Gravity between objects
            for j in 0..self.owned_objects.len() {
                if i != j {
                    let other_position = self.owned_objects[j].position();
                    let _other_mass = self.owned_objects[j].mass;

                    let distance = (other_position - self.owned_objects[i].position()).magnitude();
                    
                    // Only apply gravity if objects are not too close (prevents stuck together)
                    let min_safe_distance = (self.owned_objects[i].r + self.owned_objects[j].r) * 1.5;
                    
                    if distance > min_safe_distance {
                        let g_force = self.owned_objects[i].universal_gravitation_force(
                            &self.owned_objects[j],
                            self.gravity_constant,
                        );
                        self.owned_objects[i].add_force(g_force);

                        // Reduced torque for stability
                        let torque = self.owned_objects[i].calculate_torque(g_force, other_position);
                        self.owned_objects[i].torques.push(torque * 0.1);
                    }
                }
            }

            // Apply physics updates
            self.owned_objects[i].law_of_momentum();
            self.owned_objects[i].update_velocity(acc_delta);
            self.owned_objects[i].update_geometry(acc_delta);
            self.owned_objects[i].update_orientation(acc_delta);
        }

        // Update scene UI objects to follow the bodies
        while self.scene_ui_objects.len() < self.owned_objects.len() {
            self.scene_ui_objects.push(SceneUIObject::new(Vector(0.0, 0.0, 0.0)));
        }
        while self.scene_ui_objects.len() > self.owned_objects.len() {
            self.scene_ui_objects.pop();
        }
        
        for i in 0..self.owned_objects.len() {
            self.scene_ui_objects[i].update(
                self.owned_objects[i].position(),
                self.owned_objects[i].velocity,
                self.owned_objects[i].r
            );
        }

        for i in 0..self.owned_objects.len() {
            for j in (i + 1)..self.owned_objects.len() {
                if AstralBody::detect_collision(&self.owned_objects[i], &self.owned_objects[j]) {

                    let pos1 = self.owned_objects[i].position();
                    let pos2 = self.owned_objects[j].position();
                    let distance = (pos2 - pos1).magnitude();
                    let min_distance = self.owned_objects[i].r + self.owned_objects[j].r + 0.5;
                    
                    if distance < min_distance && distance > 0.0 {
                        let separation = (min_distance - distance) * 0.6;
                        let direction = (pos2 - pos1).normalized();
                        
                        // Push objects apart more forcefully
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
                        
                        // Add separation velocity to prevent sticking
                        let separation_velocity = direction * 0.5;
                        self.owned_objects[i].velocity = self.owned_objects[i].velocity - separation_velocity;
                        self.owned_objects[j].velocity = self.owned_objects[j].velocity + separation_velocity;
                    }

                    let (left, right) = self.owned_objects.split_at_mut(j);
                    AstralBody::handle_collision(&mut left[i], &mut right[0]);
                }
            }
        }
    }

    fn add_object(&mut self, object: AstralBody, name: String) {
        self.owned_objects.push(object);
        self.scene_ui_objects.push(SceneUIObject::new(Vector(0.0, 0.0, 0.0)));
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

    fn handle_ui_response(&mut self, response: WidgetResponse, display: &glium::Display<glium::glutin::surface::WindowSurface>, shader_manager: &ShaderManager) {
        match response {
            WidgetResponse::CreateObject(request) => {
                self.create_object_from_request(request, display, shader_manager);
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
    fn create_object_from_request(
        &mut self,
        request: ObjectCreationRequest,
        display: &glium::Display<glium::glutin::surface::WindowSurface>,
        shader_manager: &ShaderManager,
    ) {
        let sphere_constructor = SphereConstructor {
            radius: request.radius,
            longitude: 32,
            latitude: 16,
        };

        let shader_data = ShaderData {
            tex_filename: request.texture_path,
            shader_type: ShaderType::Diffuse,
            material: MaterialProperties::default(),
        };
        let mut new_object = sphere_constructor.sphere_physics_object(
            request.velocity,
            request.mass,
            display,
            shader_data,
            shader_manager,
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
            TimeControlCommand::SetSpeed(speed) => {
                self.time_multiplier = speed;
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
                if let Some(_index) = self.object_names.iter().position(|name| name == &target_object) {
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
            VisualCommand::ToggleTrails(enabled) => {
                // Toggle all trails
                for ui_obj in &mut self.scene_ui_objects {
                    ui_obj.trail.enabled = enabled;
                }
            },
            VisualCommand::ClearTrails => {
                // Clear all trails
                for ui_obj in &mut self.scene_ui_objects {
                    ui_obj.trail.clear();
                }
            },
            VisualCommand::ClearAllTrails => {
                for trail in self.object_trails.values_mut() {
                    trail.clear();
                }
                // Also clear scene UI trails
                for ui_obj in &mut self.scene_ui_objects {
                    ui_obj.trail.clear();
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
            VisualCommand::ToggleVelocityArrows(enabled) => {
                // Toggle velocity arrows for all objects
                for ui_obj in &mut self.scene_ui_objects {
                    ui_obj.velocity_arrow.enabled = enabled;
                }
            },
            VisualCommand::ToggleAccelerationArrows(enabled) => {
                // Toggle acceleration arrows for all objects
                for ui_obj in &mut self.scene_ui_objects {
                    ui_obj.acceleration_arrow.enabled = enabled;
                }
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
        let mut shader_manager = ShaderManager::new();
        shader_manager.initialize_shaders(display);
        let mut object = constructor.sphere_physics_object(velocity, mass, display, shader_data, &shader_manager);
        
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