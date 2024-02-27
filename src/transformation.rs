pub fn translate(x: f32, y: f32, z: f32) -> [[f32; 4]; 4] {
    return [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [x, y, z, 1.0],
    ];
}

pub fn rotate_pitch(theta: f32) -> [[f32; 4]; 4] {
    let (cos, sin) = (f32::cos(theta), f32::sin(theta));
    return [
        [cos, 0.0, sin, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-sin, 0.0, cos, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
}

#[allow(dead_code)]
pub fn rotate_roll(theta: f32) -> [[f32; 4]; 4] {
    let (cos, sin) = (f32::cos(theta), f32::sin(theta));
    return [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, cos, -sin, 0.0],
        [0.0, sin, cos, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
}

pub fn rotate_yaw(theta: f32) -> [[f32; 4]; 4] {
    let (cos, sin) = (f32::cos(theta), f32::sin(theta));
    return [
        [cos, sin, 0.0, 0.0],
        [-sin, cos, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
}

pub fn scale(sx: f32, sy: f32, sz: f32) -> [[f32; 4]; 4] {
    return [
        [sx, 0.0, 0.0, 0.0],
        [0.0, sy, 0.0, 0.0],
        [0.0, 0.0, sz, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
}

pub fn identity() -> [[f32; 4]; 4] {
    return [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
}
