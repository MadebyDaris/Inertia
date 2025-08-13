use super::super::render::*;
use crate::{mesh::MeshObject, physics::{physicsobject::*, world::DiffuseLight, EulerAngles, Force}, simulation::orbital_simulation::astralBody::AstralCollisionObject, utils::vector::Vector};

use glium::{glutin::surface::WindowSurface, index::PrimitiveType, uniform, Display, Frame, IndexBuffer, Surface};

const G: f32 = (5) as f32;

#[allow(dead_code)]
pub struct PhysicsWorld<'a> {
    pub children: Vec<&'a dyn 
        PhysicsObject<
            Mesh = MeshObject, 
            Velocity = Vector, 
            Acceleration = Vector, 
            AngularVelocity = Vector, 
            AngularAcceleration = Vector, 
            Mass = f32, 
            Forces = Vec<Force>, 
            Torques = Vec<Vector>, 
            MomentOfInertia = f32, 
            EulerAngles = EulerAngles, 
            CollisionObject = AstralCollisionObject>>,
    pub camera: Camera,
    pub u_light: DiffuseLight
}
impl<'a> PhysicsWorld<'a> {
    /// Creates a new World instance
    pub fn new(
        children: Vec<&'a dyn PhysicsObject<Mesh = MeshObject, Velocity = Vector, Acceleration = Vector, AngularVelocity = Vector, AngularAcceleration = Vector, Mass = f32, Forces = Vec<Force>, Torques = Vec<Vector>, MomentOfInertia = f32, EulerAngles = EulerAngles, CollisionObject = AstralCollisionObject>>,
        camera: Camera, 
        u_light: DiffuseLight
    ) -> Self {
        PhysicsWorld { children, camera, u_light}
    }

    /// Render the world with its objects, camera, and lighting
    pub fn render(
        &mut self,
        screen: &Display<WindowSurface>,
        target: &mut Frame,  
        cam: &camera::CameraMat,
        u_light: DiffuseLight,
        background_color: (f32, f32, f32, f32)
    ){
        // let mut target = screen.draw();
        target.clear_color_and_depth(background_color, 1.0);
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },  .. Default::default()};
        
        for body in &self.children {
            let object = body.mesh();
            let mesh_object = &object.data;
            let mesh_uniform = &object.uniforms;

            let index_buffer = IndexBuffer::new(
                screen,                             // The `Display` object
                PrimitiveType::TrianglesList,         // Triangle list, as we're working with triangle primitives
                &mesh_uniform.indices                 // Reference to the indices Vec<u32>
            ).expect("Failed to create index buffer");

            let uni = uniform!{
                model: mesh_uniform.transform.matrix,           // Model matrix for object transformation
                view: cam.view_mat.matrix,                      // Camera's view matrix
                perspective: cam.pers_mat.matrix,               // Camera's perspective matrix
                u_light_direction: u_light.u_light_direction,   // Light source direction
                u_light_color: u_light.u_light_color,           // Light source color
                tex: &mesh_object.texture                       // Texture to apply to the mesh
            };
            // Draw the mesh object using the provided vertex buffer, indices, shaders, and uniforms
            target.draw(&mesh_object.vert_buffer, &index_buffer, &mesh_object.program, &uni, &params).unwrap();
        }
    }       
}