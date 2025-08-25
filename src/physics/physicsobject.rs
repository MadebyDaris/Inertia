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
    
    fn update_velocity(&mut self, delta_time: f32) {
    }

    fn update_geometry(&mut self, delta_time: f32) {
    }

    fn update_orientation(&mut self, delta_time: f32) {
    }

    fn law_of_momentum(&mut self) {
    }

    // Calculate total torque from the list of torques
    fn total_torque(&self) -> Vector;

    // Calculate torque based on applied force and point of application
    fn calculate_torque(&mut self, force: Force, point: Vector) -> Vector;

    // Update angular acceleration based on applied torque and moment of inertia
    fn update_angular_acceleration(&mut self, torque: Vector, moment_of_inertia: f32);
    // get function
    fn mesh(&self) -> &Self::Mesh;
    fn position(&self) -> Vector;
    fn intersects(&self, ray: &Ray) -> Option<f32>;
}
    