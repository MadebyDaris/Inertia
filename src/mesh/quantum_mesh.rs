use glium::{
    glutin::surface::WindowSurface,
    Display, Frame, Surface, VertexBuffer, IndexBuffer,
    index::PrimitiveType, uniform, Program,
};
use std::rc::Rc;
use std::fs;

#[derive(Copy, Clone)]
pub struct QuantumVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

glium::implement_vertex!(QuantumVertex, position, tex_coords);

/// grid mesh for visualizing quantum wavefunction
pub struct QuantumMesh {
    pub vertices: VertexBuffer<QuantumVertex>,
    pub indices: IndexBuffer<u32>,
    pub resolution: (u32, u32),
    pub vertex_shader: Rc<Program>,
}

impl QuantumMesh {
    pub fn new(display: &Display<WindowSurface>, resolution: (u32, u32)) -> Self {
        let (width, height) = resolution;
        
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        
        // Each vertex maps to a texel in the simulation texture
        for y in 0..height {
            for x in 0..width {
                // Normalize coordinates to [-1, 1] for world space
                let nx = (x as f32 / (width - 1) as f32) * 2.0 - 1.0;
                let nz = (y as f32 / (height - 1) as f32) * 2.0 - 1.0;
                
                let u = x as f32 / (width - 1) as f32;
                let v = y as f32 / (height - 1) as f32;
                
                vertices.push(QuantumVertex {
                    position: [nx, 0.0, nz],
                    tex_coords: [u, v],
                });
            }
        }
        
        // Generate indices for triangles
        for y in 0..(height - 1) {
            for x in 0..(width - 1) {
                let top_left = y * width + x;
                let top_right = top_left + 1;
                let bottom_left = (y + 1) * width + x;
                let bottom_right = bottom_left + 1;
                
                // First triangle (top-left, bottom-left, top-right)
                indices.push(top_left);
                indices.push(bottom_left);
                indices.push(top_right);
                
                // Second triangle (top-right, bottom-left, bottom-right)
                indices.push(top_right);
                indices.push(bottom_left);
                indices.push(bottom_right);
            }
        }
        
        let vertex_buffer = VertexBuffer::new(display, &vertices).unwrap();
        let index_buffer = IndexBuffer::new(display, PrimitiveType::TrianglesList, &indices).unwrap();
        
        // Load shaders
        let vertex_shader = Self::load_shader_program(display)
            .expect("Failed to load quantum wave shaders");
        
        Self {
            vertices: vertex_buffer,
            indices: index_buffer,
            resolution,
            vertex_shader: Rc::new(vertex_shader),
        }
    }
    
    /// Load the wave visualization shader program
    fn load_shader_program(display: &Display<WindowSurface>) -> Result<Program, String> {
        let vertex_src = fs::read_to_string("data/glsl/wave_vertex_shdr.glsl")
            .map_err(|e| format!("Failed to read vertex shader: {}", e))?;
        
        let fragment_src = fs::read_to_string("data/glsl/wave_fragment_shdr.glsl")
            .map_err(|e| format!("Failed to read fragment shader: {}", e))?;
        
        Program::from_source(display, &vertex_src, &fragment_src, None)
            .map_err(|e| format!("Failed to compile shader: {}", e))
    }
    
    /// Render the quantum mesh with the wavefunction texture
    pub fn render(
        &self,
        display: &Display<WindowSurface>,
        target: &mut Frame,
        psi_texture: &glium::texture::Texture2d,
        height_scale: f32,
        model: [[f32; 4]; 4],
        view: [[f32; 4]; 4],
        projection: [[f32; 4]; 4],
    ) {
        let uniforms = uniform! {
            u_psi_texture: psi_texture,
            u_height_scale: height_scale,
            u_model: model,
            u_view: view,
            u_projection: projection,
        };
        
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            backface_culling: glium::BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };
        
        target.draw(
            &self.vertices,
            &self.indices,
            &*self.vertex_shader,
            &uniforms,
            &params,
        ).unwrap();
    }
    
    pub fn vertex_count(&self) -> usize {
        (self.resolution.0 * self.resolution.1) as usize
    }
    
    pub fn triangle_count(&self) -> usize {
        ((self.resolution.0 - 1) * (self.resolution.1 - 1) * 2) as usize
    }
}
