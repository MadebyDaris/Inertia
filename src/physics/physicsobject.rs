use glium::{glutin::surface::WindowSurface, Display};
use sphere::SphereConstructor;

use crate::physics::Force;

use crate::render::ray::Ray;
use crate::{mesh::*, utils::{matrix::TransformMatrix}};
use crate::ui::*;
use super::{position_euclidean, EulerAngles, Vector};

#[macro_export]
macro_rules! calculate_g_forces {
    ($body:expr, $($other_bodies:expr),*) => {
        {
        use crate::physics::position_euclidean;
        const G: f32 = 5.0; // Gravitational constant
        let bodies = vec![$($other_bodies),*];

        // Iterate over each body to calculate gravitational forces
        for other_bd in bodies {
            let g_force = $body.universal_gravitation_force(other_bd, G);
            $body.add_force(g_force); // Add total forces to the body
            let torque = $body.calculate_torque(g_force, position_euclidean(&other_bd.mesh));
            $body.torques.push(torque)
        }

        $body.law_of_momentum(); // Update acceleration based on total forces
        }
    };
}

#[macro_export]
macro_rules! update_body_physics {
    ($body:expr, $delta_time:expr) => {{
        // Call the methods to update the body
        $body.update_velocity($delta_time);
        $body.update_geometry($delta_time);
        $body.update_orientation($delta_time);
    }};
}

pub trait PhysicsObject {
    type Mesh: Sized;
    type Velocity: Sized;
    type Acceleration: Sized;
    type AngularVelocity: Sized;
    type AngularAcceleration: Sized;
    type Mass: Sized;
    type Forces: Sized;
    type Torques: Sized;
    type MomentOfInertia: Sized;
    type EulerAngles: Sized;
    type CollisionObject;

    // Required methods
    fn mesh(&self) -> &Self::Mesh;
    fn position(&self) -> Vector;
    fn mass(&self) -> f32;
    fn moment_of_inertia(&self) -> f32;
    fn velocity(&self) -> Vector;
    fn velocity_mut(&mut self) -> &mut Vector;
    fn acceleration(&self) -> Vector;
    fn acceleration_mut(&mut self) -> &mut Vector;
    fn angular_velocity(&self) -> Vector;
    fn angular_velocity_mut(&mut self) -> &mut Vector;
    fn angular_acceleration(&self) -> Vector;
    fn angular_acceleration_mut(&mut self) -> &mut Vector;
    fn euler_angles(&self) -> &EulerAngles;
    fn euler_angles_mut(&mut self) -> &mut EulerAngles;
    fn forces(&self) -> &Vec<Force>;
    fn forces_mut(&mut self) -> &mut Vec<Force>;
    fn torques(&self) -> &Vec<Vector>;
    fn torques_mut(&mut self) -> &mut Vec<Vector>;
    fn mesh_mut(&mut self) -> &mut Self::Mesh;


    // Provided methods
    fn update_velocity(&mut self, delta_time: f32) where Self: Sized {
        *self.velocity_mut() += self.acceleration() * delta_time;
        *self.angular_velocity_mut() += self.angular_acceleration() * delta_time;
    }

    fn update_geometry(&mut self, delta_time: f32) where Self: Sized {
        self.mesh_mut().translate(self.velocity().0 * delta_time, self.velocity().1 * delta_time, self.velocity().2 * delta_time);
        self.mesh_mut().rotate(self.euler_angles().pitch, self.euler_angles().yaw, self.euler_angles().roll);
    }

    fn law_of_momentum(&mut self) where Self: Sized {
        let mut resultant_force = Vector(0., 0., 0.);
        for force in self.forces() {
            resultant_force += force.direction * force.magnitude
        }
        if self.mass() != 0.0 {
            *self.acceleration_mut() = resultant_force / self.mass();
        } else {
            *self.acceleration_mut() = Vector(0.0, 0.0, 0.0);
        }
        let total_torque = self.total_torque();
        *self.angular_acceleration_mut() = total_torque / self.moment_of_inertia();

        self.torques_mut().clear();
        self.forces_mut().clear();
    }

    fn total_torque(&self) -> Vector where Self: Sized {
        self.torques().iter().fold(Vector(0., 0., 0.), |acc, t| acc + *t)
    }

    fn calculate_torque(&mut self, force: Force, point: Vector) -> Vector where Self: Sized {
        let direction = self.position() - point;
        Vector::cross(direction, force.direction * force.magnitude)
    }

    fn update_angular_acceleration(&mut self, torque: Vector, moment_of_inertia: f32) where Self: Sized {
        *self.angular_acceleration_mut() = torque / moment_of_inertia;
    }

    fn update_orientation(&mut self, delta_time: f32) where Self: Sized {
        self.euler_angles_mut().pitch += self.angular_velocity().0 * delta_time;
        self.euler_angles_mut().yaw += self.angular_velocity().1 * delta_time;
        self.euler_angles_mut().roll += self.angular_velocity().2 * delta_time;

        self.euler_angles_mut().pitch %= std::f32::consts::TAU;
        self.euler_angles_mut().yaw %= std::f32::consts::TAU;
        self.euler_angles_mut().roll %= std::f32::consts::TAU;
    }

    fn intersects(&self, ray: &Ray) -> Option<f32>;
}
    