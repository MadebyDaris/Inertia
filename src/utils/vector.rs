
use std::ops::{Add, AddAssign, Div, Mul, Sub};
#[derive(Clone, Copy, Debug)]
pub struct Vector (pub f32, pub f32, pub f32);

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}
impl Div for Vector {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vector(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}
impl Div<f32> for Vector {
    type Output = Self;
    
    fn div(self, rhs: f32) -> Self::Output {
        Vector(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}
impl Mul<f32> for Vector {
    type Output = Self;
    
    fn mul(self, rhs: f32) -> Self::Output {
        Vector(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}
impl AddAssign for Vector {    
    fn add_assign(&mut self, rhs: Self){
        *self = Vector(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl Add for Vector {    
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vector(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        // Compare each component of the vector for equality
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}
impl Vector {
    pub fn normalized(&self) -> Vector {
        let magnitude = self.magnitude();

        // Avoid division by zero
        if magnitude == 0.0 {
            return Vector(0.0, 0.0, 0.0);
        }

        // Return a new Vector with normalized components
        Vector(
            self.0 / magnitude,
            self.1 / magnitude,
            self.2 / magnitude,
        )
    }
    pub fn to_scalar(self) -> f32 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()

    }
    
    pub fn cross(a: Vector, b: Vector) -> Vector {
        return Vector(a.1*b.2 - a.2*b.1, a.2*b.0 - a.0*b.2, a.0*b.1 - a.1*b.0)}
    pub fn magnitude(&self) -> f32 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }
    pub fn rotate_x(&self, angle: f32) -> Vector {
        let cos_theta = angle.cos();
        let sin_theta = angle.sin();

        Vector(
            self.0,
            self.1 * cos_theta - self.2 * sin_theta,
            self.1 * sin_theta + self.2 * cos_theta,
        )
    }
    pub fn dot(self, other: Vector) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
    pub fn rotate_y(&self, angle: f32) -> Vector {
        let cos_theta = angle.cos();
        let sin_theta = angle.sin();

        Vector(
            self.0 * cos_theta + self.2 * sin_theta,
            self.1,
            -self.0 * sin_theta + self.2 * cos_theta,
        )
    }

    pub fn rotate_z(&self, angle: f32) -> Vector {
        let cos_theta = angle.cos();
        let sin_theta = angle.sin();

        Vector(
            self.0 * cos_theta - self.1 * sin_theta,
            self.0 * sin_theta + self.1 * cos_theta,
            self.2,
        )
    }
}
