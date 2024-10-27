use glium::{glutin::surface::WindowSurface, Display};
use sphere::SphereConstructor;

use crate::{mesh::*, utils::matrix::TransformMatrix};

use super::{position_euclidean, Vector};

pub struct AstralBody {
    pub mesh: MeshObject,
    pub velocity: Vector,
    pub acc: Vector,
    pub mass: f32,
    pub r: f32,
}
impl PartialEq for AstralBody {
    fn eq(&self, other: &Self) -> bool {
        // Compare fields that define equality for your struct
        self.mass == other.mass && 
        self.velocity == other.velocity && 
        self.acc == other.acc
        // Add more fields as necessary
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
    pub fn universal_gravitation_force(&self, body: &mut AstralBody, g: f32) -> Vector {
        let direction = position_euclidean(&body.mesh) - position_euclidean(&self.mesh);
        let mu: f32 = g * body.mass * self.mass.clone();
        let distance_squared = direction.magnitude().powi(2);
        if direction.magnitude() == 0.0 {
            return Vector(0.0, 0.0, 0.0); // Avoid division by zero
        }
        return ((direction.normalized() * mu) /distance_squared) * 1.
    }
}
impl SphereConstructor {
    pub fn sphere_physics_object(&self, velocity: Vector, mass: f32, screen: &Display<WindowSurface>, shader_data: ShaderData) -> AstralBody{
        let (data, indices) = self.new();
        let mesh = MeshObject {
            data: Mesh::new(screen, &data.verts, shader_data),
            uniforms: MeshUniforms { transform: TransformMatrix::identity(), indices },};
        return AstralBody { mesh, velocity, acc: Vector(0.,0.,0.,), mass, r: self.radius};
    }
}