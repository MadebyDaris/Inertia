use glium::{
    glutin::surface::WindowSurface,
    texture::{Texture2d, UncompressedFloatFormat, MipmapsOption},
    uniforms::{MagnifySamplerFilter, MinifySamplerFilter, SamplerBehavior, SamplerWrapFunction},
    Display, Surface, Frame,
};
use glium::framebuffer::SimpleFrameBuffer;

use super::wave_function::GaussianWavePacket;

/// Represents a 2D quantum simulation grid using GPU textures
/// Wavefunction texture R=real part, G=imaginary part using GL_RG32F format for high precision
pub struct QuantumGrid {
    pub size: (u32, u32),
    pub grid_extent: (f32, f32),
    pub wave_packet: GaussianWavePacket,
    pub wave_packet_mem: GaussianWavePacket,


    /// Using GL_R32F format - single channel float
    pub potential: Texture2d,
    pub probability_density: Texture2d,
}

pub fn create_map(display: &Display<WindowSurface>, width: u32, height: u32) -> Result<Texture2d, glium::texture::TextureCreationError> {
        // Create single-channel float texture initialized to zero
        let data: Vec<f32> = vec![0.0; (width * height) as usize];
        let image = glium::texture::RawImage2d {
            data: std::borrow::Cow::Owned(data),
            width,
            height,
            format: glium::texture::ClientFormat::F32,
        };
        let texture = Texture2d::with_format(
            display,
            image,
            UncompressedFloatFormat::F32,
            MipmapsOption::NoMipmap,
        )?;
        
        // Set wrap mode to ClampToEdge to prevent wrapping at boundaries
        texture.sampled()
            .wrap_function(SamplerWrapFunction::Clamp);
        
        Ok(texture)
}

impl QuantumGrid {
    pub fn new(display: &Display<WindowSurface>, width: u32, height: u32) -> Self {
        let grid_extent = (10.0, 10.0); // Larger extent: -5 to +5 in both directions for better visualization
        
        let wave_packet_mem =  GaussianWavePacket::new(
                display,
                0.0,
                0.0,
                0.1,
                0.0,
                0.0,
                (width, height),
                grid_extent,
            );
        
        let potential = create_map(display, width, height).unwrap();
        
        let probability_density = create_map(display, width, height).unwrap();
        
        Self {
            size: (width, height),
            grid_extent,
            wave_packet: GaussianWavePacket::new(
                display,
                0.0,
                0.0,
                0.1,
                0.0,
                0.0,
                (width, height),
                grid_extent,
            ),
            potential,
            wave_packet_mem,
            probability_density,
        }
    }
    
    /// Initialize the potential with a double-slit barrier
    pub fn init_double_slit_potential(
        &mut self,
        display: &Display<WindowSurface>,
        barrier_x: f32,
        barrier_thickness: f32,
        slit_width: f32,
        slit_separation: f32,
        barrier_height: f32,
    ) {
        let (width, height) = self.size;
        let mut data: Vec<f32> = Vec::with_capacity((width * height) as usize);
        
        let half_extent_x = self.grid_extent.0 / 2.0;
        let half_extent_y = self.grid_extent.1 / 2.0;
        
        for y in 0..height {
            for x in 0..width {
                // Map to grid extent (e.g., -5 to +5)
                let nx = (x as f32 / width as f32) * self.grid_extent.0 - half_extent_x;
                let ny = (y as f32 / height as f32) * self.grid_extent.1 - half_extent_y;
                
                // Use Gaussian profile for smoother barrier (prevents wave trapping)
                // V(x) = height * exp(-(x - barrier_x)² / width²)
                let x_distance = nx - barrier_x;
                let gaussian_x = barrier_height * (-((x_distance / barrier_thickness).powi(2))).exp();
                
                let mut potential_value = gaussian_x;
                
                // Cut out slits if specified
                if slit_width > 0.0 && slit_separation > 0.0 {
                    let upper_slit_center = slit_separation / 2.0;
                    let lower_slit_center = -slit_separation / 2.0;
                    
                    let dist_to_upper = (ny - upper_slit_center).abs();
                    let dist_to_lower = (ny - lower_slit_center).abs();
                    
                    // If in either slit, reduce potential (Gaussian slit profile)
                    if dist_to_upper < slit_width || dist_to_lower < slit_width {
                        let slit_factor = if dist_to_upper < dist_to_lower {
                            (-((dist_to_upper / (slit_width * 0.5)).powi(2))).exp()
                        } else {
                            (-((dist_to_lower / (slit_width * 0.5)).powi(2))).exp()
                        };
                        // Reduce potential in slit region
                        potential_value *= 1.0 - slit_factor;
                    }
                }
                
                data.push(potential_value);
            }
        }
        
        // Upload to GPU
        let image = glium::texture::RawImage2d {
            data: std::borrow::Cow::Owned(data),
            width,
            height,
            format: glium::texture::ClientFormat::F32,
        };
        self.potential = Texture2d::with_format(
            display,
            image,
            UncompressedFloatFormat::F32,
            MipmapsOption::NoMipmap,
        ).unwrap();
    }
    
    /// Get physical grid spacing (dx, dy)
    pub fn get_spacing(&self) -> (f32, f32) {
        let dx = self.grid_extent.0 / self.size.0 as f32;
        let dy = self.grid_extent.1 / self.size.1 as f32;
        (dx, dy)
    }
    
    /// Get sampler behavior for texture access
    pub fn sampler_behavior() -> SamplerBehavior {
        SamplerBehavior {
            magnify_filter: MagnifySamplerFilter::Linear,
            minify_filter: MinifySamplerFilter::Linear,
            ..Default::default()
        }
    }
    
    /// Swap wavefunction buffers (for ping-pong rendering)
    pub fn swap_wavefunctions(&mut self) {
        std::mem::swap(&mut self.wave_packet_mem, &mut self.wave_packet);
    }
}

/// Builder pattern for creating quantum grids with different initial conditions
pub struct QuantumGridBuilder {
    width: u32,
    height: u32,
}

impl QuantumGridBuilder {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    
    pub fn build(self, display: &Display<WindowSurface>) -> QuantumGrid {
        QuantumGrid::new(display, self.width, self.height)
    }
}
