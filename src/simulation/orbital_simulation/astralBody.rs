use glium::{glutin::surface::WindowSurface, Display};

use crate::{
    mesh::{sphere::SphereConstructor, Mesh, MeshObject, MeshUniforms, ShaderData}, 
    physics::{physicsobject::PhysicsObject, position_euclidean, EulerAngles, Force}, 
    ui::AstralBodyInfoWidget, utils::{matrix::TransformMatrix, vector::Vector}
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


    // Update velocity based on current acceleration and delta time
    fn update_velocity(&mut self, delta_time: f32) {
        // Update velocity using the formula: v = v0 + a * t
        self.velocity += self.acceleration * delta_time;

        // Update angular velocity: ω = ω0 + α * t
        self.angular_velocity += self.angular_acceleration * delta_time;
    }
    fn update_geometry(&mut self, delta_time: f32) {
        self.mesh.translate(self.velocity.0*delta_time, self.velocity.1*delta_time, self.velocity.2*delta_time);
        
        // Update orientation by rotating based on current Euler angles (pitch, yaw, roll)
        self.mesh.rotate(self.euler_angles.pitch, self.euler_angles.yaw, self.euler_angles.roll);
    }

    // Update acceleration based on total forces acting on the body
    fn law_of_momentum(&mut self) {
        let mut resultant_force = Vector(0.,0.,0.);
        for force in &self.forces {
            resultant_force += force.direction * force.magnitude
        }
        if self.mass != 0.0 { // Avoid division by zero
            self.acceleration = resultant_force / self.mass;
        } else {
            self.acceleration = Vector(0.0, 0.0, 0.0); // No acceleration if mass is zero
        }
        let total_torque = self.total_torque();
        self.angular_acceleration = total_torque / self.moment_of_inertia;
    
        self.torques.clear(); // Clear torques for next cycle
        self.forces.clear(); // Reset forces to only apply new forces in the next cycle
    }

    // 
    // angular rotation
    // 
    // Calculate total torque from the list of torques
    fn total_torque(&self) -> Vector {
        return self.torques.iter().fold(Vector(0., 0., 0.), |acc, t| acc + *t); // Sum up torques
    }

    // Calculate torque based on applied force and point of application
    fn calculate_torque(&mut self, force: Force, point: Vector) -> Vector {
        let direction = position_euclidean(&self.mesh) - point; // Direction vector from point to the body
        return Vector::cross(direction, force.direction * force.magnitude); // Cross product to get torque
    }

    // Update angular acceleration based on applied torque and moment of inertia
    fn update_angular_acceleration(&mut self, torque: Vector, moment_of_inertia: f32) {
        self.angular_acceleration = torque / moment_of_inertia; // Divide by moment of inertia to get angular acceleration
    }

    // Update the orientation of the body based on angular velocity
    fn update_orientation(&mut self, delta_time: f32) {
        self.euler_angles.pitch += self.angular_velocity.0 * delta_time; // Update pitch
        self.euler_angles.yaw += self.angular_velocity.1 * delta_time; // Update yaw
        self.euler_angles.roll += self.angular_velocity.2 * delta_time; // Update roll

        // Wrap angles to keep them in the range [0, 2*PI] if necessary
        self.euler_angles.pitch %= std::f32::consts::TAU;
        self.euler_angles.yaw %= std::f32::consts::TAU;
        self.euler_angles.roll %= std::f32::consts::TAU;
    }
    
    fn mesh(&self) -> &Self::Mesh {
        &self.mesh
    }
    
    fn position(&self) -> Vector {
        position_euclidean(&self.mesh)
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
}

//
// Sphere factory
//
impl SphereConstructor {
    pub fn sphere_physics_object(&self, velocity: Vector, mass: f32, screen: &Display<WindowSurface>, shader_data: ShaderData) -> AstralBody{
        let (data, indices) = self.new();
        let mesh = MeshObject {
            data: Mesh::new(screen, &data.verts, shader_data),
            uniforms: MeshUniforms { transform: TransformMatrix::identity(), indices },};
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