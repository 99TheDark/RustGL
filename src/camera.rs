pub struct Camera {
    pub position: [f32; 3],
    pub direction: [f32; 3],
}

const CAMERA_SPEED: f32 = 0.005;

impl Camera {
    pub fn rotate(&mut self, delta: (f64, f64)) {
        let (dx, dy) = (delta.0 as f32, delta.1 as f32);

        // TODO: Actually rotate, right it caps out at 90°
        self.direction[0] += dx * CAMERA_SPEED;
        self.direction[1] -= dy * CAMERA_SPEED;
    }
    pub fn new() -> Camera {
        Camera {
            position: [0.0, 0.0, 0.0],
            direction: [0.0, 0.0, 1.0],
        }
    }
}
