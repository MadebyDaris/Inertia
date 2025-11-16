use glium::{glutin::surface::WindowSurface, Display};

use crate::utils::matrix::TransformMatrix;
use crate::render::shader_system::ShaderManager;

use super::{Mesh, MeshData, MeshObject, ShaderData, Vertex};

pub struct SphereConstructor {
    pub radius: f32,
    pub latitude: usize,
    pub longitude: usize,
}

impl SphereConstructor {
    pub fn new(&self) -> (MeshData, Vec<u32>) {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices : Vec<u32>= Vec::new();
        
        // using the coordinates in a spherical referential we can create a sphere of a specific radius 
        for lat in 0..self.latitude+1 {
            let theta = (lat as f32)*(std::f32::consts::PI) / self.latitude as f32;
            let sin_theta = theta.sin();
            let cos_theta = theta.cos();
            for lon in 0..self.longitude+1 {
                let phi = lon as f32 * 2.0 * std::f32::consts::PI / self.longitude as f32;
                let sin_phi = phi.sin();
                let cos_phi = phi.cos();

                let x = self.radius * sin_theta * cos_phi;
                let y = self.radius * cos_theta;
                let z = self.radius * sin_theta * sin_phi;

                let u = lon as f32 / self.longitude as f32;
                let v = lat as f32 / self.latitude as f32;

                let normal = [x/self.radius, y/self.radius, z/self.radius]; // Normal is the same as the position for a sphere

                
                vertices.push(Vertex { 
                    position: [x, y, z], 
                    tex_coords: [u, v],
                    normal 
                });
            }
        }
        for lat in 0..self.latitude+1 {
            for lon in 0..self.longitude+1 {
                let current = (lat * (self.longitude + 1) + lon) as u32;
                let next = current + (self.longitude + 1) as u32;

                if lat !=0 {
                    indices.push(current);
                    indices.push(next);
                    indices.push(current + 1);
                }
                if lat != (self.latitude - 1) {
                    indices.push(current + 1);
                    indices.push(next);
                    indices.push(next + 1);
                }
            }
        }
        return (MeshData { verts: vertices}, indices);
    }

    pub fn sphere_object(&self, screen: &Display<WindowSurface>, shader_data: ShaderData, shader_manager: &ShaderManager) -> MeshObject{
        let (data, indices) = self.new();
        return MeshObject {
            data: Mesh::new(screen, &data.verts, shader_data.clone(), shader_manager),
            uniforms: super::MeshUniforms { 
                transform: TransformMatrix::identity(), 
                indices,
                material: shader_data.material,
                shader_type: shader_data.shader_type,
            },
        }
    }
}