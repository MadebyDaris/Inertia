use glium::{self, Surface, glutin::surface::WindowSurface, Display, Frame, VertexBuffer};
use glium::index::PrimitiveType;
use crate::utils::matrix::TransformMatrix;
use crate::mesh::mesh_object::{Vertex, Mesh, ShaderData};
use crate::render::shader_system::{ShaderManager, ShaderType, MaterialProperties};
use std::rc::Rc;

pub struct BarrierMesh {
    pub mesh: Mesh,
    pub transform: TransformMatrix,
    pub indices: Vec<u32>,
}

impl BarrierMesh {
    pub fn new(
        display: &Display<WindowSurface>,
        shader_manager: &ShaderManager,
        barrier_x: f32,
        barrier_thickness: f32,
        slit_width: f32,
        slit_separation: f32,
        grid_height: f32,
    ) -> Self {
        let slit_half_sep = slit_separation / 2.0; // distance from the center of slit
        let slit_half_width = slit_width / 2.0;

        // Slit 1
        let slit1_z_min = slit_half_sep - slit_half_width;
        let slit1_z_max = slit_half_sep + slit_half_width;
        
        // Slit 2
        let slit2_z_min = -slit_half_sep - slit_half_width;
        let slit2_z_max = -slit_half_sep + slit_half_width;



        let half_thickness = barrier_thickness / 2.0;
        let x_min = barrier_x - half_thickness;
        let x_max = barrier_x + half_thickness;
        

        let y_bottom = 0.0;
        let y_top = 4.0;


        let z_min = -grid_height / 2.0;
        let z_max = grid_height / 2.0;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Helper to add a wall segment from z_start to z_end
        let mut add_wall_segment = |z_start: f32, z_end: f32| {
            let base_idx = vertices.len() as u32;
            
            // Normal pointing towards -X (direction of wave travel)
            let normal = [-1.0, 0.0, 0.0];
            
            vertices.push(Vertex { position: [x_min, y_bottom, z_start], normal, tex_coords: [0.0, 0.0] });
            vertices.push(Vertex { position: [x_min, y_bottom, z_end], normal, tex_coords: [1.0, 0.0] });
            vertices.push(Vertex { position: [x_min, y_top, z_end], normal, tex_coords: [1.0, 1.0] });
            vertices.push(Vertex { position: [x_min, y_top, z_start], normal, tex_coords: [0.0, 1.0] });
            


            // Normal pointing towards +X
            let normal = [1.0, 0.0, 0.0];
            
            vertices.push(Vertex { position: [x_max, y_bottom, z_start], normal, tex_coords: [0.0, 0.0] });
            vertices.push(Vertex { position: [x_max, y_bottom, z_end], normal, tex_coords: [1.0, 0.0] });
            vertices.push(Vertex { position: [x_max, y_top, z_end], normal, tex_coords: [1.0, 1.0] });
            vertices.push(Vertex { position: [x_max, y_top, z_start], normal, tex_coords: [0.0, 1.0] });
            
            indices.extend_from_slice(&[
                base_idx, base_idx + 1, base_idx + 2,
                base_idx, base_idx + 2, base_idx + 3,
            ]);
            
            indices.extend_from_slice(&[
                base_idx + 4, base_idx + 6, base_idx + 5,
                base_idx + 4, base_idx + 7, base_idx + 6,
            ]);
        };

        add_wall_segment(z_min, slit2_z_min);
        
        add_wall_segment(slit2_z_max, slit1_z_min);
        
        add_wall_segment(slit1_z_max, z_max);

        let shader_data = ShaderData {
            tex_filename: "./data/tex/grid.png".to_string(),
            shader_type: ShaderType::Unlit,
            material: MaterialProperties {
                color: [0.6, 0.65, 0.7],
                emission_strength: 0.0,
                roughness: 0.5,
                metallic: 0.0,
                specular_intensity: 0.0,
                shininess: 1.0,
                ambient_occlusion: 1.0,
            },
        };

        let mesh = Mesh::new(display, &vertices, shader_data, shader_manager);

        let transform = TransformMatrix::identity();

        BarrierMesh {
            mesh,
            transform,
            indices,
        }
    }

    pub fn render(
        &self,
        display: &Display<WindowSurface>,
        target: &mut Frame,
        view: [[f32; 4]; 4],
        perspective: [[f32; 4]; 4],
    ) {
        use glium::uniform;
        let matcolor: [f32; 3] = [0.6, 0.65, 0.7];
        let uniforms = uniform! {
            model: self.transform.matrix,
            view: view,
            perspective: perspective,
            tex: &self.mesh.texture,
            material_color: matcolor,
            emission_strength: 0.0,
            roughness: 0.5,
            metallic: 0.0,
            specular_intensity: 0.0,
            shininess: 1.0,
            ambient_occlusion: 1.0,
            u_light_direction: [0.5, -0.5, 0.5],
            u_light_color: [1.0, 1.0, 1.0],
            camera_position: [0.0, 50.0, 80.0],
        };

        let indices = glium::IndexBuffer::new(display, PrimitiveType::TrianglesList, &self.indices).unwrap();

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            blend: glium::Blend::alpha_blending(),
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullingDisabled,
            ..Default::default()
        };

        target.draw(
            &self.mesh.vert_buffer,
            &indices,
            &self.mesh.program,
            &uniforms,
            &params,
        ).unwrap();
    }
}
