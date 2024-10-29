use crate::{mesh::MeshObject, utils::vector::Vector};

pub mod physicsobject;
pub mod physicsworld;

pub fn position_euclidean(body: &MeshObject) -> Vector {
    let matrix: [[f32; 4]; 4] = body.uniforms.transform.matrix;
    return Vector(matrix[3][0],matrix[3][1],matrix[3][2]);
}