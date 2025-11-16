use crate::{render::CameraMat, utils::vector::Vector};

pub fn project_to_screen(
    pos: Vector,
    camera_mat: &CameraMat,
    width: u32,
    height: u32,
) -> Option<(f32, f32)> {
    let view = camera_mat.view_mat.matrix;
    let proj = camera_mat.pers_mat.matrix;

    let pos_vec = [pos.0, pos.1, pos.2, 1.0];
    let mut view_pos = [0.0; 4];

    // Multiply: view * pos (column-major order)
    for i in 0..4 {
        for j in 0..4 {
            view_pos[i] += view[j][i] * pos_vec[j];
        }
    }

    let mut clip_pos = [0.0; 4];
    // Multiply: proj * view_pos (column-major order)
    for i in 0..4 {
        for j in 0..4 {
            clip_pos[i] += proj[j][i] * view_pos[j];
        }
    }

    // Perspective divide - check if point is in front of camera
    if clip_pos[3].abs() > 0.001 && clip_pos[3] > 0.0 {
        let ndc_x = clip_pos[0] / clip_pos[3];
        let ndc_y = clip_pos[1] / clip_pos[3];

        // Convert NDC to screen coordinates
        let screen_x = (ndc_x + 1.0) * 0.5 * width as f32;
        let screen_y = (1.0 - ndc_y) * 0.5 * height as f32;

        Some((screen_x, screen_y))
    } else {
        None
    }
}
