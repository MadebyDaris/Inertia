use glium::{glutin::surface::WindowSurface, Display, Frame, Surface, IndexBuffer, index::PrimitiveType, uniform};
use crate::render::shader_system::{ShaderManager, ShaderType};
use crate::mesh::mesh_object::{Vertex, Mesh, ShaderData};
use crate::utils::matrix::TransformMatrix;

pub struct AxisLines {
    mesh: Mesh,
    transform: TransformMatrix,
}

impl AxisLines {
    pub fn new(display: &Display<WindowSurface>, shader_manager: &ShaderManager, length: f32) -> Self {
        let vertices = vec![
            // Origin
            Vertex { position: [0.0, 0.0, 0.0], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 0.0] },
            
            // Positive axes
            Vertex { position: [length, 0.0, 0.0], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 0.0] },
            Vertex { position: [0.0, length, 0.0], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 0.0] },
            Vertex { position: [0.0, 0.0, length], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 0.0] },
            
            // Negative axes
            Vertex { position: [-length, 0.0, 0.0], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 0.0] },
            Vertex { position: [0.0, -length, 0.0], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 0.0] },
            Vertex { position: [0.0, 0.0, -length], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 0.0] },
        ];

        let shader_data = ShaderData {
            tex_filename: "./data/tex/grid.png".to_string(),
            shader_type: ShaderType::Unlit,
            material: Default::default(),
        };

        let mesh = Mesh::new(display, &vertices, shader_data, shader_manager);
        let transform = TransformMatrix::identity();

        Self {
            mesh,
            transform,
        }
    }

    pub fn render(&self, display: &Display<WindowSurface>, target: &mut Frame, view: [[f32; 4]; 4], perspective: [[f32; 4]; 4]) {
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            blend: glium::Blend::alpha_blending(),
            .. Default::default()
        };

        let colors = [
            [1.0f32, 0.0f32, 0.0f32],  // Red (X+)
            [0.0f32, 1.0f32, 0.0f32],  // Green (Y+)
            [0.0f32, 0.0f32, 1.0f32],  // Blue (Z+)
            [0.5f32, 0.0f32, 0.0f32],  // Dark Red (X-)
            [0.0f32, 0.5f32, 0.0f32],  // Dark Green (Y-)
            [0.0f32, 0.0f32, 0.5f32],  // Dark Blue (Z-)
        ];

        // Draw each axis line with its respective color
        for i in 0..6 {
            let line_indices = IndexBuffer::new(display, PrimitiveType::LinesList, &[0u16, (i + 1) as u16]).unwrap();
            
            let uniforms = uniform! {
                model: self.transform.matrix,
                view: view,
                perspective: perspective,
                tex: &self.mesh.texture,
                material_color: colors[i],
                emission_strength: 0.0_f32,
                roughness: 0.0_f32,
                metallic: 0.0_f32,
                specular_intensity: 0.0_f32,
                shininess: 1.0_f32,
                ambient_occlusion: 1.0_f32,
                u_light_direction: [0.5_f32, -0.5_f32, 0.5_f32],
                u_light_color: [1.0_f32, 1.0_f32, 1.0_f32],
                camera_position: [0.0_f32, 50.0_f32, 80.0_f32],
            };

            target.draw(&self.mesh.vert_buffer, &line_indices, &*self.mesh.program, &uniforms, &params).unwrap();
        }
    }
}
