use std::f32::consts::PI;

// Ignore ts

use crate::mesh::grid_floor::GridFloor;
use crate::simulation::orbital_simulation::GalaxySimulation;
use crate::simulation::simulation::Simulation;
use crate::physics::world::DiffuseLight;
use crate::simulation::timeutil::SimulationTime;
use crate::create_widgets;
// Import necessary modules and structs
use crate::mesh::ShaderData;
use crate::physics::physicsworld::*;
use crate::utils::eventhandler::handle_events;
use crate::ui::{AstralBodyInfoWidget, SimulationInfoWidget, Widget, WidgetManager};
use crate::utils::app::*;
use crate::render::*;

#[allow(unused_must_use)]
pub fn example() {
    // Initialize the basic app and camera settings
    let (
        display, 
        event_loop, 
        window
        ) = Inertia::new();

    let mut time = SimulationTime::new(0.016);
    time.set_time_acceleration(1.01);

    let mut _camera = Camera::new(&display);
    _camera.update();

    let mut simulation = GalaxySimulation::new();

    simulation.create_solar_system(&display);

    // Create a diffuse light source with specified color and direction
    let light = DiffuseLight { 
        u_light_color:(1.,1.,1.), 
        u_light_direction:(1.,0.2, 1.)
    };
    
    let mut widget_manager = WidgetManager::new(&display, &window, &event_loop);
    
    // Create grid floor
    let grid = GridFloor::new(500.0, 50, [0.3, 0.3, 0.3]);
    let grid_shader = ShaderData {
        tex_filename: "./data/tex/grid.png".to_string(), // You'll need a grid texture
        vertex_shader: "data/glsl/grid_vertex_shader.glsl".to_string(),
        fragment_shader: "data/glsl/grid_fragment_shader.glsl".to_string(),
    };
    let grid_mesh = grid.create_mesh(&display, grid_shader);


// 
//  RENDERING LOOP
//


    Inertia::update(event_loop, move |events| {
        let mut frame = display.draw();
        let (width,height) = &display.get_framebuffer_dimensions();

        simulation.update(time.accelerated_delta_time);

        // Set up the camera matrices for view and perspective, with a 60-degree field of view (PI / 3.0)
        let mut camera_mat: CameraMat = CameraMat{ 
                view_mat: _camera.view_matrix(), 
                pers_mat: _camera.get_perspective(
                    *width as f32 / *height as f32, 
                    PI/3.0, 
                    1024.,
                    0.1) 
        };
        _camera.update();
        time.update(1.002);


// 
// Render the world with current settings
// 


        let object_refs = simulation.get_physics_object_refs();
        let mut world = PhysicsWorld::new(object_refs, _camera, light);       
        world.render(&display, &mut frame, &camera_mat, light, (0.0,0.0,0.0,0.1));

        grid_mesh.render(&display, &mut frame, camera_mat.view_mat.matrix, camera_mat.pers_mat.matrix);
// 
// UI widgets
// 
        let astral_body_widgets : Vec<AstralBodyInfoWidget> = create_widgets!(simulation);

        let simulation_info = SimulationInfoWidget::new(&simulation, time);
        let ui_responses = widget_manager.render_ui(&window, &display, &mut frame, |egui_context, ui| {
            for w in &astral_body_widgets {
                w.show_widget_in_ui(egui_context, ui);
            }
            simulation_info.show_widget_in_ui(egui_context, ui);
        });
        for response in ui_responses {
            simulation.handle_ui_response(response, &display);
        }
        frame.finish().unwrap();
        handle_events(&window, events, &mut _camera, &mut camera_mat, &mut time, &mut widget_manager)
        }
    ).unwrap()
}