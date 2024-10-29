use glium::{glutin::surface::WindowSurface, Display};
use sphere::SphereConstructor;

use crate::physics::Force;

use crate::{mesh::*, utils::{matrix::TransformMatrix, ui::{AstralBodyInfoWidget, Widget}}};
use super::{position_euclidean, Vector};

#[macro_export]
macro_rules! calculate_g_forces {
    ($body:expr, $($other_bodies:expr),*) => {
        {
        const G: f32 = 5.0; // Gravitational constant
        let bodies = vec![$($other_bodies),*];

        // Iterate over each body to calculate gravitational forces
        for other_bd in bodies {
            let g_force = $body.universal_gravitation_force(other_bd, G);
            // total_force += g_force; // Sum the forces
            $body.add_force(g_force); // Add total forces to the body
        }

        $body.law_of_momentum(); // Update acceleration based on total forces
        }
    };
}

#[macro_export]
macro_rules! update_astral_body_physics {
    ($body:expr, $delta_time:expr) => {{
        // Call the methods to update the body
        $body.update_geometry($delta_time);
        $body.update_velocity($delta_time);
    }};
}

pub struct AstralBody {
    pub mesh: MeshObject,
    pub velocity: Vector,
    pub acc: Vector,
    pub mass: f32,
    pub r: f32,
    pub forces: Vec<Force>
}
impl PartialEq for AstralBody {
    fn eq(&self, other: &Self) -> bool {
        self.mass == other.mass && 
        self.velocity == other.velocity && 
        self.acc == other.acc
    }
}
impl AstralBody {
    pub fn update_velocity(&mut self, delta_time: f32) {
        // Update velocity using the formula: v = v0 + a * t
        self.velocity.0 += self.acc.0 * delta_time;
        self.velocity.1 += self.acc.1 * delta_time;
        self.velocity.2 += self.acc.2 * delta_time;   
    }
    pub fn update_geometry(&mut self, delta_time: f32) {
        self.mesh.translate(self.velocity.0*delta_time, self.velocity.1*delta_time, self.velocity.2*delta_time);
    }
    pub fn universal_gravitation_force(&self, body: &AstralBody, g: f32) -> Force {
        let direction = position_euclidean(&body.mesh) - position_euclidean(&self.mesh);
        let distance_squared = direction.magnitude().powi(2);

        let mu: f32 = g * body.mass * self.mass.clone();
        if direction.magnitude() == 0.0 {
            return Force { direction: Vector(0.0, 0.0, 0.0), magnitude: 0.}; // Avoid division by zero
        }
        return Force{direction: direction.normalized(), magnitude: mu / distance_squared}
    }
    pub fn position(&self) -> Vector {
        return position_euclidean(&self.mesh)
    }
    pub fn add_force(&mut self, force: Force) {
        self.forces.push(force);
    }
    pub fn law_of_momentum(&mut self) {
        let mut resultant_force = Vector(0.,0.,0.);
        for force in &self.forces {
            resultant_force += force.direction * force.magnitude
        }
        if self.mass != 0.0 { // Avoid division by zero
            self.acc = resultant_force / self.mass;
        } else {
            self.acc = Vector(0.0, 0.0, 0.0); // No acceleration if mass is zero
        }
        self.forces.clear(); // Reset forces to only apply new forces in the next cycle
    }
}
impl SphereConstructor {
    pub fn sphere_physics_object(&self, velocity: Vector, mass: f32, screen: &Display<WindowSurface>, shader_data: ShaderData) -> AstralBody{
        let (data, indices) = self.new();
        let mesh = MeshObject {
            data: Mesh::new(screen, &data.verts, shader_data),
            uniforms: MeshUniforms { transform: TransformMatrix::identity(), indices },};
        return AstralBody { mesh, velocity, acc: Vector(0.,0.,0.,), mass, r: self.radius, forces: Vec::new()};
    }
}


// 
// SOME FUNCTIONS FOR THE UI
// 
impl AstralBody {
    pub fn get_widget(&self, name: String)-> AstralBodyInfoWidget {
        return AstralBodyInfoWidget {name, mass: self.mass, position: self.position_str(), velocity: self.velocity_str() }
    }
    
    // Method to get a formatted string of position
    pub fn position_str(&self) -> String {
        format!("({}, {}, {})", self.position().0, self.position().1, self.position().2)
    }

    // Method to get a formatted string of velocity
    pub fn velocity_str(&self) -> String {
        format!("({}, {}, {})", self.velocity.0, self.velocity.1, self.velocity.2)
    }
}