use crate::{
    render::scene_ui::{outline::Outline, trail::Trail, vector_arrow::VectorArrow},
    utils::vector::Vector,
};

#[derive(Clone, Debug)]
pub struct SceneUIObject {
    pub position: Vector,
    pub radius: f32,
    pub velocity_arrow: VectorArrow,
    pub acceleration_arrow: VectorArrow,
    pub custom_arrows: Vec<VectorArrow>,
    pub trail: Trail,
    pub outline: Outline,
}

impl SceneUIObject {
    pub fn new(position: Vector) -> Self {
        let r = (rand::random::<f32>() * 0.5 + 0.5).min(1.0);
        let g = (rand::random::<f32>() * 0.5 + 0.5).min(1.0);
        let b = (rand::random::<f32>() * 0.5 + 0.5).min(1.0);
        
        Self {
            position,
            radius: 1.0,
            velocity_arrow: VectorArrow::velocity(),
            acceleration_arrow: VectorArrow::acceleration(),
            custom_arrows: Vec::new(),
            trail: Trail::new(0, [r, g, b]),
            outline: Outline::new(),
        }
    }

    pub fn update(&mut self, position: Vector, velocity: Vector, radius: f32) {
        self.position = position;
        self.radius = radius;
        self.velocity_arrow.offset = Vector(0.0, 0.0, 0.0);
        self.velocity_arrow.vector = velocity;
        
        if self.trail.enabled {
            self.trail.add_point(position);
        }
    }

    pub fn set_acceleration(&mut self, acceleration: Vector) {
        self.acceleration_arrow.vector = acceleration;
    }

    pub fn enable_velocity_arrow(&mut self, enabled: bool) {
        self.velocity_arrow.enabled = enabled;
    }

    pub fn enable_acceleration_arrow(&mut self, enabled: bool) {
        self.acceleration_arrow.enabled = enabled;
    }

    pub fn enable_trail(&mut self, enabled: bool) {
        self.trail.enabled = enabled;
    }

    pub fn enable_outline(&mut self, enabled: bool) {
        self.outline.enabled = enabled;
    }

    pub fn set_selected(&mut self, selected: bool) {
        if selected {
            self.outline = Outline::selected();
        } else {
            self.outline.enabled = false;
        }
    }

    pub fn point_velocity_arrow_toward(&mut self, target_position: Vector) {
        let direction = (target_position - self.position).normalized();
        self.velocity_arrow.offset = direction * self.radius;
    }

    pub fn add_custom_arrow(&mut self, direction: Vector, color: [f32; 3], scale: f32) {
        let mut arrow = VectorArrow::custom(color);
        arrow.vector = direction;
        arrow.scale = scale;
        self.custom_arrows.push(arrow);
    }

    pub fn clear_custom_arrows(&mut self) {
        self.custom_arrows.clear();
    }
}
