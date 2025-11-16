use glium::{implement_vertex, uniform, Display, Frame, IndexBuffer, Program, Surface, VertexBuffer};
use crate::{render::camera::Camera, utils::{matrix::TransformMatrix, vector::Vector}};

#[derive(Copy, Clone, Debug)]
pub struct DebugArrow {
    pub start: Vector,
    pub end: Vector,
    pub color: [f32; 3],
    pub thickness: f32,
    pub scale: f32,
}

impl DebugArrow {
    pub fn new(start: Vector, end: Vector) -> Self {
        Self {
            start,
            end,
            color: [1.0, 0.0, 0.0],
            thickness: 1.0,
            scale: 1.0,
        }
    }

    pub fn with_color(mut self, r: f32, g: f32, b: f32) -> Self {
        self.color = [r, g, b];
        self
    }

    pub fn with_thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }
}

#[derive(Copy, Clone, Debug)]
pub struct DebugOutline {
    pub color: [f32; 4],
    pub thickness: f32,
    pub offset: f32,
}

impl DebugOutline {
    pub fn new() -> Self {
        Self {
            color: [1.0, 1.0, 0.0, 1.0],
            thickness: 2.0,
            offset: 1.02,
        }
    }

    pub fn with_color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.color = [r, g, b, a];
        self
    }

    pub fn with_thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    pub fn with_offset(mut self, offset: f32) -> Self {
        self.offset = offset;
        self
    }
}

#[derive(Copy, Clone)]
struct DebugVertex {
    position: [f32; 3],
}
implement_vertex!(DebugVertex, position);

pub struct DebugRenderer {
    arrow_shaft_program: Program,
    arrow_head_program: Program,
    outline_program: Program,
}

impl DebugRenderer {
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
        let outline_program = Program::from_source(display, vertex_shader, outline_fragment_shader, None).unwrap();

        Self {
            arrow_shaft_program,
            arrow_head_program,
            outline_program,
        }
    }

    pub fn draw_arrow(&self, display: &Display<glium::glutin::surface::WindowSurface>, frame: &mut Frame, camera: &Camera, arrow: &DebugArrow) {
        let direction = (arrow.end - arrow.start).normalized();
        let length = (arrow.end - arrow.start).magnitude() * arrow.scale;

        if length < 0.001 {
            return;
        }

        let shaft_length = length * 0.8;
        let head_size = length * 0.2;

        let shaft_vertices = [
            DebugVertex { position: [0.0, 0.0, 0.0] },
            DebugVertex { position: [0.0, 0.0, shaft_length] },
        ];

        let head_base_z = shaft_length;
        let head_tip_z = length;
        let head_radius = head_size * 0.5;

        let segments = 8;
        let mut head_vertices = Vec::new();
        head_vertices.push(DebugVertex { position: [0.0, 0.0, head_tip_z] });
        
        for i in 0..segments {
            let angle = (i as f32 / segments as f32) * 2.0 * std::f32::consts::PI;
            let x = angle.cos() * head_radius;
            let y = angle.sin() * head_radius;
            head_vertices.push(DebugVertex { position: [x, y, head_base_z] });
        }
        head_vertices.push(DebugVertex { position: [head_radius, 0.0, head_base_z] });

        let shaft_vbo = VertexBuffer::new(display, &shaft_vertices).unwrap();
        let head_vbo = VertexBuffer::new(display, &head_vertices).unwrap();
        
        let shaft_ibo = IndexBuffer::new(display, glium::index::PrimitiveType::LinesList, &[0u16, 1]).unwrap();
        let head_ibo = IndexBuffer::new(display, glium::index::PrimitiveType::TriangleFan, 
            &(0..=(segments as u16)).collect::<Vec<u16>>()).unwrap();

        let rotation = TransformMatrix::rotation_from_direction(direction);
        let model_matrix = TransformMatrix::translation(arrow.start.0, arrow.start.1, arrow.start.2) * rotation;

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

        frame.draw(&shaft_vbo, &shaft_ibo, &self.arrow_shaft_program, &uniforms, &shaft_params).unwrap();
        frame.draw(&head_vbo, &head_ibo, &self.arrow_head_program, &uniforms, &head_params).unwrap();
    }

    pub fn draw_outline<V: glium::vertex::Vertex>(
        &self,
        _display: &Display<glium::glutin::surface::WindowSurface>,
        frame: &mut Frame,
        camera: &Camera,
        vbo: &VertexBuffer<V>,
        ibo: &IndexBuffer<u16>,
        model_matrix: &TransformMatrix,
        outline: &DebugOutline,
    ) {
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

    pub fn draw_velocity_arrow(&self, display: &Display<glium::glutin::surface::WindowSurface>, frame: &mut Frame, camera: &Camera, position: Vector, velocity: Vector, scale: f32) {
        let arrow = DebugArrow::new(position, position + velocity * scale)
            .with_color(1.0, 0.0, 0.0)
            .with_thickness(2.0)
            .with_scale(1.0);
        self.draw_arrow(display, frame, camera, &arrow);
    }

    pub fn draw_acceleration_arrow(&self, display: &Display<glium::glutin::surface::WindowSurface>, frame: &mut Frame, camera: &Camera, position: Vector, acceleration: Vector, scale: f32) {
        let arrow = DebugArrow::new(position, position + acceleration * scale)
            .with_color(0.0, 1.0, 0.0)
            .with_thickness(2.0)
            .with_scale(1.0);
        self.draw_arrow(display, frame, camera, &arrow);
    }

    pub fn draw_force_arrow(&self, display: &Display<glium::glutin::surface::WindowSurface>, frame: &mut Frame, camera: &Camera, position: Vector, force: Vector, scale: f32) {
        let arrow = DebugArrow::new(position, position + force * scale)
            .with_color(0.0, 0.0, 1.0)
            .with_thickness(2.0)
            .with_scale(1.0);
        self.draw_arrow(display, frame, camera, &arrow);
    }

    pub fn draw_custom_arrow(&self, display: &Display<glium::glutin::surface::WindowSurface>, frame: &mut Frame, camera: &Camera, start: Vector, end: Vector, color: [f32; 3]) {
        let arrow = DebugArrow::new(start, end)
            .with_color(color[0], color[1], color[2])
            .with_thickness(1.5)
            .with_scale(1.0);
        self.draw_arrow(display, frame, camera, &arrow);
    }
}
