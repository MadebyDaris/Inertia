use std::f32::consts::PI;

use crate::simulation::simulation::Simulation;
use crate::physics::world::DiffuseLight;
use crate::simulation::timeutil::SimulationTime;
use crate::{calculate_g_forces, update_astral_body_physics, create_widgets};
// Import necessary modules and structs
use crate::mesh::sphere::SphereConstructor;
use crate::mesh::ShaderData;
use crate::physics::{physicsobject, physicsworld::*};
use crate::utils::eventhandler::handle_events;
use crate::ui::{AstralBodyInfoWidget, SimulationInfoWidget, Widget, WidgetManager};
use crate::utils::{app::*, vector::Vector};
use crate::render::*;

#[allow(unused_must_use)]
pub fn example() {
    // Initialize the basic app and camera settings
    let (
        display, 
        event_loop, 
        window
    ) = Inertia::new();
    let event_loop = event_loop;
    // &window.set_cursor_visible(false);

    let mut time = SimulationTime::new(0.016);
    time.set_time_acceleration(1.01);

    let mut _camera = Camera::new(&display);
    _camera.update();

    let mut simulation = Simulation::new();

    // Create a diffuse light source with specified color and direction
    let light = DiffuseLight { u_light_color:(1.,1.,1.), u_light_direction:(1.,0.2, 1.)};
    
    // Initialize a sphere constructor with radius, longitude, and latitude for sphere resolution
    let sphere_constructor = SphereConstructor {radius: 2., longitude:32, latitude: 16};

    // Set up shader data, specifying texture and shader file paths
    let sphere_shaders = ShaderData {
        tex_filename: "./data/tex/2k_mercury.jpg".to_string(),
        vertex_shader: "data/glsl/vertex_shader.glsl".to_string(),
        fragment_shader: "data/glsl/fragment_shader.glsl".to_string(),
    };
    
    let earth_shader = ShaderData {
        tex_filename: "./data/tex/mars.jpg".to_string(),
        vertex_shader: "data/glsl/vertex_shader.glsl".to_string(),
        fragment_shader: "data/glsl/fragment_shader.glsl".to_string(),
    };

    // Create Meshes for the spheres
    // Create AstralBody instances for the spheres
    let mut body_1: physicsobject::AstralBody = sphere_constructor.sphere_physics_object(Vector(0.0, 0., 0.), 210.0, &display, earth_shader.clone());
    let mut body_2: physicsobject::AstralBody = sphere_constructor.sphere_physics_object(Vector(0., 0., 5.), 1.0, &display, sphere_shaders.clone());
    let mut body_3: physicsobject::AstralBody = sphere_constructor.sphere_physics_object(Vector(0., 0., 5.), 1.0, &display, sphere_shaders.clone());
    let mut widget_manager: WidgetManager = WidgetManager::new(&display, &window, &event_loop);

    {
    body_1.mesh.translate(0., 0., 0.);
    body_1.mesh.scale(2., 2., 2.);
    simulation.add_object(body_1, "Planet 1".to_string());

    body_2.mesh.translate(35., 0., 0.);
    simulation.add_object(body_2, "Smaller moon".to_string());

    body_3.mesh.translate(60., 0., 0.);
    body_3.mesh.rotate(30., 20., 0.);
    // Interface creation
    simulation.add_object(body_3, "Moon 2".to_string());
    }

// 
//  RENDERING LOOP
//


    Inertia::update(event_loop, move |events| {
        let mut frame = display.draw();
        let (width,height) = &display.get_framebuffer_dimensions();


    //  
    // SIMULTAION LOGIC HERE
    //

        // ADDING SIMULATION PHYSICS
        if !simulation.paused {
            for i in 0..simulation.owned_objects.len() {
                // Apply damping
                let damping = simulation.owned_objects[i].damping_force(0.01);
                simulation.owned_objects[i].add_force(damping);

                // Calculate gravitational forces from other objects
                for j in 0..simulation.owned_objects.len() {
                    if i != j {
                        let other_position = simulation.owned_objects[j].position();
                        let other_mass = simulation.owned_objects[j].mass;
                        
                        // Create a temporary reference to avoid borrow checker issues
                        let g_force = simulation.owned_objects[i].universal_gravitation_force(&simulation.owned_objects[j], simulation.gravity_constant);
                        simulation.owned_objects[i].add_force(g_force);
                        
                        let torque = simulation.owned_objects[i].calculate_torque(g_force, other_position);
                        simulation.owned_objects[i].torques.push(torque);
                    }
                }

                // Apply physics updates
                simulation.owned_objects[i].law_of_momentum();
                simulation.owned_objects[i].update_velocity(time.accelerated_delta_time);
                simulation.owned_objects[i].update_geometry(time.accelerated_delta_time);
                simulation.owned_objects[i].update_orientation(time.accelerated_delta_time);
            }

        // Handle collisions
        for i in 0..simulation.owned_objects.len() {
            for j in (i + 1)..simulation.owned_objects.len() {
                if physicsobject::AstralBody::detect_collision(&simulation.owned_objects[i], &simulation.owned_objects[j]) {
                    // Split borrowing to avoid conflicts
                    let (left, right) = simulation.owned_objects.split_at_mut(j);
                    physicsobject::AstralBody::handle_collision(&mut left[i], &mut right[0]);
                }
            }
        }

        // Update trails
        simulation.update_trails();
        }

    //Camera isn't a part of the simulation
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


        let object_refs = simulation.create_physics_world();
        let mut world = PhysicsWorld::new(object_refs, _camera, light);       
        world.render(&display, &mut frame, &camera_mat, light, (0.0,0.0,0.0,0.1));

        
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

// 
// 
// 
        frame.finish().unwrap();
        handle_events(&window, events, &mut _camera, &mut camera_mat, &mut time, &mut widget_manager)
        }
    ).unwrap()
}