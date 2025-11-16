pub mod mesh_object;
pub mod sphere;
pub mod grid_floor;
pub use mesh_object::*;

use glium::{glutin::surface::WindowSurface, index::PrimitiveType, uniform, Display, Frame, IndexBuffer, Surface, VertexBuffer};
use wfobj::*;
use std::rc::Rc;

use crate::utils::matrix::TransformMatrix;
use crate::render::shader_system::ShaderManager;



// 
// Mesh Object
// 
pub struct MeshObject {
    pub data: Mesh,
    pub uniforms: MeshUniforms,
}
impl MeshObject {
    // Create a new MeshObject, given data and uniforms
    pub fn new(screen: &Display<WindowSurface>, object_data: &Vec<mesh_object::Vertex>, indices_raw:Vec<u32>, shader_data: ShaderData, shader_manager: &ShaderManager) -> MeshObject {
        let vert_buffer = VertexBuffer::new(screen, object_data).unwrap().into();

        let texture = Mesh::texture(screen, shader_data.tex_filename.as_str());

        let program = shader_manager.get_program(shader_data.shader_type)
            .expect("Failed to get shader program");

        let data = Mesh {
            vert_buffer,
            texture,
            program: Rc::clone(program),
        };

        let uniforms = MeshUniforms {
            transform: TransformMatrix::identity(), // Identity matrix for default
            indices: indices_raw,
            material: shader_data.material.clone(),
            shader_type: shader_data.shader_type,
        };

        MeshObject { data, uniforms }
    }
    // Render the mesh using the provided uniforms
    pub fn render(&self, screen: &Display<WindowSurface>, target: &mut Frame, view: [[f32; 4]; 4], perspective: [[f32; 4]; 4]) {
        let indices = IndexBuffer::new(screen, PrimitiveType::TrianglesList, &self.uniforms.indices).unwrap();
        // Set uniforms for rendering
        let uniforms = uniform! {
            model: self.uniforms.transform.matrix,
            view: view,
            perspective: perspective,
            tex: &self.data.texture,
            material_color: self.uniforms.material.color,
            emission_strength: self.uniforms.material.emission_strength,
            roughness: self.uniforms.material.roughness,
            metallic: self.uniforms.material.metallic,
            specular_intensity: self.uniforms.material.specular_intensity,
            shininess: self.uniforms.material.shininess,
            ambient_occlusion: self.uniforms.material.ambient_occlusion,
            u_light_direction: [0.5_f32, -0.5_f32, 0.5_f32],
            u_light_color: [1.0_f32, 1.0_f32, 1.0_f32],
            camera_position: [0.0_f32, 50.0_f32, 80.0_f32],
        };
        
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            blend: glium::Blend::alpha_blending(),
            .. Default::default()
        };
        
        // Draw call
        target
            .draw(
                &self.data.vert_buffer,
                &indices,
                &self.data.program,
                &uniforms,
                &params,
            )
            .unwrap();
    }
    pub fn translate(&mut self, kx:f32, ky:f32 ,kz:f32) {
        self.uniforms.transform = self.uniforms.transform.translate(kx, ky, kz);
    }
    pub fn rotate(&mut self, kx:f32, ky:f32 ,kz:f32) {
        self.uniforms.transform.rotate((kx, ky, kz));
    }
    pub fn scale(&mut self, kx:f32, ky:f32 ,kz:f32) {
        self.uniforms.transform = self.uniforms.transform.scale(kx, ky, kz);
    }
}

// 
// Importing Models
// 

pub fn load_gltf(filename: &str) -> MeshData {
    let (gltf, buffers, _) = gltf::import(filename).unwrap();
    
    let mut vert_pos: Vec<[f32;3]> =  vec![];
    let mut vert_norm: Vec<[f32;3]> =  vec![];
    let mut vert_tex: Vec<[f32;2]> =  vec![];
    let mut indices: Vec<u32> =  vec![];

    for mesh in gltf.meshes() {
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            if let Some(iter) = reader.read_positions() {
                for vertex_position in iter {
                    vert_pos.push(vertex_position)}}
            if let Some(iter) = reader.read_normals() {
                for vertex_normal in iter {
                    vert_norm.push(vertex_normal)}}
            if let Some(iter) = reader.read_tex_coords(0) {
                for vertex_tex_coord in iter.into_f32() {
                    vert_tex.push(vertex_tex_coord)}}
            for vert_ind in reader.read_indices().unwrap().into_u32() {
                indices.push(vert_ind);}
            }       
        }

    let mut vertex_data: Vec<Vertex> = Vec::new(); 
    for i in indices {
        let i: usize = i.try_into().unwrap();
        vertex_data.push(Vertex { position: vert_pos[i], normal: vert_norm[i], tex_coords: vert_tex[i] });
    }
    return MeshData {verts: vertex_data}
}

pub fn load_obj_file(filename: &str) -> MeshData {
    // PARSING THE FILE
        let wrld = parse_file(filename).unwrap();
        let (v, n, t) = (wrld.vertices, wrld.normals, wrld.textures);

        let mut vertex_data: Vec<mesh_object::Vertex> = Vec::new();
        for face in wrld.faces {
            for vert in face { let mut x: [f32;3] = [0.;3];
            
                // POSITION
                for i in 0..2 { 
                    let index_v: usize = (vert[0].saturating_sub(1)).try_into().unwrap();
                    x[i] = v[index_v][i]; 
                }
                let position = a4_2_a3(v[vert[0] as usize - 1]);

                // NORMALS
                let index_n = vert[2].saturating_sub(1);
                let normal = n[index_n as usize];

                // TEXTURES
                let index_tx = vert[1].saturating_sub(1); 
                let mut tex_coords = [0.;2];
                for i in 0..1 { 
                    tex_coords[i] = t[index_tx as usize][i];
                }

                vertex_data.push(Vertex {
                    position,
                    normal,
                    tex_coords,
                });
            }
        }
        return MeshData { verts: vertex_data};
}