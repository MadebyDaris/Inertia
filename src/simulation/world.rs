#[allow(dead_code)]
use super::super::render::*;
use crate::mesh::MeshObject;

use glium::{glutin::surface::WindowSurface, index::PrimitiveType, uniform, IndexBuffer, Surface};
#[derive(Clone, Copy)]
pub struct DiffuseLight {
    pub u_light_direction: (f32, f32, f32),
    pub u_light_color: (f32, f32, f32),
}

#[allow(dead_code)]
pub trait World {
    fn render(&mut self, screen: &glium::Display<WindowSurface>, cam: &CameraMat, u_light: DiffuseLight, background_color: (f32,f32,f32,f32));
}

#[allow(dead_code)]
pub struct StationnaryWorld<'a> {
    pub children: Vec<&'a MeshObject>,
    pub camera: Camera,
    pub u_light: DiffuseLight
}
#[allow(dead_code)]
impl<'a> StationnaryWorld<'a> {
    /// Creates a new World instance
    pub fn new(
        children: Vec<&'a MeshObject>,
        camera: Camera, 
        u_light: DiffuseLight
    ) -> Self {
        StationnaryWorld { children, camera, u_light}
    }
}
impl World for StationnaryWorld<'_> {
    /// Render the world with its objects, camera, and lighting
    fn render(
        &mut self, 
        screen: &glium::Display<WindowSurface>,  
        cam: &CameraMat,
        u_light: DiffuseLight,
        background_color: (f32, f32, f32, f32)
    ){
        let mut target = screen.draw();
        target.clear_color_and_depth(background_color, 1.);
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },  .. Default::default()};
        
        for i in self.children.iter().enumerate() {
            
            let mesh_object = &self.children[i.0].data;
            let mesh_uniform = &self.children[i.0].uniforms;

            let index_buffer = IndexBuffer::new(
                screen,                // The `Display` object
                PrimitiveType::TrianglesList, // Triangle list, as we're working with triangle primitives
                &mesh_uniform.indices                // Reference to the indices Vec<u32>
            ).expect("Failed to create index buffer");

            let uni = uniform!{
                model: mesh_uniform.transform.matrix,  // Model matrix for object transformation
                view: cam.view_mat,            // Camera's view matrix
                perspective: cam.pers_mat,               // Camera's perspective matrix
                u_light_direction: u_light.u_light_direction,                     // Light source position/intensity
                u_light_color: u_light.u_light_color,
                tex: &mesh_object.texture             // Texture to apply to the mesh
            };
            // Draw the mesh object using the provided vertex buffer, indices, shaders, and uniforms
            target.draw(&mesh_object.vert_buffer, &index_buffer, &mesh_object.program, &uni, &params).unwrap();
        }
        target.finish().unwrap();
    }
}