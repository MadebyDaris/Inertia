#[derive(Clone, Debug)]
pub struct Outline {
    pub color: [f32; 4],
    pub thickness: f32,
    pub offset: f32,
    pub enabled: bool,
}

impl Outline {
    pub fn new() -> Self {
        Self {
            color: [1.0, 1.0, 0.0, 1.0],
            thickness: 2.0,
            offset: 1.02,
            enabled: false,
        }
    }

    pub fn selected() -> Self {
        Self {
            color: [1.0, 1.0, 0.0, 1.0],
            thickness: 3.0,
            offset: 1.03,
            enabled: true,
        }
    }
}
