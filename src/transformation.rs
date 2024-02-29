#[allow(dead_code)]
pub fn translate(x: f32, y: f32, z: f32) -> [[f32; 4]; 4] {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [x, y, z, 1.0],
    ]
}

#[allow(dead_code)]
pub fn rotate_roll(theta: f32) -> [[f32; 4]; 4] {
    let (cos, sin) = (f32::cos(theta), f32::sin(theta));

    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, cos, sin, 0.0],
        [0.0, -sin, cos, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

#[allow(dead_code)]
pub fn rotate_pitch(theta: f32) -> [[f32; 4]; 4] {
    let (cos, sin) = (f32::cos(theta), f32::sin(theta));

    [
        [cos, 0.0, -sin, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [sin, 0.0, cos, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

#[allow(dead_code)]
pub fn rotate_yaw(theta: f32) -> [[f32; 4]; 4] {
    let (cos, sin) = (f32::cos(theta), f32::sin(theta));

    [
        [cos, sin, 0.0, 0.0],
        [-sin, cos, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

#[allow(dead_code)]
pub fn scale(sx: f32, sy: f32, sz: f32) -> [[f32; 4]; 4] {
    [
        [sx, 0.0, 0.0, 0.0],
        [0.0, sy, 0.0, 0.0],
        [0.0, 0.0, sz, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

#[allow(dead_code)]
pub fn perspective(width: u32, height: u32, near: f32, far: f32) -> [[f32; 4]; 4] {
    let aspect = height as f32 / width as f32;

    let fov: f32 = std::f32::consts::PI / 3.0;
    let sf = 1.0 / (fov / 2.0).tan();

    [
        [sf * aspect, 0.0, 0.0, 0.0],
        [0.0, sf, 0.0, 0.0],
        [0.0, 0.0, (near + far) / (far - near), 1.0],
        [0.0, 0.0, -(2.0 * near * far) / (far - near), 0.0],
    ]
}

#[allow(dead_code)]
pub fn identity() -> [[f32; 4]; 4] {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}
