// src/mesh/grid_floor.rs
use glium::{glutin::surface::WindowSurface, Display};
use crate::{
    mesh::{MeshObject, ShaderData, Vertex},
};

pub struct GridFloor {
    pub size: f32,
    pub divisions: usize,
    pub color: [f32; 3],
}

impl GridFloor {
    pub fn new(size: f32, divisions: usize, color: [f32; 3]) -> Self {
        Self { size, divisions, color }
    }

    pub fn create_mesh(&self, display: &Display<WindowSurface>, shader_data: ShaderData) -> MeshObject {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let step = self.size / self.divisions as f32;
        let half_size = self.size / 2.0;

        // Create a simple flat plane for the grid
        let y = -10.0; // Place well below other objects
        
        // Just create 4 vertices for a simple plane
        vertices.push(Vertex {
            position: [-half_size, y, -half_size],
            normal: [0.0, 1.0, 0.0],
            tex_coords: [0.0, 0.0],
        });
        vertices.push(Vertex {
            position: [half_size, y, -half_size],
            normal: [0.0, 1.0, 0.0],
            tex_coords: [1.0, 0.0],
        });
        vertices.push(Vertex {
            position: [half_size, y, half_size],
            normal: [0.0, 1.0, 0.0],
            tex_coords: [1.0, 1.0],
        });
        vertices.push(Vertex {
            position: [-half_size, y, half_size],
            normal: [0.0, 1.0, 0.0],
            tex_coords: [0.0, 1.0],
        });

        // Two triangles to form a plane
        indices.extend_from_slice(&[
            0, 1, 2,  // First triangle
            0, 2, 3   // Second triangle
        ]);

        MeshObject::new(display, &vertices, indices, shader_data)
    }
}