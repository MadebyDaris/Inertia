use glium::{Display, Texture2d, glutin::surface::WindowSurface, texture::{MipmapsOption, UncompressedFloatFormat}, uniforms::SamplerWrapFunction};
pub use num::complex::Complex;

/// Takes a closure that generates (real, imag) for each grid point
pub fn create_wavefunction_texture<F>(
    display: &Display<WindowSurface>,
    size: (u32, u32),
    grid_extent: (f32, f32),
    mut generator: F,
) -> Texture2d
where
    F: FnMut(f32, f32) -> (f32, f32), // (nx, ny) -> (real, imag)
{
    let (width, height) = size;
    let mut data: Vec<f32> = Vec::with_capacity((width * height * 2) as usize);
    
    let half_extent_x = grid_extent.0 / 2.0;
    let half_extent_y = grid_extent.1 / 2.0;
    
    for y in 0..height {
        for x in 0..width {
            let nx = (x as f32 / width as f32) * grid_extent.0 - half_extent_x;
            let ny = (y as f32 / height as f32) * grid_extent.1 - half_extent_y;
            
            let (real, imag) = generator(nx, ny);
            data.push(real);
            data.push(imag);
        }
    }
    
    // Upload to GPU as RG format (Real, Imaginary)
    let image = glium::texture::RawImage2d {
        data: std::borrow::Cow::Owned(data),
        width,
        height,
        format: glium::texture::ClientFormat::F32F32,
    };
    
    let texture = Texture2d::with_format(
        display,
        image,
        UncompressedFloatFormat::F32F32,
        MipmapsOption::NoMipmap,
    ).unwrap();
    
    // Set wrap mode to ClampToEdge to prevent wrapping at boundaries
    texture.sampled()
        .wrap_function(SamplerWrapFunction::Clamp);
    
    texture
}

#[derive(Debug, Clone, Copy)]
pub struct WaveFunction {
    pub amplitude: f64,
    pub phase: f64,
}
impl WaveFunction {
    pub fn new(amplitude: f64, phase: f64) -> Self {
        Self {
            amplitude,
            phase,
        }
    }
    pub fn as_complex(&self) -> Complex<f64> {
        let real = self.amplitude * (self.phase).cos();
        let imag = self.amplitude * (self.phase).sin();
        Complex::new(real, imag)
    }
}

pub struct GaussianWavePacket {
    pub wave_function_map: Texture2d,
    // CENTER-POSITION
    pub center_x: f32,
    pub center_y: f32,
    pub sigma: f32,
    // MOMENTUM
    pub kx: f32,
    pub ky: f32,
}

impl GaussianWavePacket {
    pub fn new(
        display: &Display<WindowSurface>,
        center_x: f32,
        center_y: f32,
        sigma: f32,
        kx: f32,
        ky: f32,
        size: (u32, u32),
        grid_extent: (f32, f32),
    ) -> Self {
        // Use the shared helper with Gaussian generator
        let map = create_wavefunction_texture(display, size, grid_extent, |nx, ny| {
            let dx = nx - center_x;
            let dy = ny - center_y;
            let r_squared = dx * dx + dy * dy;
            let amplitude = (-r_squared / (2.0 * sigma * sigma)).exp();
            
            let phase = kx * nx + ky * ny;
            let real = amplitude * phase.cos();
            let imag = amplitude * phase.sin();
            (real, imag)
        });
        
        Self {
            wave_function_map: map,
            center_x,
            center_y,
            sigma,
            kx,
            ky,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ProbabilityDenstityFunction {
    pub wave_function: WaveFunction,
    pub probability_density: f64,
}

impl ProbabilityDenstityFunction {
    pub fn from_wave_function(wf: WaveFunction) -> Self {
        let probability_density = wf.amplitude * wf.amplitude;
        Self {
            wave_function: wf,
            probability_density,
        }
    }
}

pub struct QuantumState {
    pub wave_function: WaveFunction,
    pub position: (f64, f64, f64),
}

