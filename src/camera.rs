use crate::transformation::Matrix4;

pub struct Camera {
    pub position: [f32; 3],
    pub direction: [f32; 3],
}

const CAMERA_SPEED: f32 = 0.002;

impl Camera {
    pub fn rotate(&mut self, delta: (f64, f64)) {
        let (dx, dy) = (delta.0 as f32, delta.1 as f32);

        let mut matrix = Matrix4::identity();
        matrix.rotate_x(dy * CAMERA_SPEED);
        matrix.rotate_y(dx * CAMERA_SPEED);

        self.direction = matrix * self.direction;
    }

    pub fn new() -> Camera {
        Camera {
            position: [0.0, 0.0, 0.0],
            direction: [0.0, 0.0, 1.0],
        }
    }
}
