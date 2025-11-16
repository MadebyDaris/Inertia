use crate::utils::vector::Vector;

#[derive(Clone, Debug)]
pub struct Trail {
    pub positions: Vec<Vector>,
    pub color: [f32; 3],
    pub max_length: usize,
    pub enabled: bool,
}

impl Trail {
    pub fn new(max_length: usize, color: [f32; 3]) -> Self {
        Self {
            positions: Vec::new(),
            color,
            max_length,
            enabled: true,
        }
    }

    pub fn add_point(&mut self, position: Vector) {
        self.positions.push(position);
        if self.max_length > 0 && self.positions.len() > self.max_length {
            self.positions.remove(0);
        }
    }

    pub fn clear(&mut self) {
        self.positions.clear();
    }
}
