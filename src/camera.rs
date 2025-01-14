use glam::{Mat4, Vec3};

pub struct Camera {
    pub position: Vec3,
    pub pitch: f32, // Up/Down
    pub yaw: f32,   // Left/Right
}

impl Camera {
    pub const fn new(position: Vec3, pitch: f32, yaw: f32) -> Self {
        Self {
            position,
            pitch,
            yaw,
        }
    }

    pub fn get_view(&self) -> Mat4 {
        Mat4::look_to_rh(self.position.into(), self.get_forward(), Vec3::Y)
    }

    pub fn get_forward(&self) -> Vec3 {
        Vec3::new(
            self.pitch.cos() * self.yaw.sin(),
            self.pitch.sin(),
            -self.pitch.cos() * self.yaw.cos(),
        )
    }
}
