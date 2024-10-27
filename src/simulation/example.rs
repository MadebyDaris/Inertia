use std::f32::consts::PI;

use std::time::Instant;

use crate::mesh::sphere::SphereConstructor;
use crate::mesh::ShaderData;
use crate::physics::physicsworld::PhysicsWorld;
use crate::physics::Vector;
use super::{super::utils::app::*, world::*};
use super::super::render::*;
use glium::winit::event::{ElementState, KeyEvent};
use glium::winit::event::WindowEvent;
use glium::winit::keyboard::{Key, KeyCode};
use glium::winit::{self, event::{self}, window::Fullscreen};

const G: f32 = (5) as f32; 

pub fn example() {
    // Initialize the basic app and camera settings
    let app = App::new();
    let mut last_frame = Instant::now();

    app.window.set_cursor_visible(false);
    app.window.set_fullscreen(Some(Fullscreen::Borderless(None)));
    let mut _camera = Camera::new(&app.display);
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
    
    // Create Meshes for the spheres
    // Create AstralBody instances for the spheres
    let mut body_1 = sphere_constructor.sphere_physics_object(Vector(0.0, 0., 0.), 210.0, &app.display, sphere_shaders.clone());
    body_1.mesh.translate(0., 0., 0.);
    body_1.mesh.scale(2., 2., 2.);

    let mut body_2 = sphere_constructor.sphere_physics_object(Vector(0., 0., 5.), 1.0, &app.display, sphere_shaders.clone());
    body_2.mesh.translate(35., 0., 0.);

    let mut body_3 = sphere_constructor.sphere_physics_object(Vector(0., 0., 5.), 1.0, &app.display, sphere_shaders.clone());
    body_3.mesh.translate(60., 0., 0.);


// 
//  RENDERING LOOP
//
    App::update(app.event_loop, move |events| {

// 
// SIMULTAION LOGIC HERE
// 
        let now = Instant::now();
        let delta_time = now.duration_since(last_frame).as_secs_f32();
        last_frame = now;
        // Get the current screen dimensions
        let (width,height) = app.display.get_framebuffer_dimensions();

        let force_1_2 = body_1.universal_gravitation_force(&mut body_2, G);
        let force_1_3 = body_1.universal_gravitation_force(&mut body_3, G);
        let force_2_1 = body_2.universal_gravitation_force(&mut body_1, G); // Equal and opposite
        let force_2_3 = body_2.universal_gravitation_force(&mut body_3, G);
        let force_3_1 = body_3.universal_gravitation_force(&mut body_1, G); // Equal and opposite
        let force_3_2 = body_3.universal_gravitation_force(&mut body_2, G);

            // Update velocities based on calculated forces
        body_1.acc = (force_1_2 + force_1_3) / body_1.mass;
        body_2.acc = (force_2_1 + force_2_3) / body_2.mass;
        body_3.acc = (force_3_1 + force_3_2) / body_3.mass;


        // Update geometries for each body
        for body in [&mut body_1, &mut body_2, &mut body_3] {
            body.update_velocity(delta_time);
            body.update_geometry(delta_time);
        }

        // Set up the camera matrices for view and perspective, with a 60-degree field of view (PI / 3.0)
        let mut camera_mat: CameraMat = CameraMat{ 
                view_mat: _camera.view_matrix(), 
                pers_mat: _camera.get_perspective(
                    width as f32 / height as f32, 
                    PI/3.0, 
                    1024.,
                    0.1) 
        };
        _camera.update();

// 
// 
// 
        // Render the world with current settings
        let mut w: PhysicsWorld = PhysicsWorld::new(vec![&body_1, &body_2, &body_3], _camera, light);        
        w.render(&app.display, &camera_mat, light, (0.0,0.0,0.0,0.1));
    
//
// EVENT HANDLER
// 
        let mut action = Action::Continue;
        for event in events {
            match event {
                // Handle device events (e.g., mouse, keyboard) and update camera view
                event::Event::DeviceEvent { event, .. } => {
                    _camera.look_at(&event); // Handle Device Events
                    _camera.update();

                }
                // Handle window events
                event::Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::Resized(size) => {
                            // Update the projection matrix only
                            camera_mat.pers_mat = _camera.get_perspective(
                                size.width as f32 / size.height as f32,
                                45.0,
                                0.1,
                                100.0);
                        }
                        WindowEvent::CloseRequested => {
                            action = Action::Stop; // Stop the application
                        }
                        WindowEvent::KeyboardInput { event, .. } => {
                            _camera.input(event); // Pass keyboard input to camera
                            if event.state == ElementState::Pressed 
                               && event.logical_key == glium::winit::keyboard::Key::Named(glium::winit::keyboard::NamedKey::Escape) {
                                action = Action::Stop; // Stop on Escape key press
                            }
                        }
                        _ => {
                        }
                    }
                }
                _ => (),
            }
        }
        action}
    ).unwrap()
}