use std::f32::consts::PI;

use crate::mesh::grid_floor::GridFloor;
use crate::mesh::ShaderData;
use crate::physics::physicsworld::PhysicsWorld;
use crate::physics::world::DiffuseLight;
use crate::render::scene_ui::SceneUIRenderer;
use crate::render::{Camera, CameraMat};
use crate::render::shader_system::ShaderManager;
use crate::simulation::orbital_simulation::GalaxySimulation;
use crate::simulation::simulation::Simulation;
use crate::simulation::timeutil::SimulationTime;
use crate::ui::{DebugRenderer, DebugVisualization, HoverSystem, NameplateRenderer, PhysicsUI, WidgetManager};
use crate::utils::app::Inertia;
use crate::utils::eventhandler::handle_events;
use crate::utils::vector::Vector;
#[allow(unused_must_use)]
pub fn example() {
    // Initialize application
    let (display, event_loop, window) = Inertia::new();

    // Initialize shader manager
    let mut shader_manager = ShaderManager::new();
    shader_manager.initialize_shaders(&display);
  let mut time = SimulationTime::new(0.016);
    time.set_time_acceleration(1.0);
    time.toggle_pause();
    let mut camera = Camera::new(&display);
    camera.position = Vector(0.0, 50.0, 80.0);
    camera.yaw = -PI / 2.0;
    camera.pitch = -0.3;
    camera.update_look();
    camera.update();

    let mut simulation = GalaxySimulation::new();
    simulation.create_solar_system(&display, &mut shader_manager);

    let light = DiffuseLight {
        u_light_color: (1.0, 1.0, 1.0),
        u_light_direction: (0.5, -0.5, 0.5),
    };
    let mut widget_manager = WidgetManager::new(&display, &window, &event_loop);
    let scene_ui_renderer = SceneUIRenderer::new(&display);
    let mut hover_system = HoverSystem::new();
    let mut debug_viz = DebugVisualization::default();

    let grid = GridFloor::new(500.0, 20, [0.15, 0.15, 0.15]);
    let grid_shader = ShaderData {
        tex_filename: "./data/tex/grid.png".to_string(),
        shader_type: crate::render::shader_system::ShaderType::Unlit,
        material: crate::render::shader_system::MaterialProperties {
            color: [1.0, 1.0, 1.0],
            ..Default::default()
        },
    };
    let grid_mesh = grid.create_mesh(&display, grid_shader, &shader_manager);

// 
//  RENDERING LOOP
//
    Inertia::update(event_loop, move |events| {
        let mut frame = display.draw();
        let (width, height) = display.get_framebuffer_dimensions();

        // Update mouse position
        for event in events {
            if let glium::winit::event::Event::WindowEvent { event, .. } = event {
                if let glium::winit::event::WindowEvent::CursorMoved { position, .. } = event {
                    hover_system.update_mouse_position(position.x as f32, position.y as f32);
                }
            }
        }

        simulation.update(time.accelerated_delta_time);

        camera.update();
        let mut camera_mat = CameraMat {
            view_mat: camera.view_matrix(),
            pers_mat: camera.get_perspective(width as f32 / height as f32, PI / 3.0, 1024.0, 0.1),
        };
        time.update(1.5); // Acceleration multiplier per key press

        hover_system.update_hover(&camera, &camera_mat, &mut simulation, light, width, height);

        let object_refs = simulation.get_physics_object_refs();
        let mut world = PhysicsWorld::new(object_refs, camera, light);
        world.render(&display, &mut frame, &camera_mat, light, (0.02, 0.02, 0.02, 1.0));

        if debug_viz.show_grid {
            grid_mesh.render(&display, &mut frame, camera_mat.view_mat.matrix, camera_mat.pers_mat.matrix);
        }
        
        DebugRenderer::render_debug_ray(&display, &mut frame, &hover_system, &camera_mat);
        
        for ui_object in &simulation.scene_ui_objects {
            scene_ui_renderer.render(&display, &mut frame, &camera, ui_object);
        }

        let ui_responses = widget_manager.render_ui(&window, &display, &mut frame, |ctx| {
            PhysicsUI::configure_theme(ctx);
            PhysicsUI::render_toolbar(ctx, &time, &hover_system, &simulation);
            let shoot_ray = PhysicsUI::render_control_panel(ctx, &time, &simulation, &hover_system, &mut debug_viz);
            NameplateRenderer::render(ctx, &simulation, &camera_mat, &hover_system, width, height);
            
            // Handle ray shooting button click
            if shoot_ray {
                hover_system.shoot_debug_ray(&camera, &camera_mat, width, height);
            }
            
            // Render egui debug visualizations
            if debug_viz.show_velocity_vectors {
                DebugRenderer::render_velocity_vectors(ctx, &simulation, &camera_mat, width, height);
            }
            
            if debug_viz.show_acceleration_vectors {
                DebugRenderer::render_acceleration_vectors(ctx, &simulation, &camera_mat, width, height);
            }
            
            if debug_viz.show_orbital_trails {
                DebugRenderer::render_orbital_trails(ctx, &simulation, &camera_mat, width, height);
            }
        });

        for response in ui_responses {
            simulation.handle_ui_response(response, &display, &shader_manager);
        }

        frame.finish().unwrap();
        handle_events(&window, events, &mut camera, &mut camera_mat, &mut time, &mut widget_manager)
    }).unwrap()
}
