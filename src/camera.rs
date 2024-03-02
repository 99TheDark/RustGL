use crate::transformation::Matrix4;

pub struct Camera {
    pub position: [f32; 3],
    pub direction: [f32; 3],
}

const CAMERA_SPEED: f32 = 0.002;
const PLAYER_SPEED: f32 = 0.015;

impl Camera {
    pub fn rotate(&mut self, delta: (f64, f64)) {
        let (dx, dy) = (delta.0 as f32, delta.1 as f32);

        let mut matrix = Matrix4::identity();
        matrix.rotate_x(dy * CAMERA_SPEED);
        matrix.rotate_y(dx * CAMERA_SPEED);

        self.direction = matrix * self.direction;
    }

    pub fn step(&mut self, norm_x: f32, norm_y: f32, norm_z: f32) {
        let (x, y, z) = (
            norm_x * PLAYER_SPEED,
            norm_y * PLAYER_SPEED,
            norm_z * PLAYER_SPEED,
        );

        let forward = {
            let (dir_x, dir_z) = (self.direction[0], self.direction[2]);
            dir_z.atan2(dir_x)
        };
        let sideways = forward - std::f32::consts::FRAC_PI_2;

        self.position[0] += x * sideways.cos() + z * forward.cos();
        self.position[1] += y;
        self.position[2] += x * sideways.sin() + z * forward.sin();
    }

    pub fn new() -> Camera {
        Camera {
            position: [0.0, 0.0, 0.0],
            direction: [0.0, 0.0, 1.0],
        }
    }
}
