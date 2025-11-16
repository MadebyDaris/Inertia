use crate::{physics::Force, utils::vector::Vector};

#[derive(Debug, Clone)]
pub struct ObjectCreationRequest {
    pub name: String,
    pub mass: f32,
    pub radius: f32,
    pub position: Vector,
    pub velocity: Vector,
    pub texture_path: String,
}

#[derive(Debug, Clone)]
pub enum TimeControlCommand {
    Play,
    Pause,
    SpeedUp(f32),
    SlowDown(f32),
    SetSpeed(f32), // Set absolute speed multiplier
    Reset,
    GoBackward(f32), // Time travel backward by seconds
}

#[derive(Debug, Clone)]
pub enum ForceCommand {
    AddForce { target_object: String, force: Force },
    RemoveForce { target_object: String, force_id: usize },
    ToggleDamping { target_object: String, enabled: bool },
    SetGravity(f32),
}

#[derive(Debug, Clone)]
pub enum VisualCommand {
    ToggleOrbitTrail { object_name: String, enabled: bool },
    ToggleTrails(bool),
    ClearTrails,
    ClearAllTrails,
    SetTrailLength(usize),
    ToggleVelocityVectors(bool),
    ToggleVelocityArrows(bool),
    ToggleAccelerationArrows(bool),
    ToggleForceVectors(bool),
}