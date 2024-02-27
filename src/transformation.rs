pub fn translate(x: f32, y: f32) -> [[f32; 3]; 3] {
    return [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [x, y, 1.0]];
}

pub fn rotate(theta: f32) -> [[f32; 3]; 3] {
    let (cos, sin) = (f32::cos(theta), f32::sin(theta));
    return [[cos, sin, 0.0], [-sin, cos, 0.0], [0.0, 0.0, 1.0]];
}

pub fn scale(x: f32, y: f32) -> [[f32; 3]; 3] {
    return [[x, 0.0, 0.0], [0.0, y, 0.0], [0.0, 0.0, 1.0]];
}
