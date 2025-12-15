use glium::{Display, glutin::surface::WindowSurface};
use glium::texture::{Texture2d, UncompressedFloatFormat, MipmapsOption};
use glium::buffer::{Buffer, BufferType, BufferMode};
use glium::GlObject;
use image;
use std::fs;
use crate::simulation::quantum_simulation::wave_function::GaussianWavePacket;
use crate::simulation::quantum_simulation::quantum_object::QuantumObject;
use crate::simulation::simulation::Simulation;
use crate::simulation::basic_simulation::BasicObject;
use crate::ui::manager::WidgetResponse;
use crate::render::shader_system::ShaderManager;
use crate::render::compute::{ComputeShader, TextureAccess, BarrierType};
use super::quantum_grid::QuantumGrid;

/// Main quantum simulation implementing the Schrödinger equation on GPU
pub struct QuantumSimulation {
    /// The quantum grid storing wavefunction and potential
    pub grid: QuantumGrid,
    
    /// Visualization mesh object
    pub quantum_object: QuantumObject,
    
    /// Compute shader for time evolution (Schrödinger equation)
    pub evolution_shader: ComputeShader,
    
    /// Compute shader for probability density calculation
    pub density_shader: ComputeShader,
    
    /// Compute shader for converting wavefunction to RGBA8 for export
    wavefunction_to_rgba_shader: ComputeShader,
    
    /// Compute shader for converting probability density to RGBA8 for export
    density_to_rgba_shader: ComputeShader,
    
    /// Compute shader for converting amplitude to RGBA8 for export
    amplitude_to_rgba_shader: ComputeShader,
    
    /// Compute shader for calculating unitarity (total probability)
    unitarity_shader: ComputeShader,
    
    /// Buffer for partial sums from unitarity shader
    unitarity_buffer: Buffer<[f32]>,
    
    /// RGBA8 intermediate texture for export
    rgba_export_texture: Texture2d,
    
    /// Simulation parameters
    pub params: QuantumParams,
    
    /// Simulation state
    pub paused: bool,
    pub time: f32,
    
    /// Auto-export settings
    pub auto_export: bool,
    pub export_interval: f32,
    pub last_export_time: f32,
    
    /// Frame counter for leapfrog integration (even/odd frames)
    frame_counter: u32,
    
    /// Total probability (should be ~1.0 for unitarity)
    pub total_probability: f32,
    
    /// Empty objects vector for trait compatibility
    empty_objects: Vec<BasicObject>,
}

#[derive(Debug, Clone, Copy)]
pub struct QuantumParams {
    // Most of these values are normalized such that units are hbar=1, mass=1
    pub hbar: f32,
    pub mass: f32,

    /// Time step for evolution (must be very small for stability!)
    pub dt: f32,
    
    pub dx: f32,
    pub dy: f32,
}

impl Default for QuantumParams {
    fn default() -> Self {
        Self {
            hbar: 1.0,
            mass: 1.0,
            dt: 0.001,  // Larger timestep for larger grid (10x increase)
            dx: 0.01,
            dy: 0.01,
        }
    }
}

impl QuantumSimulation {
    /// Create a new quantum simulation with specified grid size
    pub fn new(display: &Display<WindowSurface>, grid_size: u32) -> Self {
        let grid = QuantumGrid::new(display, grid_size, grid_size);
        let quantum_object = QuantumObject::new(display, (grid_size, grid_size));
        
        // Get physical grid spacing from the grid
        let (dx, dy) = grid.get_spacing();
        
        // Load compute shaders for Schrödinger equation solver
        let evolution_shader = ComputeShader::from_file("data/compute/evolution_leapfrog.glsl")
            .expect("Failed to load evolution compute shader");
        
        let density_shader = ComputeShader::from_file("data/compute/probability.glsl")
            .expect("Failed to load probability compute shader");
        
        // Load conversion shaders for export
        let wavefunction_to_rgba_shader = ComputeShader::from_file("data/compute/wavefunction_to_rgba.glsl")
            .expect("Failed to load wavefunction_to_rgba compute shader");
        
        let density_to_rgba_shader = ComputeShader::from_file("data/compute/density_to_rgba.glsl")
            .expect("Failed to load density_to_rgba compute shader");
        
        let amplitude_to_rgba_shader = ComputeShader::from_file("data/compute/amplitude_to_rgba.glsl")
            .expect("Failed to load amplitude_to_rgba compute shader");
        
        // Load unitarity compute shader
        let unitarity_shader = ComputeShader::from_file("data/compute/unitarity.glsl")
            .expect("Failed to load unitarity compute shader");
        
        // Create buffer for unitarity calculation (one value per workgroup)
        // For 256x256 grid with 16x16 workgroups, we need (256/16)² = 256 values
        let num_workgroups = ((grid_size + 15) / 16) * ((grid_size + 15) / 16);
        let unitarity_buffer = Buffer::empty_unsized(
            display,
            BufferType::ShaderStorageBuffer,
            num_workgroups as usize,
            BufferMode::Dynamic,
        ).unwrap();
        
        // Create RGBA8 intermediate texture for export
        let rgba_export_texture = Texture2d::empty_with_format(
            display,
            UncompressedFloatFormat::U8U8U8U8,
            MipmapsOption::NoMipmap,
            grid_size,
            grid_size,
        ).unwrap();
        
        // Initialize params with physical grid spacing
        let params = QuantumParams {
            dx,
            dy,
            ..Default::default()
        };
        
        Self {
            grid,
            quantum_object,
            evolution_shader,
            density_shader,
            wavefunction_to_rgba_shader,
            density_to_rgba_shader,
            amplitude_to_rgba_shader,
            unitarity_shader,
            unitarity_buffer,
            rgba_export_texture,
            params,
            paused: true,
            time: 0.0,
            auto_export: false,
            export_interval: 1.0,
            last_export_time: 0.0,
            frame_counter: 0,
            total_probability: 1.0,
            empty_objects: Vec::new(),
        }
    }

    /// Export the current wavefunction texture to an image file in the 'output' folder
    pub fn export_wavefunction_image(&self) {
        // 1. Convert F32F32 to RGBA8 using compute shader
        self.wavefunction_to_rgba_shader.bind();
        
        // Bind input (wavefunction) and output (RGBA8)
        self.wavefunction_to_rgba_shader.bind_sampler(0, &self.grid.wave_packet.wave_function_map);
        self.wavefunction_to_rgba_shader.set_uniform_int("u_wavefunction", 0);
        self.wavefunction_to_rgba_shader.bind_texture(1, &self.rgba_export_texture, TextureAccess::WriteOnly);
        
        // Dispatch conversion
        let (width, height) = self.grid.size;
        self.wavefunction_to_rgba_shader.dispatch_auto((width, height, 1));
        ComputeShader::barrier(BarrierType::ShaderImageAccess);
        
        // 2. Read RGBA8 texture (glium supports this!)
        let raw_data: Vec<Vec<(u8, u8, u8, u8)>> = self.rgba_export_texture.read();
        
        // 3. Convert to image buffer - try without Y-flip first to see orientation
        let mut img = image::RgbaImage::new(width, height);
        for (y, row) in raw_data.iter().enumerate() {
            if y >= height as usize {
                break;
            }
            for (x, &(r, g, b, a)) in row.iter().enumerate() {
                if x >= width as usize {
                    break;
                }
                // Try direct mapping (no flip) to see if that's the issue
                img.put_pixel(x as u32, y as u32, image::Rgba([r, g, b, a]));
            }
        }
        
        // 4. Save to file
        if let Err(e) = fs::create_dir_all("output") { 
            eprintln!("Failed to create output directory: {}", e);
            return;
        }
        
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let filename = format!("output/wavefunction_{}.png", timestamp);
        
        match img.save(&filename) {
            Ok(_) => println!("Exported wavefunction to {}", filename),
            Err(e) => eprintln!("Failed to export wavefunction: {}", e),
        }
    }
    
    pub fn setup_gaussian_wave_packet(
        &mut self,
        display: &Display<WindowSurface>,
        center_x: f32,
        center_y: f32,
        sigma: f32,
        kx: f32,
        ky: f32,
    ) {
        self.grid.wave_packet = GaussianWavePacket::new(
            display,
            center_x,
            center_y,
            sigma,
            kx,
            ky,
            self.grid.size,
            self.grid.grid_extent,
        );
        self.grid.wave_packet_mem = GaussianWavePacket::new(
            display,
            center_x,
            center_y,
            sigma,
            kx,
            ky,
            self.grid.size,
            self.grid.grid_extent,
        );
        self.time = 0.0;
    }
    
    pub fn setup_double_slit(
        &mut self,
        display: &Display<WindowSurface>,
        barrier_x: f32,
        barrier_thickness: f32,
        slit_width: f32,
        slit_separation: f32,
        barrier_height: f32,
    ) {
        self.grid.init_double_slit_potential(
            display,
            barrier_x,
            barrier_thickness,
            slit_width,
            slit_separation,
            barrier_height,
        );
    }
    
    /// Schrödinger equation
    pub fn evolve_wavefunction(&mut self, _display: &Display<WindowSurface>, dt: f32) {
        if self.paused {
            return;
        }


    // BINDING TO THE EVOLUTION SHADER
        self.evolution_shader.bind();
        
        self.evolution_shader.set_uniform_float("u_dt", dt * self.params.dt);
        self.evolution_shader.set_uniform_float("u_dx", self.params.dx);
        self.evolution_shader.set_uniform_float("u_dy", self.params.dy);
        self.evolution_shader.set_uniform_float("u_hbar", self.params.hbar);
        self.evolution_shader.set_uniform_float("u_mass", self.params.mass);
        
        // Pass frame parity for leapfrog integration: 0 for even, 1 for odd
        self.evolution_shader.set_uniform_int("u_pass", (self.frame_counter % 2) as i32);
        
        // Use texture units 4 and 5 to avoid conflicts with glium's default unit 0
        self.evolution_shader.set_uniform_int("u_current_psi", 4);
        self.evolution_shader.set_uniform_int("u_potential", 5);
        
        // Bind textures to higher units (4, 5) to avoid glium state cache conflicts
        self.evolution_shader.bind_sampler(4, &self.grid.wave_packet.wave_function_map);
        self.evolution_shader.bind_sampler(5, &self.grid.potential);
        
        // Image binding uses separate namespace from texture units
        self.evolution_shader.bind_texture(2, &self.grid.wave_packet_mem.wave_function_map, TextureAccess::WriteOnly);
        
        let (width, height) = self.grid.size;
        self.evolution_shader.dispatch_auto((width, height, 1));
        
        // Wait for compute to finish and make writes visible to texture fetches
        ComputeShader::barrier(BarrierType::ShaderImageAccess);
        ComputeShader::barrier(BarrierType::TextureFetch);
        
        // CRITICAL: Reset OpenGL state to prevent glium state desync
        unsafe { 
            gl::UseProgram(0);
            
            // Unbind our texture units
            gl::ActiveTexture(gl::TEXTURE4);
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::ActiveTexture(gl::TEXTURE5);
            gl::BindTexture(gl::TEXTURE_2D, 0);
            
            // Reset to unit 0 (glium's default expectation)
            gl::ActiveTexture(gl::TEXTURE0);
        }
    
        // Swap buffers (ping-pong)
        self.grid.swap_wavefunctions();
        
        unsafe {
            gl::UseProgram(0);
            gl::ActiveTexture(gl::TEXTURE4); gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::ActiveTexture(gl::TEXTURE5); gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::ActiveTexture(gl::TEXTURE0);
        }

        self.time += dt;
        self.frame_counter += 1;
    }
    
    /// Calculate total probability (unitarity check)
    /// Total probability should be ~1.0 if evolution preserves unitarity
    pub fn calculate_unitarity(&mut self) {
        // Bind shader and textures
        self.unitarity_shader.bind();
        self.unitarity_shader.bind_sampler(0, &self.grid.wave_packet.wave_function_map);
        self.unitarity_shader.set_uniform_int("u_wavefunction", 0);
        
        // Bind output buffer using glium's buffer binding
        unsafe {
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 1, self.unitarity_buffer.get_id());
        }
        
        // Dispatch compute shader
        let (width, height) = self.grid.size;
        self.unitarity_shader.dispatch_auto((width, height, 1));
        ComputeShader::barrier(BarrierType::ShaderStorageBuffer);
        
        // Read back partial sums and compute total
        let partial_sums = self.unitarity_buffer.read().unwrap();
        let mut total: f32 = partial_sums.iter().sum();
        
        // Multiply by cell area (dx * dy) to get integral
        total *= self.params.dx * self.params.dy;
        
        self.total_probability = total;
        
        // Reset OpenGL state
        unsafe {
            gl::UseProgram(0);
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 1, 0);
            gl::ActiveTexture(gl::TEXTURE0);
        }
    }
    
    /// Calculate probability density sort of broken
    pub fn calculate_probability_density(&mut self) {
        self.density_shader.bind();
        self.density_shader.bind_sampler(0, &self.grid.wave_packet.wave_function_map);
        self.density_shader.set_uniform_int("u_psi_texture", 0);
        self.density_shader.bind_texture(1, &self.grid.probability_density, TextureAccess::WriteOnly);
        
        let (width, height) = self.grid.size;
        self.density_shader.dispatch_auto((width, height, 1));
        
        // Wait for compute to finish
        ComputeShader::barrier(BarrierType::TextureFetch);
    }
    
    /// Get the grid size
    pub fn grid_size(&self) -> (u32, u32) {
        self.grid.size
    }
    
    /// Get reference to the probability density texture for rendering
    pub fn get_probability_texture(&self) -> &glium::texture::Texture2d {
        &self.grid.probability_density
    }
    
    /// Export the probability density (|ψ|²) to an image file
    pub fn export_probability_density_image(&self) {
        // 1. Convert wavefunction to grayscale probability density using compute shader
        self.density_to_rgba_shader.bind();
        
        // Bind input (wavefunction) and output (RGBA8)
        self.density_to_rgba_shader.bind_sampler(0, &self.grid.wave_packet.wave_function_map);
        self.density_to_rgba_shader.set_uniform_int("u_wavefunction", 0);
        self.density_to_rgba_shader.bind_texture(1, &self.rgba_export_texture, TextureAccess::WriteOnly);
        
        // Dispatch conversion
        let (width, height) = self.grid.size;
        self.density_to_rgba_shader.dispatch_auto((width, height, 1));
        ComputeShader::barrier(BarrierType::ShaderImageAccess);
        
        // 2. Read RGBA8 texture
        let raw_data: Vec<Vec<(u8, u8, u8, u8)>> = self.rgba_export_texture.read();
        
        // 3. Convert to image buffer
        let mut img = image::RgbaImage::new(width, height);
        for (y, row) in raw_data.iter().enumerate() {
            for (x, &(r, g, b, a)) in row.iter().enumerate() {
                img.put_pixel(x as u32, y as u32, image::Rgba([r, g, b, a]));
            }
        }
        
        // 4. Save to file
        if let Err(e) = fs::create_dir_all("output") {
            eprintln!("Failed to create output directory: {}", e);
            return;
        }
        
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let filename = format!("output/density_{}.png", timestamp);
        
        match img.save(&filename) {
            Ok(_) => println!("✅ Exported probability density to {}", filename),
            Err(e) => eprintln!("❌ Failed to export density: {}", e),
        }
    }

    /// Export the amplitude (|ψ|) to an image file with color mapping
    pub fn export_amplitude_image(&self) {
        // 1. Convert wavefunction amplitude to colored RGBA8 using compute shader
        self.amplitude_to_rgba_shader.bind();
        
        // Bind input (wavefunction) and output (RGBA8)
        self.amplitude_to_rgba_shader.bind_sampler(0, &self.grid.wave_packet.wave_function_map);
        self.amplitude_to_rgba_shader.set_uniform_int("u_wavefunction", 0);
        self.amplitude_to_rgba_shader.bind_texture(1, &self.rgba_export_texture, TextureAccess::WriteOnly);
        
        // Dispatch conversion
        let (width, height) = self.grid.size;
        self.amplitude_to_rgba_shader.dispatch_auto((width, height, 1));
        ComputeShader::barrier(BarrierType::ShaderImageAccess);
        
        // 2. Read RGBA8 texture
        let raw_data: Vec<Vec<(u8, u8, u8, u8)>> = self.rgba_export_texture.read();
        
        // 3. Convert to image buffer
        let mut img = image::RgbaImage::new(width, height);
        for (y, row) in raw_data.iter().enumerate() {
            for (x, &(r, g, b, a)) in row.iter().enumerate() {
                img.put_pixel(x as u32, y as u32, image::Rgba([r, g, b, a]));
            }
        }
        
        // 4. Save to file
        if let Err(e) = fs::create_dir_all("output") {
            eprintln!("Failed to create output directory: {}", e);
            return;
        }
        
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let filename = format!("output/amplitude_{}.png", timestamp);
        
        match img.save(&filename) {
            Ok(_) => println!("✅ Exported amplitude to {}", filename),
            Err(e) => eprintln!("❌ Failed to export amplitude: {}", e),
        }
    }

    /// Update auto-export logic
    pub fn update_auto_export(&mut self) {
        if self.auto_export && self.time - self.last_export_time >= self.export_interval {
            self.export_wavefunction_image();
            self.export_probability_density_image();
            self.export_amplitude_image();
            self.last_export_time = self.time;
        }
    }

    /// Render the quantum wavefunction visualization
    pub fn render(
        &self,
        display: &Display<WindowSurface>,
        target: &mut glium::Frame,
        view: [[f32; 4]; 4],
        projection: [[f32; 4]; 4],
    ) {
        self.quantum_object.render(
            display,
            target,
            &self.grid.wave_packet.wave_function_map,
            view,
            projection,
        );
    }
}

// Implement the Simulation trait for integration with the engine
impl Simulation for QuantumSimulation {
    type Object = BasicObject; // Using BasicObject for trait compatibility, but not used
    
    fn new() -> Self {
        panic!("Use QuantumSimulation::new(display, grid_size) instead");
    }
    
    fn update(&mut self, _delta_time: f32) {
        if !self.paused {
            // Call evolve_wavefunction manually from the example with display access
        }
    }
    
    fn add_object(&mut self, _object: Self::Object, _name: String) {
        // Not applicable for current simulations
    }
    
    fn remove_object(&mut self, _name: &str) -> Option<Self::Object> {
        None
    }
    
    fn get_object(&self, _name: &str) -> Option<&Self::Object> {
        None
    }
    
    fn get_object_mut(&mut self, _name: &str) -> Option<&mut Self::Object> {
        None
    }
    
    fn get_objects(&self) -> &Vec<Self::Object> {
        &self.empty_objects
    }
    
    fn get_object_names(&self) -> &[String] {
        &[]
    }
    
    fn handle_ui_response(
        &mut self,
        _response: WidgetResponse,
        _display: &Display<WindowSurface>,
        _shader_manager: &ShaderManager,
    ) {
    }
    
    fn is_paused(&self) -> bool {
        self.paused
    }
    
    fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }
    
    fn reset(&mut self) {
        self.time = 0.0;
        self.paused = true;
    }
}
