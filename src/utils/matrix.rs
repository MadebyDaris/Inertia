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

    fn mat_x_mat(m_: TransformMatrix, m: TransformMatrix) -> TransformMatrix {
        let m1 = m_.matrix;
        let m2 = m.matrix;
        let mut f = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    f[i][j] += m1[i][k] * m2[k][j]
                }
            }
        }
        return TransformMatrix { matrix : f };
    }

    pub fn translate(mut self, kx: f32, ky: f32, kz: f32) -> Self {
        self.matrix[3][0] += kx;
        self.matrix[3][1] += ky;
        self.matrix[3][2] += kz;
        self
    }
    
    pub fn scale(mut self, kx: f32, ky: f32, kz:f32) -> Self {
        self.matrix[0][0] *= kx;
        self.matrix[1][1] *= ky;
        self.matrix[2][2] *= kz;
        self
    }


    // Instead of combining all rotations at once, apply 
    // each rotation incrementally to the transformation matrix. 
    // This approach can sometimes alleviate the order-dependent problems in Euler angle-based rotations:
    pub fn rotate(&mut self, rot_param: (f32, f32, f32)) {
        let mut x_rot = TransformMatrix::identity().matrix;
        x_rot = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, rot_param.0.cos(), -rot_param.0.sin(), 0.0],
        [0.0, rot_param.0.sin(), rot_param.0.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
        ];
        self.matrix = TransformMatrix::mat_x_mat(*self, TransformMatrix{ matrix: x_rot}).matrix;

        let mut y_rot = TransformMatrix::identity().matrix;
        let y_rot = [
            [rot_param.1.cos(), 0.0, rot_param.1.sin(), 0.0],
            [0.0,               1.0, 0.0,               0.0],
            [-rot_param.1.sin(),0.0, rot_param.1.cos(), 0.0],
            [0.0,               0.0, 0.0,               1.0],
        ];
        self.matrix = TransformMatrix::mat_x_mat(*self, TransformMatrix{ matrix: y_rot}).matrix;

        let mut z_rot = TransformMatrix::identity().matrix;
        let z_rot = [
            [rot_param.2.cos(), -rot_param.2.sin(), 0.0, 0.0],
            [rot_param.2.sin(), rot_param.2.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        self.matrix = TransformMatrix::mat_x_mat(*self, TransformMatrix{ matrix: z_rot}).matrix;
    }
}