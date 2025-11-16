use crate::utils::vector::Vector;

#[derive(Clone, Debug)]
pub struct VectorArrow {
    pub offset: Vector,
    pub vector: Vector,
    pub color: [f32; 3],
    pub thickness: f32,
    pub scale: f32,
    pub enabled: bool,
}

impl VectorArrow {
    pub fn velocity() -> Self {
        Self {
            offset: Vector(0.0, 0.0, 0.0),
            vector: Vector(0.0, 0.0, 0.0),
            color: [1.0, 0.0, 0.0],
            thickness: 2.0,
            scale: 0.01,
            enabled: false,
        }
    }

    pub fn acceleration() -> Self {
        Self {
            offset: Vector(0.0, 0.0, 0.0),
            vector: Vector(0.0, 0.0, 0.0),
            color: [0.0, 1.0, 0.0],
            thickness: 2.0,
            scale: 1.0,
            enabled: false,
        }
    }

    pub fn force() -> Self {
        Self {
            offset: Vector(0.0, 0.0, 0.0),
            vector: Vector(0.0, 0.0, 0.0),
            color: [0.0, 0.5, 1.0],
            thickness: 2.0,
            scale: 0.5,
            enabled: false,
        }
    }

    pub fn custom(color: [f32; 3]) -> Self {
        Self {
            offset: Vector(0.0, 0.0, 0.0),
            vector: Vector(0.0, 0.0, 0.0),
            color,
            thickness: 1.5,
            scale: 1.0,
            enabled: true,
        }
    }
}
