use crate::utils::vector::Vector;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Self {
        Ray { origin, direction: direction.normalized() }
    }

    pub fn at(&self, t: f32) -> Vector {
        self.origin + self.direction * t
    }
}
