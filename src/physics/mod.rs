use std::{default, ops::{Add, AddAssign}};

use crate::{mesh::MeshObject, utils::vector::Vector};

pub mod physicsobject;
pub mod physicsworld;
pub mod physicsinterface;

pub fn position_euclidean(body: &MeshObject) -> Vector {
    let matrix: [[f32; 4]; 4] = body.uniforms.transform.matrix;
    return Vector(matrix[3][0],matrix[3][1],matrix[3][2]);
}

#[derive(Clone, Copy, Debug)]
pub struct Force {
    pub direction: Vector,
    pub magnitude: f32
}
pub struct EulerAngles {
    pub pitch: f32, // Rotation around the X-axis
    pub yaw: f32,   // Rotation around the Y-axis
    pub roll: f32,  // Rotation around the Z-axis
}

impl Default for Force {
    fn default() -> Self {
        Self { direction: Vector(0.,0.,0.), magnitude: 0. }
    }
}
impl Add for Force {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        // Vector addition of directions
        let resultant_direction = self.direction * self.magnitude + other.direction * other.magnitude ;
        let resultant_magnitude = resultant_direction.magnitude(); // Calculate magnitude of the resultant vector

        Self {
            direction: resultant_direction.normalized(),
            magnitude: resultant_magnitude,
        }
    }
}
impl AddAssign for Force {
    fn add_assign(&mut self, rhs: Self) {
        // Vector addition of directions
        let resultant_direction = self.direction * self.magnitude + rhs.direction * rhs.magnitude ;
        let resultant_magnitude = resultant_direction.magnitude();

        self.direction = resultant_direction.normalized();
        self.magnitude = resultant_magnitude;
    }
}
impl std::iter::Sum for Force {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |acc, force| acc + force)
    }
}