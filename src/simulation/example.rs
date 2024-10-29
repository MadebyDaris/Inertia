use std::f32::consts::PI;
use std::time::Instant;

use crate::{calculate_g_forces, update_astral_body_physics};
// Import necessary modules and structs
use crate::mesh::sphere::SphereConstructor;
use crate::mesh::ShaderData;
use crate::physics::physicsworld::*;
use crate::utils::eventhandler::handle_events;
use crate::utils::ui::{AstralBodyInfoWidget, MyUI, SimulationInfoWidget, Widget};
use crate::utils::{app::*, vector::Vector};
use super::world::*;
use super::super::render::*;

#[allow(unused_must_use)]
pub fn example() {
    // Initialize the basic app and camera settings
    let (
        display, 
        event_loop, 
        window) = Ogl::new();
    let event_loop = event_loop;
    // &window.set_cursor_visible(false);

    let mut last_frame = Instant::now();

    let mut _camera = Camera::new(&display);
    _camera.update();

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
        tex_filename: "./data/tex/earth.jpg".to_string(),
        vertex_shader: "data/glsl/vertex_shader.glsl".to_string(),
        fragment_shader: "data/glsl/fragment_shader.glsl".to_string(),
    };

    // Create Meshes for the spheres
    // Create AstralBody instances for the spheres
    let mut body_1 = sphere_constructor.sphere_physics_object(Vector(0.0, 0., 0.), 210.0, &display, earth_shader.clone());
    body_1.mesh.translate(0., 0., 0.);
    body_1.mesh.scale(2., 2., 2.);

    let mut body_2 = sphere_constructor.sphere_physics_object(Vector(0., 0., 5.), 1.0, &display, sphere_shaders.clone());
    body_2.mesh.translate(35., 0., 0.);

    let mut body_3 = sphere_constructor.sphere_physics_object(Vector(0., 0., 5.), 1.0, &display, sphere_shaders.clone());
    body_3.mesh.translate(60., 0., 0.);

    // Interface creation
    let mut ui_object = MyUI::new(&display, &window, &event_loop);


// 
//  RENDERING LOOP
//

    Ogl::update(event_loop, move |events| {
        let mut frame = display.draw();

//  
// SIMULTAION LOGIC HERE
//

        let now = Instant::now();
        let delta_time = now.duration_since(last_frame).as_secs_f32();
        last_frame = now;
        // Get the current screen dimensions
        let (width,height) = &display.get_framebuffer_dimensions();

        // Update geometries for each body
        let mut bodies = [&body_1, &body_2, &body_3].clone();
        calculate_g_forces!(body_1, &body_2, &body_3);
        update_astral_body_physics!(body_1, delta_time);

        calculate_g_forces!(body_2, &body_1, &body_3);
        update_astral_body_physics!(body_2, delta_time);

        calculate_g_forces!(body_3, &body_1, &body_2);
        update_astral_body_physics!(body_3, delta_time);


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

// 
// UI widgets
// 
        let astral_body_info: AstralBodyInfoWidget = body_1.get_widget("Planet 1".to_owned());
        let astral_body_info2: AstralBodyInfoWidget = body_2.get_widget("Smaller moon".to_owned());


        let simulation_info = SimulationInfoWidget {
            elapsed_time: delta_time,
        };

// 
// Render the world with current settings
// 

        let mut w: PhysicsWorld = PhysicsWorld::new(vec![&body_1, &body_2, &body_3], _camera, light);        
        w.render(&display, &mut frame, &camera_mat, light, (0.0,0.0,0.0,0.1));

        ui_object.render_ui(&window, &display, &mut frame, |egui_context| {
            &astral_body_info.show(egui_context);
            &astral_body_info2.show(egui_context);
            &simulation_info.show(egui_context);
        });

        frame.finish().unwrap();
        handle_events(events, &mut _camera, &mut camera_mat)
        }
    ).unwrap()
}