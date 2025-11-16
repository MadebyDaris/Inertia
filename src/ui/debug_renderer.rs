use egui::{Color32, Stroke};
use glium::{Display, Frame, Surface, glutin::surface::WindowSurface, implement_vertex};
use crate::render::CameraMat;
use crate::ui::{hover_system::HoverSystem, projection::project_to_screen};
use crate::utils::vector::Vector;

pub struct DebugRenderer;

impl DebugRenderer {

    /// Render debug ray visualization using OpenGL
    /// Draws a 3D line showing the trajectory of the raycast from camera through mouse cursor
    pub fn render_debug_ray(
        display: &Display<WindowSurface>,
        frame: &mut Frame,
        hover_system: &HoverSystem,
        camera_mat: &CameraMat,
    ) {
        if !hover_system.show_debug_ray {
            return;
        }

        if let Some(ray) = &hover_system.debug_ray {
            use glium::index::PrimitiveType;
            use glium::{uniform, IndexBuffer, VertexBuffer};

            #[derive(Copy, Clone)]
            struct Vertex {
                position: [f32; 3],
                color: [f32; 3],
            }
            implement_vertex!(Vertex, position, color);

            let ray_length = 500.0;
            let ray_end = ray.origin + ray.direction * ray_length;

            // Create vertices for the line
            let vertices = vec![
                Vertex { position: [ray.origin.0, ray.origin.1, ray.origin.2], color: [1.0, 0.0, 0.0] },
                Vertex { position: [ray_end.0, ray_end.1, ray_end.2], color: [1.0, 1.0, 0.0] },
            ];

            let vertex_buffer = VertexBuffer::new(display, &vertices).unwrap();
            let indices = IndexBuffer::new(display, PrimitiveType::LinesList, &[0u16, 1u16]).unwrap();

            // Simple shader for colored lines
            let vertex_shader_src = r#"
                #version 140
                in vec3 position;
                in vec3 color;
                out vec3 v_color;
                uniform mat4 view;
                uniform mat4 perspective;
                void main() {
                    v_color = color;
                    gl_Position = perspective * view * vec4(position, 1.0);
                }
            "#;

            let fragment_shader_src = r#"
                #version 140
                in vec3 v_color;
                out vec4 f_color;
                void main() {
                    f_color = vec4(v_color, 1.0);
                }
            "#;

            let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

            let uniforms = uniform! {
                view: camera_mat.view_mat.matrix,
                perspective: camera_mat.pers_mat.matrix,
            };

            let params = glium::DrawParameters {
                line_width: Some(3.0),
                depth: glium::Depth {
                    test: glium::DepthTest::IfLess,
                    write: true,
                    .. Default::default()
                },
                .. Default::default()
            };

            frame.draw(&vertex_buffer, &indices, &program, &uniforms, &params).unwrap();
        }
    }

    pub fn render_velocity_vectors(
        ctx: &egui::Context,
        simulation: &crate::simulation::orbital_simulation::GalaxySimulation,
        camera_mat: &CameraMat,
        width: u32,
        height: u32,
    ) {
        let painter = ctx.debug_painter();
        let scale = 0.3; // Visual scale for velocity vectors (reduced from 5.0)
        
        for body in &simulation.owned_objects {
            let pos = body.position();
            
            // Start arrow from surface of planet, not center
            let arrow_start = pos + body.velocity.normalized() * body.r;
            let vel_end = arrow_start + body.velocity * scale;
            
            if let (Some((x1, y1)), Some((x2, y2))) = (
                project_to_screen(arrow_start, camera_mat, width, height),
                project_to_screen(vel_end, camera_mat, width, height),
            ) {
                // Draw velocity arrow
                painter.arrow(
                    egui::pos2(x1, y1),
                    egui::vec2(x2 - x1, y2 - y1),
                    Stroke::new(2.0, Color32::from_rgb(0, 255, 150)),
                );
            }
        }
    }

    pub fn render_acceleration_vectors(
        ctx: &egui::Context,
        simulation: &crate::simulation::orbital_simulation::GalaxySimulation,
        camera_mat: &CameraMat,
        width: u32,
        height: u32,
    ) {
        let painter = ctx.debug_painter();
        let scale = 0.1; // Visual scale for acceleration vectors
        
        for body in &simulation.owned_objects {
            let pos = body.position();
            
            let mut accel = Vector(0.0, 0.0, 0.0);
            for other in &simulation.owned_objects {
                if std::ptr::eq(body, other) {
                    continue;
                }
                let diff = other.position() - pos;
                let dist_sq = diff.dot(diff).max(0.01);
                let force_mag = simulation.gravity_constant * other.mass / dist_sq;
                accel = accel + diff.normalized() * force_mag;
            }
            
            let arrow_start = pos + accel.normalized() * body.r;
            let accel_end = arrow_start + accel * scale;
            
            if let (Some((x1, y1)), Some((x2, y2))) = (
                project_to_screen(arrow_start, camera_mat, width, height),
                project_to_screen(accel_end, camera_mat, width, height),
            ) {
                painter.arrow(
                    egui::pos2(x1, y1),
                    egui::vec2(x2 - x1, y2 - y1),
                    Stroke::new(2.0, Color32::from_rgb(255, 100, 100)),
                );
            }
        }
    }

    pub fn render_orbital_trails(
        ctx: &egui::Context,
        simulation: &crate::simulation::orbital_simulation::GalaxySimulation,
        camera_mat: &CameraMat,
        width: u32,
        height: u32,
    ) {

        let painter = ctx.debug_painter();
        
        for (idx, body) in simulation.owned_objects.iter().enumerate() {
            if idx == 0 {
                continue;
            }
            
            let radius = body.position().magnitude();
            let steps = 64;
            let mut points = Vec::new();
            
            for i in 0..=steps {
                let angle = (i as f32) / (steps as f32) * std::f32::consts::PI * 2.0;
                let x = angle.cos() * radius;
                let z = angle.sin() * radius;
                let pos = Vector(x, 0.0, z);
                
                if let Some((sx, sy)) = project_to_screen(pos, camera_mat, width, height) {
                    points.push(egui::pos2(sx, sy));
                }
            }
            
            if points.len() > 1 {
                let color = crate::ui::PhysicsUI::get_body_color(idx);
                painter.add(egui::Shape::line(
                    points,
                    Stroke::new(1.0, Color32::from_rgb(color.r() / 2, color.g() / 2, color.b() / 2)),
                ));
            }
        }
    }
}
