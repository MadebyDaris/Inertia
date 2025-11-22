use std::f32::consts::PI;

use glium::Surface;
use glium::glutin::display::GetGlDisplay;
use glium::glutin::display::GlDisplay as _;
use crate::mesh::axis_lines::AxisLines;
use crate::mesh::barrier::BarrierMesh;
use crate::mesh::grid_floor::GridFloor;
use crate::mesh::ShaderData;
use crate::physics::world::DiffuseLight;
use crate::render::scene_ui::SceneUIRenderer;
use crate::render::{Camera, CameraMat};
use crate::render::shader_system::ShaderManager;
use crate::simulation::quantum_simulation::quantum_sim::QuantumSimulation;
use crate::simulation::simulation::Simulation;
use crate::simulation::timeutil::SimulationTime;
use crate::ui::WidgetManager;
use crate::utils::app::Inertia;
use crate::utils::eventhandler::handle_events;
use crate::utils::vector::Vector;

#[allow(unused_must_use)]
pub fn example() {
    // Initialize application
    let (display, event_loop, window, gl_config) = Inertia::new();

    // Initialize shader manager
    let mut shader_manager = ShaderManager::new();
    shader_manager.initialize_shaders(&display);

    // Initialize OpenGL for compute shaders (must be done after creating display)
    gl::load_with(|ptr| {
        let c_str = std::ffi::CString::new(ptr).unwrap();
        gl_config.display().get_proc_address(&c_str)
    });
    
    let mut time = SimulationTime::new(0.01);
    time.set_time_acceleration(10.0);

    let mut camera = Camera::new(&display);
    camera.position = Vector(0.0, -10.0, 80.0);
    camera.yaw = -PI / 2.0;
    camera.pitch = -0.3;
    camera.update_look();
    camera.update();

    // Create quantum simulation with 256*256 grid
    let mut simulation = QuantumSimulation::new(&display, 256);

    // CURRENTLY BROKEN
    // simulation.setup_plane_wave(
    //     &display,
    //     3.0,
    //     0.0,
    //     0.3,
    // );
    
    simulation.setup_gaussian_wave_packet(
        &display,
        -3.5,
        0.0,
        0.5,
        30.0,
        0.0,
    );
    
    simulation.setup_double_slit(
        &display,
        0.0,
        0.1,
        0.4,
        1.0,
        50.0,
    );
    
    // Create visual barrier mesh
    let mut barrier_mesh = BarrierMesh::new(
        &display,
        &shader_manager,
        0.0,
        0.1,
        0.4,
        1.0,
        10.0,
    );

    barrier_mesh.transform = barrier_mesh.transform
            .scale(10.0, 1.0, 10.0)
            .translate(0.0, -10.0, 0.0);
    
    simulation.quantum_object.set_height_scale(20.0);
    simulation.quantum_object.transform = simulation.quantum_object.transform
        .scale(100., 1., 100.)
        .translate(0., -5., 0.);
    
    simulation.paused = false;
    time.paused = false;

    let _light = DiffuseLight {
        u_light_color: (1.0, 1.0, 1.0),
        u_light_direction: (0.5, -0.5, 0.5),
    };
    let mut widget_manager = WidgetManager::new(&display, &window, &event_loop);
    let _scene_ui_renderer = SceneUIRenderer::new(&display);

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

    let axis_lines = AxisLines::new(&display, &shader_manager, 50.0);

// 
//  RENDERING LOOP
//
    Inertia::update(event_loop, move |events| {
        // PHYSICS & COMPUTE STEP
        // Run raw OpenGL compute operations BEFORE creating the Glium frame
        // Or weird black artifact stuff

        time.update(1.5);

        if !time.paused {
            let num_substeps = 10;
            for _ in 0..num_substeps {
                simulation.evolve_wavefunction(&display, time.accelerated_delta_time / num_substeps as f32);
            }
        }
        // simulation.calculate_probability_density();
        // very very broken

        // RENDERING STEP
        let mut frame = display.draw();
        frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        let (width, height) = display.get_framebuffer_dimensions();

        camera.update();
        let mut camera_mat = CameraMat {
            view_mat: camera.view_matrix(),
            pers_mat: camera.get_perspective(width as f32 / height as f32, PI / 3.0, 1024.0, 0.1),
        };

        simulation.render(&display, &mut frame, camera_mat.view_mat.matrix, camera_mat.pers_mat.matrix);
        
        barrier_mesh.render(&display, &mut frame, camera_mat.view_mat.matrix, camera_mat.pers_mat.matrix);
        
        axis_lines.render(&display, &mut frame, camera_mat.view_mat.matrix, camera_mat.pers_mat.matrix);
        
        grid_mesh.render(&display, &mut frame, camera_mat.view_mat.matrix, camera_mat.pers_mat.matrix);

        let ui_responses = widget_manager.render_ui(&window, &display, &mut frame, |ctx| {
            egui::Window::new("Inertia Quantum Simulation")
                .default_pos([10.0, 10.0])
                .default_width(300.0)
                .show(ctx, |ui| {
                    ui.heading("Quantum Mechanics Simulation");
                    ui.separator();
                    
                    // Playback Controls
                    ui.horizontal(|ui| {
                        if ui.button(if time.paused { "▶ Play" } else { "⏸ Pause" }).clicked() {
                            time.toggle_pause();
                            simulation.paused = time.paused;
                        }
                        ui.label(format!("Time: {:.3}s", simulation.time));
                    });
                    
                    ui.separator();
                    
                    ui.label(format!("Grid: {}×{} points", simulation.grid_size().0, simulation.grid_size().1));
                    ui.label(format!("Domain: -5.0 to +5.0 units")); // Changing this will break shit
                    ui.label(format!("Time Scale: {:.2}×", time.accelerated_time_factor));
                    ui.label(format!("Frame Delta: {:.3}ms", time.accelerated_delta_time * 1000.0));
                    
                    ui.separator();
                    
                    ui.label("Wave Packet:");
                    ui.label(format!("  Position: x={:.2}", -3.5 + simulation.time * 20.0 * 0.01));
                    ui.label(format!("  Momentum: k={:.1}", 20.0));
                    ui.label(format!("  Width: σ ={:.2}", 0.5));
                    
                    ui.separator();
                    
                    ui.label("Visualization:");
                    let mut height_scale = simulation.quantum_object.height_scale;
                    if ui.add(egui::Slider::new(&mut height_scale, 0.1..=50.0)
                        .text("Height Scale")
                        .logarithmic(true)).changed() {
                        simulation.quantum_object.set_height_scale(height_scale);
                    }
                    
                    ui.separator();
                    
                    ui.label("Physics:");
                    ui.label(format!("  ℏ (hbar): {:.3}", simulation.params.hbar));
                    ui.label(format!("  Mass: {:.3}", simulation.params.mass));
                    ui.label(format!("  Δx: {:.5}", simulation.params.dx));
                    ui.label(format!("  Δt: {:.6}", simulation.params.dt));
                });
        });

        for response in ui_responses {
            simulation.handle_ui_response(response, &display, &shader_manager);
        }

        frame.finish().unwrap();
        handle_events(&window, events, &mut camera, &mut camera_mat, &mut time, &mut widget_manager)
    }).unwrap()
}
