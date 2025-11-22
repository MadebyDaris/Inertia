use glium::{glutin::surface::WindowSurface, Display, Frame};
use crate::mesh::quantum_mesh::QuantumMesh;
use crate::utils::matrix::TransformMatrix;

/// High-level object for rendering quantum wavefunctions
/// Combines the mesh with transform and rendering parameters
pub struct QuantumObject {
    /// The high-resolution mesh for visualization
    pub mesh: QuantumMesh,
    
    /// Transform matrix for positioning/scaling the visualization
    pub transform: TransformMatrix,
    
    /// Height scale factor for vertical displacement
    /// Higher values = taller "mountains" in the wavefunction
    pub height_scale: f32,
    
    /// Whether to render this object
    pub visible: bool,
}

impl QuantumObject {
    /// Create a new quantum object with the specified grid resolution
    /// resolution: Should match your simulation texture size (e.g., 256x256)
    pub fn new(display: &Display<WindowSurface>, resolution: (u32, u32)) -> Self {
        let mesh = QuantumMesh::new(display, resolution);
        
        Self {
            mesh,
            transform: TransformMatrix::identity(),
            height_scale: 1.0,
            visible: true,
        }
    }
    
    /// Render the quantum wavefunction visualization
    pub fn render(
        &self,
        display: &Display<WindowSurface>,
        target: &mut Frame,
        psi_texture: &glium::texture::Texture2d,
        view: [[f32; 4]; 4],
        projection: [[f32; 4]; 4],
    ) {
        if !self.visible {
            return;
        }
        
        self.mesh.render(
            display,
            target,
            psi_texture,
            self.height_scale,
            self.transform.matrix,
            view,
            projection,
        );
    }
    
    pub fn set_height_scale(&mut self, scale: f32) {
        self.height_scale = scale;
    }
    
    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.transform = TransformMatrix::identity().translate(x, y, z);
    }
    
    pub fn set_scale(&mut self, sx: f32, sy: f32, sz: f32) {
        self.transform = self.transform.scale(sx, sy, sz);
    }
    
    pub fn translate(&mut self, dx: f32, dy: f32, dz: f32) {
        self.transform = self.transform.translate(dx, dy, dz);
    }
    
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
    
    pub fn get_stats(&self) -> QuantumObjectStats {
        QuantumObjectStats {
            resolution: self.mesh.resolution,
            vertex_count: self.mesh.vertex_count(),
            triangle_count: self.mesh.triangle_count(),
            height_scale: self.height_scale,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct QuantumObjectStats {
    pub resolution: (u32, u32),
    pub vertex_count: usize,
    pub triangle_count: usize,
    pub height_scale: f32,
}

impl std::fmt::Display for QuantumObjectStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Quantum Mesh: {}x{} | Vertices: {} | Triangles: {} | Height Scale: {:.2}",
            self.resolution.0,
            self.resolution.1,
            self.vertex_count,
            self.triangle_count,
            self.height_scale
        )
    }
}
