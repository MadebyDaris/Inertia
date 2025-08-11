use crate::utils::vector::Vector;

// In src/ui/mod.rs or wherever you keep your macros
#[macro_export]
macro_rules! create_widgets {
    ($simulation:expr) => {{
        $simulation
            .owned_objects  // Updated to match the new field name
            .iter()
            .zip($simulation.object_names.iter())
            .map(|(obj, name)| obj.get_widget(&name.clone()))
            .collect::<Vec<_>>()
    }};
}

// Enhanced version with filtering
#[macro_export]
macro_rules! create_widgets_filtered {
    ($simulation:expr, $filter:expr) => {{
        $simulation
            .owned_objects
            .iter()
            .zip($simulation.object_names.iter())
            .filter($filter)
            .map(|(obj, name)| obj.get_widget(name.clone()))
            .collect::<Vec<_>>()
    }};
}

// Version that returns specific widget types
#[macro_export]
macro_rules! create_info_widgets {
    ($simulation:expr) => {{
        use crate::ui::AstralBodyInfoWidget;
        $simulation
            .owned_objects
            .iter()
            .zip($simulation.object_names.iter())
            .map(|(obj, name)| obj.get_widget(name.clone()))
            .collect::<Vec<AstralBodyInfoWidget>>()
    }};
}

// Orbit trail system for visualizing object paths
#[derive(Clone)]
pub struct OrbitTrail {
    positions: Vec<Vector>,
    pub(crate) max_length: usize,
    color: [f32; 3],
    pub enabled: bool
}
impl OrbitTrail {
    pub fn new(max_length: usize, color: [f32; 3]) -> Self {
        Self {
            positions: Vec::new(),
            max_length,
            color,
            enabled: false,
        }
    }

    pub fn add_position(&mut self, position: Vector) {
        if !self.enabled { return; }
        
        self.positions.push(position);
        if self.positions.len() > self.max_length {
            self.positions.remove(0);
        }
    }

    pub fn clear(&mut self) {
        self.positions.clear();
    }    
}
