use crate::{mesh::MeshObject, physics::{physicsobject::PhysicsObject, position_euclidean, EulerAngles, Force}, simulation::orbital_simulation::AstralCollisionObject, utils::vector::Vector};

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
pub struct Rigidbody {
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
use crate::render::ray::Ray;

impl PhysicsObject for Rigidbody {
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

    fn mesh(&self) -> &Self::Mesh {
        &self.mesh
    }

    fn position(&self) -> Vector {
        position_euclidean(&self.mesh)
    }

    fn mass(&self) -> f32 {
        self.mass
    }

    fn moment_of_inertia(&self) -> f32 {
        self.moment_of_inertia
    }

    fn velocity(&self) -> Vector {
        self.velocity
    }

    fn velocity_mut(&mut self) -> &mut Vector {
        &mut self.velocity
    }

    fn acceleration(&self) -> Vector {
        self.acceleration
    }

    fn acceleration_mut(&mut self) -> &mut Vector {
        &mut self.acceleration
    }

    fn angular_velocity(&self) -> Vector {
        self.angular_velocity
    }

    fn angular_velocity_mut(&mut self) -> &mut Vector {
        &mut self.angular_velocity
    }

    fn angular_acceleration(&self) -> Vector {
        self.angular_acceleration
    }

    fn angular_acceleration_mut(&mut self) -> &mut Vector {
        &mut self.angular_acceleration
    }

    fn euler_angles(&self) -> &EulerAngles {
        &self.euler_angles
    }

    fn euler_angles_mut(&mut self) -> &mut EulerAngles {
        &mut self.euler_angles
    }

    fn forces(&self) -> &Vec<Force> {
        &self.forces
    }

    fn forces_mut(&mut self) -> &mut Vec<Force> {
        &mut self.forces
    }

    fn torques(&self) -> &Vec<Vector> {
        &self.torques
    }

    fn torques_mut(&mut self) -> &mut Vec<Vector> {
        &mut self.torques
    }

    fn mesh_mut(&mut self) -> &mut Self::Mesh {
        &mut self.mesh
    }

    fn intersects(&self, _ray: &Ray) -> Option<f32> {
        None
    }
}

impl Rigidbody {
    pub fn update_geometry(&mut self, delta_time: f32) {
        self.mesh.translate(self.velocity.0 * delta_time, self.velocity.1 * delta_time, self.velocity.2 * delta_time);
        self.mesh.rotate(self.euler_angles.pitch, self.euler_angles.yaw, self.euler_angles.roll);
    }
}