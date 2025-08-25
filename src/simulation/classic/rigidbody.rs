use crate::{mesh::MeshObject, physics::{physicsobject::PhysicsObject, EulerAngles, Force}, simulation::orbital_simulation::AstralCollisionObject, utils::vector::Vector};

pub struct ClassicCollisionObject {

}

pub type RigidBodyObject = dyn PhysicsObject<
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
    CollisionObject = AstralCollisionObject,
>;
pub struct AstralBody {
    pub mesh: MeshObject,
    pub velocity: Vector,
    pub acceleration: Vector,
    pub angular_velocity: Vector,
    pub angular_acceleration: Vector,
    pub mass: f32,
    pub forces: Vec<Force>,
    pub torques: Vec<Vector>,
    pub moment_of_inertia: f32,
    pub euler_angles: EulerAngles
}
impl PhysicsObject for AstralBody {
    type Mesh = MeshObject;
    type Velocity = Vector;
    type Acceleration = Vector;
    type AngularVelocity = Vector;
    type AngularAcceleration = Vector;
    type Mass = f32;
    type Forces = Vec<Force>;
    type Torques = Vec<Vector>;
    type MomentOfInertia = f32;
    type EulerAngles = EulerAngles;
    type CollisionObject = AstralCollisionObject;
    
    fn total_torque(&self) -> Vector {
        todo!()
    }
    
    fn calculate_torque(&mut self, force: Force, point: Vector) -> Vector {
        todo!()
    }
    
    fn update_angular_acceleration(&mut self, torque: Vector, moment_of_inertia: f32) {
        todo!()
    }
    
    fn mesh(&self) -> &Self::Mesh {
        todo!()
    }
    
    fn position(&self) -> Vector {
        todo!()
    }
    
    fn intersects(&self, ray: &crate::render::ray::Ray) -> Option<f32> {
        todo!()
    }
    
    fn update_velocity(&mut self, delta_time: f32) {
    }
    
    fn update_geometry(&mut self, delta_time: f32) {
    }
    
    fn update_orientation(&mut self, delta_time: f32) {
    }
    
    fn law_of_momentum(&mut self) {
    }

}