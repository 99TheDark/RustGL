use std::ops;

pub struct Matrix4([[f32; 4]; 4]);

impl Matrix4 {
    pub fn to_array(&self) -> [[f32; 4]; 4] {
        self.0
    }

    pub fn copy(&self) -> Matrix4 {
        Matrix4(self.0)
    }

    fn apply(&mut self, transformation: Matrix4) {
        let prod = self.copy() * transformation;
        *self = prod;
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.apply(Matrix4([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [x, y, z, 1.0],
        ]));
    }

    pub fn scale(&mut self, sx: f32, sy: f32, sz: f32) {
        self.apply(Matrix4([
            [sx, 0.0, 0.0, 0.0],
            [0.0, sy, 0.0, 0.0],
            [0.0, 0.0, sz, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]));
    }

    pub fn rotate_x(&mut self, theta: f32) {
        let (cos, sin) = (f32::cos(theta), f32::sin(theta));

        self.apply(Matrix4([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cos, sin, 0.0],
            [0.0, -sin, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]));
    }

    pub fn rotate_y(&mut self, theta: f32) {
        let (cos, sin) = (f32::cos(theta), f32::sin(theta));

        self.apply(Matrix4([
            [cos, 0.0, -sin, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [sin, 0.0, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]));
    }

    pub fn rotate_z(&mut self, theta: f32) {
        let (cos, sin) = (f32::cos(theta), f32::sin(theta));

        self.apply(Matrix4([
            [cos, sin, 0.0, 0.0],
            [-sin, cos, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]));
    }

    pub fn identity() -> Matrix4 {
        Matrix4([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

impl ops::Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: Matrix4) -> Self::Output {
        let (a, b) = (self.0, rhs.0);
        Matrix4([
            [
                a[0][0] * b[0][0] + a[1][0] * b[0][1] + a[2][0] * b[0][2] + a[3][0] * b[0][3],
                a[0][1] * b[0][0] + a[1][1] * b[0][1] + a[2][1] * b[0][2] + a[3][1] * b[0][3],
                a[0][2] * b[0][0] + a[1][2] * b[0][1] + a[2][2] * b[0][2] + a[3][2] * b[0][3],
                a[0][3] * b[0][0] + a[1][3] * b[0][1] + a[2][3] * b[0][2] + a[3][3] * b[0][3],
            ],
            [
                a[0][0] * b[1][0] + a[1][0] * b[1][1] + a[2][0] * b[1][2] + a[3][0] * b[1][3],
                a[0][1] * b[1][0] + a[1][1] * b[1][1] + a[2][1] * b[1][2] + a[3][1] * b[1][3],
                a[0][2] * b[1][0] + a[1][2] * b[1][1] + a[2][2] * b[1][2] + a[3][2] * b[1][3],
                a[0][3] * b[1][0] + a[1][3] * b[1][1] + a[2][3] * b[1][2] + a[3][3] * b[1][3],
            ],
            [
                a[0][0] * b[2][0] + a[1][0] * b[2][1] + a[2][0] * b[2][2] + a[3][0] * b[2][3],
                a[0][1] * b[2][0] + a[1][1] * b[2][1] + a[2][1] * b[2][2] + a[3][1] * b[2][3],
                a[0][2] * b[2][0] + a[1][2] * b[2][1] + a[2][2] * b[2][2] + a[3][2] * b[2][3],
                a[0][3] * b[2][0] + a[1][3] * b[2][1] + a[2][3] * b[2][2] + a[3][3] * b[2][3],
            ],
            [
                a[0][0] * b[3][0] + a[1][0] * b[3][1] + a[2][0] * b[3][2] + a[3][0] * b[3][3],
                a[0][1] * b[3][0] + a[1][1] * b[3][1] + a[2][1] * b[3][2] + a[3][1] * b[3][3],
                a[0][2] * b[3][0] + a[1][2] * b[3][1] + a[2][2] * b[3][2] + a[3][2] * b[3][3],
                a[0][3] * b[3][0] + a[1][3] * b[3][1] + a[2][3] * b[3][2] + a[3][3] * b[3][3],
            ],
        ])
    }
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
fn view(position: &[f32; 3], direction: &[f32; 3]) -> [[f32; 4]; 4] {
    let up = [0.0, 1.0, 0.0];

    let f = {
        let f = direction;
        let length = (direction[0] * direction[0]
            + direction[1] * direction[1]
            + direction[2] * direction[2])
            .sqrt();
        [f[0] / length, f[1] / length, f[2] / length]
    };

    let s = [
        up[1] * f[2] - up[2] * f[1],
        up[2] * f[0] - up[0] * f[2],
        up[0] * f[1] - up[1] * f[0],
    ];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [
        f[1] * s_norm[2] - f[2] * s_norm[1],
        f[2] * s_norm[0] - f[0] * s_norm[2],
        f[0] * s_norm[1] - f[1] * s_norm[0],
    ];

    let p = [
        -position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
        -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
        -position[0] * f[0] - position[1] * f[1] - position[2] * f[2],
    ];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}
