use std::time::Instant;

use glium::winit::event::KeyEvent;

use crate::handle_input;

#[derive(Clone, Copy)]
pub struct SimulationTime {
    pub accelerated_time_factor: f32,
    pub accelerated_delta_time: f32,
    pub delta_time: f32,
    pub simulation_time: f32,
    pub last_frame: Instant,
    pub fixed_time_step: f32,
    pub sped_up: bool,
    pub slowed_down: bool,
    pub paused: bool,
    pub reverse_time: bool,
}

impl SimulationTime {
    pub fn new(fixed_time_step: f32) -> Self {
        let last_frame = Instant::now();
        Self { 
            accelerated_time_factor: 1.0,
            accelerated_delta_time: 0.0, 
            delta_time: 0., 
            fixed_time_step, 
            last_frame, 
            simulation_time: 0., 
            sped_up: false, 
            slowed_down: false,
            paused: false,
            reverse_time: false,
        }
    }

    pub fn set_time_acceleration(&mut self, acceleration: f32) {
        self.accelerated_time_factor = acceleration;
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    pub fn toggle_reverse(&mut self) {
        self.reverse_time = !self.reverse_time;
    }

    pub fn update(&mut self, acceleration_per_tap: f32) {
        let now = Instant::now();
        self.delta_time = now.duration_since(self.last_frame).as_secs_f32();
        self.last_frame = now;

        if self.paused {
            self.accelerated_delta_time = 0.0;
            // Reset input flags even when paused
            self.sped_up = false;
            self.slowed_down = false;
            return;
        }

        if self.sped_up {
            self.accelerated_time_factor *= acceleration_per_tap;
            self.sped_up = false; // Reset flag after processing
        }
        if self.slowed_down {
            self.accelerated_time_factor /= acceleration_per_tap;
            if self.accelerated_time_factor < 0.001 {
                self.accelerated_time_factor = 0.001;
            }
            self.slowed_down = false; // Reset flag after processing
        }

        let time_direction = if self.reverse_time { -1.0 } else { 1.0 };
        self.accelerated_delta_time = self.delta_time * self.accelerated_time_factor * time_direction;
        
        let mut accumulated_time: f32 = 0.0;
        while accumulated_time.abs() + self.fixed_time_step <= self.accelerated_delta_time.abs() {
            self.simulation_time += self.fixed_time_step * time_direction;
            accumulated_time += self.fixed_time_step * time_direction;
        }
    }

    pub fn input(&mut self, event: &KeyEvent) {
        handle_input!(self, event, 
            (ArrowUp, sped_up),
            (ArrowDown, slowed_down)
        );
    }

    pub fn get_time_scale_display(&self) -> String {
        if self.paused {
            "PAUSED".to_string()
        } else if self.reverse_time {
            format!("REVERSE {:.2}x", self.accelerated_time_factor)
        } else {
            format!("{:.2}x", self.accelerated_time_factor)
        }
    }
}