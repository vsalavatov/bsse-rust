use crate::geometry::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct LightSource {
    pub position: Vec3,
    pub intensity: f32,
}
