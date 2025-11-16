use glium::{Display, Program};
use glium::glutin::surface::WindowSurface;
use std::{collections::HashMap, fs::File, io::Read, rc::Rc};

/// Shader type enumeration for different material properties
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderType {
    /// Basic diffuse lighting with texture
    Diffuse,
    /// Emissive shader for light-emitting objects (stars, etc.)
    Emission,
    /// Glossy/specular shader with shininess
    Glossy,
    /// Physically-based rendering shader
    PBR,
    /// Unlit shader (no lighting calculations)
    Unlit,
    /// Custom shader loaded from files
    Custom(usize),
}

/// Material properties that can be passed to shaders
#[derive(Debug, Clone)]
pub struct MaterialProperties {
    /// Base color tint (multiplied with texture)
    pub color: [f32; 3],
    /// Emission strength (for emissive materials)
    pub emission_strength: f32,
    /// Roughness (0 = smooth, 1 = rough) for PBR
    pub roughness: f32,
    /// Metallic property (0 = dielectric, 1 = metal) for PBR
    pub metallic: f32,
    /// Specular intensity for glossy materials
    pub specular_intensity: f32,
    /// Shininess/glossiness factor
    pub shininess: f32,
    /// Ambient occlusion factor
    pub ambient_occlusion: f32,
}

impl Default for MaterialProperties {
    fn default() -> Self {
        Self {
            color: [1.0, 1.0, 1.0],
            emission_strength: 0.0,
            roughness: 0.5,
            metallic: 0.0,
            specular_intensity: 0.5,
            shininess: 32.0,
            ambient_occlusion: 1.0,
        }
    }
}

/// Shader manager that handles compilation and caching of shaders
pub struct ShaderManager {
    programs: HashMap<ShaderType, Rc<Program>>,
    custom_shaders: Vec<(String, String)>, // vertex, fragment paths
}

impl ShaderManager {
    /// Create a new shader manager
    pub fn new() -> Self {
        Self {
            programs: HashMap::new(),
            custom_shaders: Vec::new(),
        }
    }

    /// Initialize all built-in shaders
    pub fn initialize_shaders(&mut self, display: &Display<WindowSurface>) {
        // Load built-in shaders from files
        self.programs.insert(
            ShaderType::Diffuse,
            Rc::new(self.load_and_compile_shader(display, 
                "data/glsl/diffuse_vertex_shader.glsl", 
                "data/glsl/diffuse_fragment_shader.glsl")),
        );
        
        self.programs.insert(
            ShaderType::Emission,
            Rc::new(self.load_and_compile_shader(display, 
                "data/glsl/emission_vertex_shader.glsl", 
                "data/glsl/emission_fragment_shader.glsl")),
        );
        
        self.programs.insert(
            ShaderType::Glossy,
            Rc::new(self.load_and_compile_shader(display, 
                "data/glsl/glossy_vertex_shader.glsl", 
                "data/glsl/glossy_fragment_shader.glsl")),
        );
        
        self.programs.insert(
            ShaderType::PBR,
            Rc::new(self.load_and_compile_shader(display, 
                "data/glsl/pbr_vertex_shader.glsl", 
                "data/glsl/pbr_fragment_shader.glsl")),
        );
        
        self.programs.insert(
            ShaderType::Unlit,
            Rc::new(self.load_and_compile_shader(display, 
                "data/glsl/unlit_vertex_shader.glsl", 
                "data/glsl/unlit_fragment_shader.glsl")),
        );
    }

    /// Add a custom shader from file paths
    pub fn add_custom_shader(
        &mut self,
        display: &Display<WindowSurface>,
        vertex_path: &str,
        fragment_path: &str,
    ) -> ShaderType {
        let index = self.custom_shaders.len();
        self.custom_shaders.push((vertex_path.to_string(), fragment_path.to_string()));
        
        let shader_type = ShaderType::Custom(index);
        let program = Rc::new(self.load_and_compile_shader(display, vertex_path, fragment_path));
        self.programs.insert(shader_type, program);
        
        shader_type
    }

    /// Get a compiled shader program
    pub fn get_program(&self, shader_type: ShaderType) -> Option<&Rc<Program>> {
        self.programs.get(&shader_type)
    }

    /// Load and compile a shader from file paths
    fn load_and_compile_shader(&self, display: &Display<WindowSurface>, vertex_path: &str, fragment_path: &str) -> Program {
        let vertex = Self::read_shader_file(vertex_path);
        let fragment = Self::read_shader_file(fragment_path);
        self.compile_shader(display, &vertex, &fragment)
    }

    /// Compile a shader from source strings
    fn compile_shader(&self, display: &Display<WindowSurface>, vertex: &str, fragment: &str) -> Program {
        Program::from_source(display, vertex, fragment, None)
            .unwrap_or_else(|e| panic!("Failed to compile shader: {}", e))
    }

    /// Read shader source from file
    fn read_shader_file(path: &str) -> String {
        let mut file = File::open(path)
            .unwrap_or_else(|e| panic!("Failed to open shader file {}: {}", path, e));
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .unwrap_or_else(|e| panic!("Failed to read shader file {}: {}", path, e));
        contents
    }

    /// Reload a shader (useful for hot-reloading during development)
    pub fn reload_shader(&mut self, display: &Display<WindowSurface>, shader_type: ShaderType) {
        match shader_type {
            ShaderType::Custom(index) => {
                if let Some((vertex_path, fragment_path)) = self.custom_shaders.get(index) {
                    let program = Rc::new(self.load_and_compile_shader(display, vertex_path, fragment_path));
                    self.programs.insert(shader_type, program);
                }
            }
            ShaderType::Diffuse => {
                let program = Rc::new(self.load_and_compile_shader(display, 
                    "data/glsl/diffuse_vertex_shader.glsl", 
                    "data/glsl/diffuse_fragment_shader.glsl"));
                self.programs.insert(shader_type, program);
            }
            ShaderType::Emission => {
                let program = Rc::new(self.load_and_compile_shader(display, 
                    "data/glsl/emission_vertex_shader.glsl", 
                    "data/glsl/emission_fragment_shader.glsl"));
                self.programs.insert(shader_type, program);
            }
            ShaderType::Glossy => {
                let program = Rc::new(self.load_and_compile_shader(display, 
                    "data/glsl/glossy_vertex_shader.glsl", 
                    "data/glsl/glossy_fragment_shader.glsl"));
                self.programs.insert(shader_type, program);
            }
            ShaderType::PBR => {
                let program = Rc::new(self.load_and_compile_shader(display, 
                    "data/glsl/pbr_vertex_shader.glsl", 
                    "data/glsl/pbr_fragment_shader.glsl"));
                self.programs.insert(shader_type, program);
            }
            ShaderType::Unlit => {
                let program = Rc::new(self.load_and_compile_shader(display, 
                    "data/glsl/unlit_vertex_shader.glsl", 
                    "data/glsl/unlit_fragment_shader.glsl"));
                self.programs.insert(shader_type, program);
            }
        }
    }
}
