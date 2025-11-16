use crate::utils::vector::Vector;

#[derive(Copy, Clone)]
pub struct TransformMatrix {
    pub matrix: [[f32;4]; 4]
}
impl TransformMatrix {
    pub fn identity() -> Self {
        Self{ matrix:[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]] 
        }
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        Self {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [x,   y,   z,   1.0],
            ]
        }
    }

    pub fn translate(self, kx: f32, ky: f32, kz: f32) -> Self {
        let translation_matrix = Self::translation(kx, ky, kz);
        self * translation_matrix
    }

    pub fn scale(self, kx: f32, ky: f32, kz: f32) -> Self {
        let scale_matrix = Self {
            matrix: [
                [kx,  0.0, 0.0, 0.0],
                [0.0, ky,  0.0, 0.0],
                [0.0, 0.0, kz,  0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        };
        self * scale_matrix
    }

    pub fn rotation_from_direction(direction: Vector) -> Self {
        let forward = direction.normalized();
        
        // Choose up vector that's not parallel to forward
        let up = if forward.1.abs() > 0.9 {
            Vector(1.0, 0.0, 0.0)
        } else {
            Vector(0.0, 1.0, 0.0)
        };
        
        let right = Vector::cross(up, forward).normalized();
        let actual_up = Vector::cross(forward, right).normalized();

        // Store as row vectors to match the row-major convention used in translation/scale
        // Rows are: right, actual_up, forward, [0,0,0,1]
        Self {
            matrix: [
                [right.0, right.1, right.2, 0.0],
                [actual_up.0, actual_up.1, actual_up.2, 0.0],
                [forward.0, forward.1, forward.2, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        }
    }

    pub fn rotate(&mut self, rot_param: (f32, f32, f32)) {
        let x_rot = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, rot_param.0.cos(), -rot_param.0.sin(), 0.0],
            [0.0, rot_param.0.sin(), rot_param.0.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        *self = *self * TransformMatrix { matrix: x_rot };

        let y_rot = [
            [rot_param.1.cos(), 0.0, rot_param.1.sin(), 0.0],
            [0.0,               1.0, 0.0,               0.0],
            [-rot_param.1.sin(),0.0, rot_param.1.cos(), 0.0],
            [0.0,               0.0, 0.0,               1.0],
        ];
        *self = *self * TransformMatrix { matrix: y_rot };

        let z_rot = [
            [rot_param.2.cos(), -rot_param.2.sin(), 0.0, 0.0],
            [rot_param.2.sin(), rot_param.2.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        *self = *self * TransformMatrix { matrix: z_rot };
    }
}

impl std::ops::Mul for TransformMatrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result[i][j] += self.matrix[i][k] * rhs.matrix[k][j];
                }
            }
        }
        Self { matrix: result }
    }
}