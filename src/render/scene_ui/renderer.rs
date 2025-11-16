use glium::{implement_vertex, uniform, Display, Frame, IndexBuffer, Program, Surface, VertexBuffer};
use crate::{
    render::{camera::Camera, scene_ui::{scene_ui_object::SceneUIObject, trail::Trail, vector_arrow::VectorArrow, outline::Outline}},
    utils::{matrix::TransformMatrix, vector::Vector},
};

#[derive(Copy, Clone)]
pub struct DebugVertex {
    pub position: [f32; 3],
}
implement_vertex!(DebugVertex, position);

pub struct SceneUIRenderer {
    arrow_shaft_program: Program,
    arrow_head_program: Program,
    trail_program: Program,
    outline_program: Program,
    arrow_shaft_vbo: VertexBuffer<DebugVertex>,
    arrow_shaft_ibo: IndexBuffer<u16>,
    arrow_head_vbo: VertexBuffer<DebugVertex>,
    arrow_head_ibo: IndexBuffer<u16>,
}

impl SceneUIRenderer {
    pub fn new(display: &Display<glium::glutin::surface::WindowSurface>) -> Self {
        let vertex_shader = r#"
            #version 140
            in vec3 position;
            uniform mat4 model;
            uniform mat4 view;
            uniform mat4 perspective;
            void main() {
                gl_Position = perspective * view * model * vec4(position, 1.0);
            }
        "#;

        let colored_fragment_shader = r#"
            #version 140
            uniform vec3 color;
            uniform float alpha;
            out vec4 f_color;
            void main() {
                f_color = vec4(color, alpha);
            }
        "#;

        let outline_fragment_shader = r#"
            #version 140
            uniform vec4 outline_color;
            out vec4 f_color;
            void main() {
                f_color = outline_color;
            }
        "#;

        let arrow_shaft_program = Program::from_source(display, vertex_shader, colored_fragment_shader, None).unwrap();
        let arrow_head_program = Program::from_source(display, vertex_shader, colored_fragment_shader, None).unwrap();
        let trail_program = Program::from_source(display, vertex_shader, colored_fragment_shader, None).unwrap();
        let outline_program = Program::from_source(display, vertex_shader, outline_fragment_shader, None).unwrap();

        let shaft_vertices = [
            DebugVertex { position: [0.0, 0.0, 0.0] },
            DebugVertex { position: [0.0, 0.0, 0.8] },
        ];
        let arrow_shaft_vbo = VertexBuffer::new(display, &shaft_vertices).unwrap();
        let arrow_shaft_ibo = IndexBuffer::new(display, glium::index::PrimitiveType::LinesList, &[0u16, 1]).unwrap();

        let segments = 6;
        let head_base_z = 0.8;
        let head_tip_z = 1.0;
        let head_radius = 0.1;
        
        let mut head_vertices = Vec::new();
        head_vertices.push(DebugVertex { position: [0.0, 0.0, head_tip_z] });
        
        for i in 0..segments {
            let angle = (i as f32 / segments as f32) * 2.0 * std::f32::consts::PI;
            let x = angle.cos() * head_radius;
            let y = angle.sin() * head_radius;
            head_vertices.push(DebugVertex { position: [x, y, head_base_z] });
        }
        head_vertices.push(DebugVertex { position: [head_radius, 0.0, head_base_z] });
        
        let arrow_head_vbo = VertexBuffer::new(display, &head_vertices).unwrap();
        let arrow_head_ibo = IndexBuffer::new(display, glium::index::PrimitiveType::TriangleFan, 
            &(0..=(segments as u16)).collect::<Vec<u16>>()).unwrap();

        Self {
            arrow_shaft_program,
            arrow_head_program,
            trail_program,
            outline_program,
            arrow_shaft_vbo,
            arrow_shaft_ibo,
            arrow_head_vbo,
            arrow_head_ibo,
        }
    }

    pub fn render(
        &self,
        display: &Display<glium::glutin::surface::WindowSurface>,
        frame: &mut Frame,
        camera: &Camera,
        ui_object: &SceneUIObject,
    ) {
        if ui_object.velocity_arrow.enabled && ui_object.velocity_arrow.vector.magnitude() > 0.01 {
            self.render_arrow(display, frame, camera, ui_object.position, &ui_object.velocity_arrow);
        }

        if ui_object.acceleration_arrow.enabled && ui_object.acceleration_arrow.vector.magnitude() > 0.001 {
            self.render_arrow(display, frame, camera, ui_object.position, &ui_object.acceleration_arrow);
        }

        for arrow in &ui_object.custom_arrows {
            if arrow.enabled && arrow.vector.magnitude() > 0.001 {
                self.render_arrow(display, frame, camera, ui_object.position, arrow);
            }
        }

        if ui_object.trail.enabled && ui_object.trail.positions.len() > 1 {
            self.render_trail(display, frame, camera, &ui_object.trail);
        }
    }

    pub fn render_outline<V: glium::vertex::Vertex>(
        &self,
        _display: &Display<glium::glutin::surface::WindowSurface>,
        frame: &mut Frame,
        camera: &Camera,
        vbo: &VertexBuffer<V>,
        ibo: &IndexBuffer<u16>,
        model_matrix: &TransformMatrix,
        outline: &Outline,
    ) {
        if !outline.enabled {
            return;
        }

        let scaled_model = model_matrix.scale(outline.offset, outline.offset, outline.offset);

        let uniforms = uniform! {
            model: scaled_model.matrix,
            view: camera.clone().view_matrix().matrix,
            perspective: camera.clone().get_perspective(camera.aspect_ratio, 1.0, 1024.0, 0.1).matrix,
            outline_color: outline.color,
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLessOrEqual,
                write: false,
                ..Default::default()
            },
            polygon_mode: glium::PolygonMode::Line,
            line_width: Some(outline.thickness),
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        frame.draw(vbo, ibo, &self.outline_program, &uniforms, &params).unwrap();
    }

    fn render_arrow(
        &self,
        _display: &Display<glium::glutin::surface::WindowSurface>,
        frame: &mut Frame,
        camera: &Camera,
        base_position: Vector,
        arrow: &VectorArrow,
    ) {
        let start = base_position;
        let direction = arrow.vector.normalized();
        let length = (arrow.vector * arrow.scale).magnitude();

        if length < 0.001 {
            return;
        }

        let rotation = TransformMatrix::rotation_from_direction(direction);
        let translation = TransformMatrix::translation(start.0, start.1, start.2);
        let scale_matrix = TransformMatrix::identity().scale(length, length, length);
        let model_matrix = translation * rotation * scale_matrix;

        let uniforms = uniform! {
            model: model_matrix.matrix,
            view: camera.clone().view_matrix().matrix,
            perspective: camera.clone().get_perspective(camera.aspect_ratio, 1.0, 1024.0, 0.1).matrix,
            color: arrow.color,
            alpha: 1.0f32,
        };

        let shaft_params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            line_width: Some(arrow.thickness),
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        let head_params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        frame.draw(&self.arrow_shaft_vbo, &self.arrow_shaft_ibo, &self.arrow_shaft_program, &uniforms, &shaft_params).unwrap();
        frame.draw(&self.arrow_head_vbo, &self.arrow_head_ibo, &self.arrow_head_program, &uniforms, &head_params).unwrap();
    }

    fn render_trail(
        &self,
        display: &Display<glium::glutin::surface::WindowSurface>,
        frame: &mut Frame,
        camera: &Camera,
        trail: &Trail,
    ) {
        let vertices: Vec<DebugVertex> = trail.positions
            .iter()
            .map(|pos| DebugVertex { position: [pos.0, pos.1, pos.2] })
            .collect();

        if vertices.len() < 2 {
            return;
        }

        let indices: Vec<u16> = (0..vertices.len() as u16).collect();

        let vbo = VertexBuffer::new(display, &vertices).unwrap();
        let ibo = IndexBuffer::new(display, glium::index::PrimitiveType::LineStrip, &indices).unwrap();

        let uniforms = uniform! {
            model: TransformMatrix::identity().matrix,
            view: camera.clone().view_matrix().matrix,
            perspective: camera.clone().get_perspective(camera.aspect_ratio, 1.0, 1024.0, 0.1).matrix,
            color: trail.color,
            alpha: 0.4f32,
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: false,
                ..Default::default()
            },
            line_width: Some(1.5),
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        frame.draw(&vbo, &ibo, &self.trail_program, &uniforms, &params).unwrap();
    }
}
