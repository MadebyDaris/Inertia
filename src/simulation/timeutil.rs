use std::time::{Duration, Instant};

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
    pub slowed_down: bool

}
impl SimulationTime {
    pub fn new(fixed_time_step: f32) -> Self {
        let last_frame = Instant::now();
        let time= Instant::now();
        Self { 
            accelerated_time_factor: 1.0235,
            accelerated_delta_time: 0.0, 
            delta_time: 0., 
            fixed_time_step, 
            last_frame, 
            simulation_time:0., 
            sped_up: false, 
            slowed_down: false}
    }

    pub fn set_time_acceleration(&mut self, acceleration: f32) {
        self.accelerated_time_factor = acceleration;
    }
    pub fn update(&mut self, acceleration_per_tap: f32) {
        let now = Instant::now();
        self.delta_time = now.duration_since(self.last_frame).as_secs_f32();
        self.last_frame = now;

        if self.sped_up {
            self.accelerated_time_factor *= acceleration_per_tap; // Double time acceleration
        }
        if self.slowed_down {
            self.accelerated_time_factor /= acceleration_per_tap;
        }

        self.accelerated_delta_time = self.delta_time * self.accelerated_time_factor;
        
       let mut accumulated_time = 0.0;
       while accumulated_time + self.fixed_time_step <= self.accelerated_delta_time {
           self.simulation_time += self.fixed_time_step;
           accumulated_time += self.fixed_time_step;
       }
    }
    pub fn input(&mut self, event: &KeyEvent) {
        handle_input!(self, event, 
        (ArrowUp, sped_up),
        (ArrowDown, slowed_down)
        );
    }
}