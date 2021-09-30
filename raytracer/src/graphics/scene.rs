use crate::geometry::{Plane, Sphere};

use super::{LightSource, Material};

#[derive(Debug)]
pub enum SceneEntity {
    Ball {
        geometry: Sphere,
        material: Material,
    },
    Checkerboard {
        geometry: Plane,
        materials: [Material; 2],
        cell_size: f32,
    },
}

#[derive(Debug)]
pub struct Scene {
    pub entities: Vec<SceneEntity>,
    pub lights: Vec<LightSource>,
}
