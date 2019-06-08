#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }
}

pub mod consts {
    use super::*;
    pub const I: Vec3 = Vec3::new(1.0, 0.0, 0.0);
    pub const J: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    pub const K: Vec3 = Vec3::new(0.0, 0.0, 1.0);
}
