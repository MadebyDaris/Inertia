pub mod matrix;
pub mod vector;
pub mod app;
pub mod ui;

pub fn cross(a: (f32,f32,f32), b: (f32,f32,f32)) -> (f32,f32,f32) {
    return (a.1*b.2 - a.2*b.1, a.2*b.0 - a.0*b.2, a.0*b.1 - a.1*b.0)}
pub fn normalize(a: (f32,f32,f32)) -> (f32,f32,f32) {
    let len = a.0*a.0 + a.1*a.1 + a.2*a.2; let len = len.sqrt();
    return (a.0 / len,  a.1 / len,  a.2 / len)}
