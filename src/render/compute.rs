use std::fs;
use std::ffi::CString;
use std::ptr;
use glium::texture::Texture2d;
use glium::GlObject;

pub struct ComputeShader {
    program_id: gl::types::GLuint,
    work_group_size: (u32, u32, u32),
}

impl ComputeShader {
    /// Load and compile a compute shader from file
    pub fn from_file(path: &str) -> Result<Self, String> {
        let source = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read compute shader {}: {}", path, e))?;
        
        Self::from_source(&source)
    }
    
    /// Compile a compute shader from source code
    pub fn from_source(source: &str) -> Result<Self, String> {
        unsafe {
            // Create shader
            let shader = gl::CreateShader(gl::COMPUTE_SHADER);
            
            // Compile shader
            let c_str = CString::new(source.as_bytes()).unwrap();
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
            gl::CompileShader(shader);
            
            // Check compilation status
            let mut success = gl::FALSE as gl::types::GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            
            if success != gl::TRUE as gl::types::GLint {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buffer = vec![0u8; len as usize];
                gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buffer.as_mut_ptr() as *mut gl::types::GLchar);
                gl::DeleteShader(shader);
                return Err(format!("Compute shader compilation failed:\n{}", 
                    String::from_utf8_lossy(&buffer)));
            }
            
            // Create program
            let program = gl::CreateProgram();
            gl::AttachShader(program, shader);
            gl::LinkProgram(program);
            
            // Check linking status
            let mut success = gl::FALSE as gl::types::GLint;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            
            if success != gl::TRUE as gl::types::GLint {
                let mut len = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                let mut buffer = vec![0u8; len as usize];
                gl::GetProgramInfoLog(program, len, ptr::null_mut(), buffer.as_mut_ptr() as *mut gl::types::GLchar);
                gl::DeleteProgram(program);
                gl::DeleteShader(shader);
                return Err(format!("Compute shader linking failed:\n{}", 
                    String::from_utf8_lossy(&buffer)));
            }
            
            gl::DeleteShader(shader);
            
            // Extract work group size from shader
            let work_group_size = Self::extract_work_group_size(source);
            
            Ok(ComputeShader {
                program_id: program,
                work_group_size,
            })
        }
    }
    
    /// Extract work group size from shader source (parses layout(local_size_x = ...) line)
    fn extract_work_group_size(source: &str) -> (u32, u32, u32) {
        let mut x = 1;
        let mut y = 1;
        let mut z = 1;
        
        for line in source.lines() {
            if line.contains("local_size_x") {
                if let Some(val) = line.split("local_size_x").nth(1)
                    .and_then(|s| s.split('=').nth(1))
                    .and_then(|s| s.split(|c: char| !c.is_numeric()).find(|s| !s.is_empty()))
                    .and_then(|s| s.parse().ok()) {
                    x = val;
                }
            }
            if line.contains("local_size_y") {
                if let Some(val) = line.split("local_size_y").nth(1)
                    .and_then(|s| s.split('=').nth(1))
                    .and_then(|s| s.split(|c: char| !c.is_numeric()).find(|s| !s.is_empty()))
                    .and_then(|s| s.parse().ok()) {
                    y = val;
                }
            }
            if line.contains("local_size_z") {
                if let Some(val) = line.split("local_size_z").nth(1)
                    .and_then(|s| s.split('=').nth(1))
                    .and_then(|s| s.split(|c: char| !c.is_numeric()).find(|s| !s.is_empty()))
                    .and_then(|s| s.parse().ok()) {
                    z = val;
                }
            }
        }
        
        (x, y, z)
    }
    
    /// Bind this compute shader for use
    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }
    
    /// Unbind compute shader
    pub fn unbind() {
        unsafe {
            gl::UseProgram(0);
        }
    }
    
    /// Set uniform float
    pub fn set_uniform_float(&self, name: &str, value: f32) {
        unsafe {
            let c_name = CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.program_id, c_name.as_ptr());
            gl::Uniform1f(location, value);
        }
    }
    
    /// Set uniform int
    pub fn set_uniform_int(&self, name: &str, value: i32) {
        unsafe {
            let c_name = CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.program_id, c_name.as_ptr());
            gl::Uniform1i(location, value);
        }
    }
    
    /// Bind a texture to a specific binding point
    pub fn bind_texture(&self, binding: u32, texture: &Texture2d, access: TextureAccess) {
        unsafe {
            let gl_access = match access {
                TextureAccess::ReadOnly => gl::READ_ONLY,
                TextureAccess::WriteOnly => gl::WRITE_ONLY,
                TextureAccess::ReadWrite => gl::READ_WRITE,
            };
            
            // Get internal texture format
            let internal_format = Self::get_texture_format(texture);
            
            gl::BindImageTexture(
                binding,
                texture.get_id(),
                0, // mipmap level
                gl::FALSE, // layered
                0, // layer
                gl_access,
                internal_format,
            );
        }
    }
    
    /// Bind a texture to a specific texture unit for sampling (sampler2D)
    /// Use this for ReadOnly textures accessed via sampler2D in the shader
    pub fn bind_sampler(&self, unit: u32, texture: &Texture2d) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::BindTexture(gl::TEXTURE_2D, texture.get_id());
        }
    }
    
    /// Get OpenGL internal format from glium texture
    fn get_texture_format(_texture: &Texture2d) -> gl::types::GLenum {
        // Try to determine format from texture
        // For now, assume common formats based on typical usage
        // You may need to extend this based on your texture types
        gl::RG32F // Common for complex wavefunction (Real, Imaginary)
    }
    
    /// Dispatch compute shader
    /// num_groups: Number of work groups in (x, y, z) dimensions
    pub fn dispatch(&self, num_groups: (u32, u32, u32)) {
        unsafe {
            gl::DispatchCompute(num_groups.0, num_groups.1, num_groups.2);
        }
    }
    
    /// Dispatch compute with automatic work group calculation
    /// grid_size: Total number of elements in (x, y, z) dimensions
    pub fn dispatch_auto(&self, grid_size: (u32, u32, u32)) {
        let (local_x, local_y, local_z) = self.work_group_size;
        let num_groups = (
            (grid_size.0 + local_x - 1) / local_x,
            (grid_size.1 + local_y - 1) / local_y,
            (grid_size.2 + local_z - 1) / local_z,
        );
        self.dispatch(num_groups);
    }
    
    /// Wait for compute operations to complete
    pub fn barrier(barrier_type: BarrierType) {
        unsafe {
            let bits = match barrier_type {
                BarrierType::ShaderImageAccess => gl::SHADER_IMAGE_ACCESS_BARRIER_BIT,
                BarrierType::TextureFetch => gl::TEXTURE_FETCH_BARRIER_BIT,
                BarrierType::All => gl::ALL_BARRIER_BITS,
            };
            gl::MemoryBarrier(bits);
        }
    }
    
    /// Get the program ID (for advanced usage)
    pub fn program_id(&self) -> u32 {
        self.program_id
    }
    
    /// Get work group size
    pub fn work_group_size(&self) -> (u32, u32, u32) {
        self.work_group_size
    }
}

impl Drop for ComputeShader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program_id);
        }
    }
}

/// Texture access mode for compute shaders
#[derive(Debug, Clone, Copy)]
pub enum TextureAccess {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

/// Memory barrier types
#[derive(Debug, Clone, Copy)]
pub enum BarrierType {
    ShaderImageAccess,
    TextureFetch,
    All,
}

/// Helper struct for managing compute shader pipelines
pub struct ComputePipeline {
    shaders: Vec<ComputeShader>,
}

impl ComputePipeline {
    pub fn new() -> Self {
        Self {
            shaders: Vec::new(),
        }
    }
    
    /// Add a compute shader to the pipeline
    pub fn add_shader(&mut self, shader: ComputeShader) {
        self.shaders.push(shader);
    }
    
    /// Load and add a compute shader from file
    pub fn load_shader(&mut self, path: &str) -> Result<usize, String> {
        let shader = ComputeShader::from_file(path)?;
        self.shaders.push(shader);
        Ok(self.shaders.len() - 1)
    }
    
    /// Get a shader by index
    pub fn get_shader(&self, index: usize) -> Option<&ComputeShader> {
        self.shaders.get(index)
    }
    
    /// Get a mutable reference to a shader by index
    pub fn get_shader_mut(&mut self, index: usize) -> Option<&mut ComputeShader> {
        self.shaders.get_mut(index)
    }
}

/// Initialize OpenGL function pointers
/// Call this once at startup before using compute shaders
/// 
/// Example:
/// ```
/// use glium::glutin::display::GetGlDisplay;
/// crate::render::compute::init_gl(|s| {
///     let c_str = std::ffi::CString::new(s).unwrap();
///     display.get_context().get_proc_address(&c_str)
/// });
/// ```
pub fn init_gl<F>(get_proc_address: F) 
where
    F: FnMut(&str) -> *const std::os::raw::c_void
{
    gl::load_with(get_proc_address);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_work_group_size_extraction() {
        let source = r#"
            #version 450
            layout(local_size_x = 16, local_size_y = 16) in;
            void main() {}
        "#;
        
        let size = ComputeShader::extract_work_group_size(source);
        assert_eq!(size, (16, 16, 1));
    }
}
