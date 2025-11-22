use glium::{Display, glutin::surface::WindowSurface};
use crate::{
    mesh::MeshObject,
    physics::{physicsobject::PhysicsObject, position_euclidean, Force, EulerAngles},
    render::{shader_system::ShaderManager, ray::Ray},
    simulation::simulation::Simulation,
    ui::manager::WidgetResponse,
    utils::vector::Vector,
};

pub struct BasicObject {
    pub mesh: MeshObject,
    pub velocity: Vector,
    pub acceleration: Vector,
    pub angular_velocity: Vector,
    pub angular_acceleration: Vector,
    pub mass: f32,
    pub forces: Vec<Force>,
    pub torques: Vec<Vector>,
    pub euler_angles: EulerAngles,
    pub moment_of_inertia: f32,
}

impl BasicObject {
    pub fn new(mesh: MeshObject, mass: f32) -> Self {
        Self {
            mesh,
            velocity: Vector(0.0, 0.0, 0.0),
            acceleration: Vector(0.0, 0.0, 0.0),
            angular_velocity: Vector(0.0, 0.0, 0.0),
            angular_acceleration: Vector(0.0, 0.0, 0.0),
            mass,
            forces: Vec::new(),
            torques: Vec::new(),
            euler_angles: EulerAngles { pitch: 0.0, yaw: 0.0, roll: 0.0 },
            moment_of_inertia: 1.0,
        }
    }
}

impl PhysicsObject for BasicObject {
    type CollisionObject = ();
    type Mesh = MeshObject;
    type Velocity = Vector;
    type Acceleration = Vector;
    type AngularVelocity = Vector;
    type AngularAcceleration = Vector;
    type Mass = f32;
    type Forces = Vec<Force>;
    type Torques = Vec<Vector>;
    type MomentOfInertia = f32;
    type EulerAngles = EulerAngles;

    fn mesh(&self) -> &MeshObject {
        &self.mesh
    }

    fn mesh_mut(&mut self) -> &mut MeshObject {
        &mut self.mesh
    }

    fn position(&self) -> Vector {
        position_euclidean(&self.mesh)
    }

    fn velocity(&self) -> Vector {
        self.velocity
    }

    fn velocity_mut(&mut self) -> &mut Vector {
        &mut self.velocity
    }

    fn acceleration(&self) -> Vector {
        self.acceleration
    }

    fn acceleration_mut(&mut self) -> &mut Vector {
        &mut self.acceleration
    }

    fn angular_velocity(&self) -> Vector {
        self.angular_velocity
    }

    fn angular_velocity_mut(&mut self) -> &mut Vector {
        &mut self.angular_velocity
    }

    fn angular_acceleration(&self) -> Vector {
        self.angular_acceleration
    }

    fn angular_acceleration_mut(&mut self) -> &mut Vector {
        &mut self.angular_acceleration
    }

    fn euler_angles(&self) -> &EulerAngles {
        &self.euler_angles
    }

    fn euler_angles_mut(&mut self) -> &mut EulerAngles {
        &mut self.euler_angles
    }

    fn mass(&self) -> f32 {
        self.mass
    }

    fn forces(&self) -> &Vec<Force> {
        &self.forces
    }

    fn forces_mut(&mut self) -> &mut Vec<Force> {
        &mut self.forces
    }

    fn torques(&self) -> &Vec<Vector> {
        &self.torques
    }

    fn torques_mut(&mut self) -> &mut Vec<Vector> {
        &mut self.torques
    }

    fn moment_of_inertia(&self) -> f32 {
        self.moment_of_inertia
    }

    fn intersects(&self, _ray: &Ray) -> Option<f32> {
        None
    }
}

pub struct BasicSimulation {
    pub objects: Vec<BasicObject>,
    pub object_names: Vec<String>,
    pub paused: bool,
    pub gravity: Vector,
}

impl Simulation for BasicSimulation {
    type Object = BasicObject;

    fn new() -> Self {
        Self {
            objects: Vec::new(),
            object_names: Vec::new(),
            paused: false,
            gravity: Vector(0.0, -9.81, 0.0),
        }
    }

    fn update(&mut self, delta_time: f32) {
        if self.paused {
            return;
        }

        for object in &mut self.objects {
            object.acceleration = self.gravity;
            object.velocity = object.velocity + object.acceleration * delta_time;
            
            let displacement = object.velocity * delta_time;
            object.mesh.translate(displacement.0, displacement.1, displacement.2);
        }
    }

    fn add_object(&mut self, object: Self::Object, name: String) {
        self.objects.push(object);
        self.object_names.push(name);
    }

    fn remove_object(&mut self, name: &str) -> Option<Self::Object> {
        if let Some(index) = self.object_names.iter().position(|n| n == name) {
            self.object_names.remove(index);
            Some(self.objects.remove(index))
        } else {
            None
        }
    }

    fn get_object(&self, name: &str) -> Option<&Self::Object> {
        self.object_names
            .iter()
            .position(|n| n == name)
            .map(|index| &self.objects[index])
    }

    fn get_object_mut(&mut self, name: &str) -> Option<&mut Self::Object> {
        if let Some(index) = self.object_names.iter().position(|n| n == name) {
            Some(&mut self.objects[index])
        } else {
            None
        }
    }

    fn get_objects(&self) -> &Vec<Self::Object> {
        &self.objects
    }

    fn get_object_names(&self) -> &[String] {
        &self.object_names
    }

    fn handle_ui_response(&mut self, _response: WidgetResponse, _display: &Display<WindowSurface>, _shader_manager: &ShaderManager) {}

    fn is_paused(&self) -> bool {
        self.paused
    }

    fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

    fn reset(&mut self) {
        self.objects.clear();
        self.object_names.clear();
        self.paused = false;
    }
}
