extern crate glium;

use crate::simulation::*;

mod simulation;
mod render;
mod utils;
mod physics;
mod ui;
pub mod mesh;
mod example;

fn main() {
    example::example()
}