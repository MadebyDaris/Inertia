use glium::{Display, glutin::surface::WindowSurface};
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
    
    /// Simulation parameters
    pub params: QuantumParams,
    
    /// Simulation state
    pub paused: bool,
    pub time: f32,
    
    /// Empty objects vector for trait compatibility
    empty_objects: Vec<BasicObject>,
}

#[derive(Debug, Clone, Copy)]
pub struct QuantumParams {
    /// Planck's constant (reduced) - ℏ (normalized to 1.0)
    pub hbar: f32,
    
    /// Particle mass (normalized to 1.0)
    pub mass: f32,

    /// Time step for evolution (must be very small for stability!)
    pub dt: f32,
    
    /// Spatial grid spacing
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
        let evolution_shader = ComputeShader::from_file("data/compute/evolution.glsl")
            .expect("Failed to load evolution compute shader");
        
        let density_shader = ComputeShader::from_file("data/compute/probability.glsl")
            .expect("Failed to load probability compute shader");
        
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
            params,
            paused: true,
            time: 0.0,
            empty_objects: Vec::new(),
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
        
        self.time += dt;
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
