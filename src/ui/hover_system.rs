use crate::{
    physics::physicsworld::PhysicsWorld,
    render::{camera::Camera, ray::Ray, CameraMat},
    simulation::orbital_simulation::GalaxySimulation,
    utils::vector::Vector,
};
pub struct HoverSystem {
    pub hovered_index: Option<usize>,
    mouse_x: f32,
    mouse_y: f32,
    pub debug_ray: Option<Ray>,
    pub show_debug_ray: bool,
}

impl HoverSystem {
    pub fn new() -> Self {
        Self {
            hovered_index: None,
            mouse_x: 0.0,
            mouse_y: 0.0,
            debug_ray: None,
            show_debug_ray: false,
        }
    }

    pub fn toggle_debug_ray(&mut self) {
        self.show_debug_ray = !self.show_debug_ray;
    }

    pub fn update_mouse_position(&mut self, x: f32, y: f32) {
        self.mouse_x = x;
        self.mouse_y = y;
    }

    pub fn shoot_debug_ray(&mut self, camera: &Camera, camera_mat: &CameraMat, width: u32, height: u32) {
        let ndc_x = (2.0 * self.mouse_x) / (width as f32) - 1.0;
        let ndc_y = 1.0 - (2.0 * self.mouse_y) / (height as f32);
        let ray = self.create_ray_from_screen(camera, camera_mat, ndc_x, ndc_y);
        self.debug_ray = Some(ray);
        self.show_debug_ray = true;
        
        println!("Debug Ray Shot!");
        println!("  Origin: ({:.2}, {:.2}, {:.2})", ray.origin.0, ray.origin.1, ray.origin.2);
        println!("  Direction: ({:.2}, {:.2}, {:.2})", ray.direction.0, ray.direction.1, ray.direction.2);
        println!("  Mouse: ({:.0}, {:.0})", self.mouse_x, self.mouse_y);
        println!("  NDC: ({:.2}, {:.2})", ndc_x, ndc_y);
    }

    pub fn update_hover(
        &mut self,
        camera: &Camera,
        camera_mat: &CameraMat,
        simulation: &mut GalaxySimulation,
        light: crate::physics::world::DiffuseLight,
        width: u32,
        height: u32,
    ) {
        let ndc_x = (2.0 * self.mouse_x) / (width as f32) - 1.0;
        let ndc_y = 1.0 - (2.0 * self.mouse_y) / (height as f32);

        let ray = self.create_ray_from_screen(camera, camera_mat, ndc_x, ndc_y);

        let object_refs = simulation.get_physics_object_refs();
        
        if self.mouse_x % 50.0 < 1.0 && self.mouse_y % 50.0 < 1.0 { // Only print occasionally
            for (i, obj) in object_refs.iter().enumerate() {
                let pos = obj.position();
                println!("Object {}: position = ({:.2}, {:.2}, {:.2}), radius = {:.2}", 
                    i, pos.0, pos.1, pos.2, obj.mesh().uniforms.transform.matrix[0][0]);
            }
            println!("Ray: origin = ({:.2}, {:.2}, {:.2}), direction = ({:.2}, {:.2}, {:.2})", 
                ray.origin.0, ray.origin.1, ray.origin.2, 
                ray.direction.0, ray.direction.1, ray.direction.2);
        }
        
        let world = PhysicsWorld::new(object_refs, *camera, light);

        self.hovered_index = world.cast_ray(ray).map(|(index, _distance)| index);

        for (i, ui_obj) in simulation.scene_ui_objects.iter_mut().enumerate() {
            ui_obj.set_selected(Some(i) == self.hovered_index);
        }
    }

    fn create_ray_from_screen(
        &self,
        camera: &Camera,
        camera_mat: &CameraMat,
        ndc_x: f32,
        ndc_y: f32,
    ) -> Ray {
        let view = camera_mat.view_mat.matrix;
        let proj = camera_mat.pers_mat.matrix;

        let clip = [ndc_x, ndc_y, -1.0, 1.0];

        let proj_inv = invert_projection_matrix(&proj);
        let mut eye = [0.0; 4];
        for i in 0..4 {
            for j in 0..4 {
                eye[i] += proj_inv[j][i] * clip[j]; // Note: j,i for column-major
            }
        }
        eye[2] = -1.0;
        eye[3] = 0.0;

        let view_inv = invert_view_matrix(&view);
        let mut world = [0.0; 4];
        for i in 0..4 {
            for j in 0..4 {
                world[i] += view_inv[j][i] * eye[j]; // Note: j,i for column-major
            }
        }

        let ray_direction = Vector(world[0], world[1], world[2]).normalized();
        Ray::new(camera.position, ray_direction)
    }

    pub fn get_hovered_name<'a>(&self, names: &'a [String]) -> Option<&'a String> {
        self.hovered_index.and_then(|idx| names.get(idx))
    }
}

fn invert_projection_matrix(m: &[[f32; 4]; 4]) -> [[f32; 4]; 4] {
    // For a perspective projection matrix:
    // [f/a  0   0   0]
    // [0    f   0   0]
    // [0    0   A   1]
    // [0    0   B   0]
    // where A = (far+near)/(far-near), B = -(2*far*near)/(far-near)
    
    let mut inv = [[0.0; 4]; 4];
    
    // Inverse is:
    // [a/f  0    0      0   ]
    // [0   1/f   0      0   ]
    // [0    0    0     1/B  ]
    // [0    0    1     A/B  ]
    
    if m[0][0].abs() > 1e-6 {
        inv[0][0] = 1.0 / m[0][0];
    }
    if m[1][1].abs() > 1e-6 {
        inv[1][1] = 1.0 / m[1][1];
    }
    
    inv[2][3] = 1.0;
    
    if m[3][2].abs() > 1e-6 {
        inv[3][2] = 1.0 / m[3][2];
        inv[3][3] = m[2][2] / m[3][2];
    }
    
    inv
}

fn invert_view_matrix(m: &[[f32; 4]; 4]) -> [[f32; 4]; 4] {
    // [rx  ry  rz  0]
    // [ux  uy  uz  0]
    // [fx  fy  fz  0]
    // [tx  ty  tz  1]
    
    let mut inv = [[0.0; 4]; 4];
    
    // Transpose the rotation part (3x3 upper-left) - column to row major swap
    for i in 0..3 {
        for j in 0..3 {
            inv[i][j] = m[j][i];
        }
    }
    
    // Inverse translation: -R^T * T
    inv[3][0] = -(m[3][0] * m[0][0] + m[3][1] * m[0][1] + m[3][2] * m[0][2]);
    inv[3][1] = -(m[3][0] * m[1][0] + m[3][1] * m[1][1] + m[3][2] * m[1][2]);
    inv[3][2] = -(m[3][0] * m[2][0] + m[3][1] * m[2][1] + m[3][2] * m[2][2]);
    inv[3][3] = 1.0;
    
    inv
}
