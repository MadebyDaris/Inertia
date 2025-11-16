use glium::{glutin::surface::WindowSurface, Display};

use crate::{
    mesh::{sphere::SphereConstructor, mesh_object::Mesh, MeshObject, MeshUniforms, ShaderData}, 
    physics::{physicsobject::PhysicsObject, position_euclidean, EulerAngles, Force}, 
    render::{ray::Ray, shader_system::ShaderManager},
    ui::AstralBodyInfoWidget, 
    utils::{matrix::TransformMatrix, vector::Vector}
};


pub struct AstralCollisionObject {

}
pub trait CollisionObject {
    fn handle_collision(body1: &mut AstralBody, body2: &mut AstralBody) {
    }
    fn handle_angular_impulse(body1: &mut AstralBody, body2: &mut AstralBody, collision_point: Vector){
    }
}

pub type AstralPhysicsObject = dyn PhysicsObject<
    Mesh = MeshObject,
    Velocity = Vector,
    Acceleration = Vector,
    AngularVelocity = Vector,
    AngularAcceleration = Vector,
    Mass = f32,
    Forces = Vec<Force>,
    Torques = Vec<Vector>,
    MomentOfInertia = f32,
    EulerAngles = EulerAngles,
    CollisionObject = AstralCollisionObject,
>;

pub struct AstralBody {
    pub mesh: MeshObject,
    pub velocity: Vector,
    pub acceleration: Vector,
    pub angular_velocity: Vector,
    pub angular_acceleration: Vector,
    pub mass: f32,
    pub r: f32,
    pub forces: Vec<Force>,
    pub torques: Vec<Vector>,
    pub moment_of_inertia: f32,
    pub euler_angles: EulerAngles
}
impl PartialEq for AstralBody {
    fn eq(&self, other: &Self) -> bool {
        self.mass == other.mass && 
        self.velocity == other.velocity && 
        self.acceleration == other.acceleration
    }
}
impl PhysicsObject for AstralBody {
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
    type CollisionObject = AstralCollisionObject;

    fn mesh(&self) -> &Self::Mesh {
        &self.mesh
    }

    fn position(&self) -> Vector {
        position_euclidean(&self.mesh)
    }

    fn mass(&self) -> f32 {
        self.mass
    }

    fn moment_of_inertia(&self) -> f32 {
        self.moment_of_inertia
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

    fn mesh_mut(&mut self) -> &mut Self::Mesh {
        &mut self.mesh
    }

    fn intersects(&self, ray: &Ray) -> Option<f32> {
        let oc = ray.origin - self.position();
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - self.r * self.r;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            None
        } else {
            Some((-b - discriminant.sqrt()) / (2.0 * a))
        }
    }
}

impl AstralBody {
    pub fn detect_collision(body1: &AstralBody, body2: &AstralBody) -> bool {
        let distance = (body1.position() - body2.position()).magnitude();
        distance <= (body1.r + body2.r)
    }

    pub fn handle_collision(body1: &mut AstralBody, body2: &mut AstralBody) {
        let m1 = body1.mass;
        let m2 = body2.mass;

        let v1 = body1.velocity;
        let v2 = body2.velocity;

        let collision_normal = (body2.position() - body1.position()).normalized();
        let v1_normal = Vector::dot(v1, collision_normal);
        let v2_normal = Vector::dot(v2, collision_normal);

        let v1_normal_new = (v1_normal * (m1 - m2) + 2.0 * m2 * v2_normal) / (m1 + m2);
        let v2_normal_new = (v2_normal * (m2 - m1) + 2.0 * m1 * v1_normal) / (m1 + m2);

        body1.velocity += collision_normal * (v1_normal_new - v1_normal);
        body2.velocity += collision_normal * (v2_normal_new - v2_normal);
    }

    pub fn handle_angular_impulse(body1: &mut AstralBody, body2: &mut AstralBody, collision_point: Vector) {
        let r1 = collision_point - body1.position();
        let r2 = collision_point - body2.position();

        let impulse = (body2.velocity - body1.velocity).magnitude();
        let torque1 = Vector::cross(r1, Vector(impulse, 0., 0.));
        let torque2 = Vector::cross(r2, Vector(impulse, 0., 0.));

        body1.angular_velocity += torque1 / body1.moment_of_inertia;
        body2.angular_velocity += torque2 / body2.moment_of_inertia;
    }
}

impl AstralBody{
    pub fn universal_gravitation_force(&self, body: &AstralBody, g: f32) -> Force {
        let direction = position_euclidean(&body.mesh) - position_euclidean(&self.mesh);
        let distance_squared = direction.magnitude().powi(2);

        let mu: f32 = g * body.mass * self.mass.clone();
        if direction.magnitude() == 0.0 {
            return Force { direction: Vector(0.0, 0.0, 0.0), magnitude: 0.}; // Avoid division by zero
        }
        return Force{direction: direction.normalized(), magnitude: mu / distance_squared}
    }
    pub fn damping_force(&self, damping_coefficient: f32) -> Force {
        // Calculate damping force
        let damping_vector = self.velocity * -damping_coefficient; // Opposing direction to the velocity
        Force {
            direction: damping_vector.normalized(), // Normalize the direction
            magnitude: damping_vector.magnitude(),   // Use the magnitude of the damping force
        }
    }
    pub fn position(&self) -> Vector {
        return position_euclidean(&self.mesh)
    }
    pub fn add_force(&mut self, force: Force) {
        self.forces.push(force);
    }

    pub fn update_geometry(&mut self, delta_time: f32) {
        self.mesh.translate(self.velocity.0 * delta_time, self.velocity.1 * delta_time, self.velocity.2 * delta_time);
        self.mesh.rotate(self.euler_angles.pitch, self.euler_angles.yaw, self.euler_angles.roll);
    }
}

//
// Sphere factory
//
impl SphereConstructor {
    pub fn sphere_physics_object(&self, velocity: Vector, mass: f32, screen: &Display<WindowSurface>, shader_data: ShaderData, shader_manager: &ShaderManager) -> AstralBody{
        let (data, indices) = self.new();
        let mesh = MeshObject {
            data: Mesh::new(screen, &data.verts, shader_data.clone(), shader_manager),
            uniforms: MeshUniforms { 
                transform: TransformMatrix::identity(), 
                indices,
                material: shader_data.material,
                shader_type: shader_data.shader_type,
            },
        };
        let moment_of_inertia = (2.0 / 5.0) * mass * self.radius.powi(2);
        return AstralBody { 
            mesh, 
            velocity, 
            acceleration: Vector(0.,0.,0.,),
            angular_acceleration: Vector(0.,0.,0.,), 
            mass, 
            r: self.radius, 
            forces: Vec::new(),
            torques: Vec::new(), 
            angular_velocity: Vector(0.,0.,0.,), 
            moment_of_inertia,
            euler_angles: EulerAngles { pitch: 0., yaw: 0., roll: 0. },
        };
    }
}


// 
// SOME FUNCTIONS FOR THE UI
// 
impl AstralBody {
    pub fn get_widget(&self, name: &String)-> AstralBodyInfoWidget {
        return AstralBodyInfoWidget {
            name: name.to_string(),
            mass: self.mass,
            position: self.position_str(),
            velocity: self.velocity_str(),
            angular_velocity: format!(
                "({}, {}, {}) rad/s",
                self.angular_velocity.0,
                self.angular_velocity.1,
                self.angular_velocity.2
            ),
            acc: format!(
                "({}, {}, {}) m/s²",
                self.acceleration.0,
                self.acceleration.1,
                self.acceleration.2
            ),
            moment_of_inertia: self.moment_of_inertia,
            euler_angles: format!(
                "Pitch: {:.2}°, Yaw: {:.2}°, Roll: {:.2}°",
                self.euler_angles.pitch.to_degrees(),
                self.euler_angles.yaw.to_degrees(),
                self.euler_angles.roll.to_degrees()
            ),
            radius: self.r,
        }
    }
    
    // Method to get a formatted string of position
    pub fn position_str(&self) -> String {
        format!("({}, {}, {})", self.position().0, self.position().1, self.position().2)
    }

    // Method to get a formatted string of velocity
    pub fn velocity_str(&self) -> String {
        format!("({}, {}, {})", self.velocity.0, self.velocity.1, self.velocity.2)
    }
}
