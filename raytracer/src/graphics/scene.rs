use crate::geometry::Sphere;

use super::{LightSource, Material};

#[derive(Debug)]
pub enum SceneEntity {
    Ball {
        geometry: Sphere,
        material: Material,
    },
}

#[derive(Debug)]
pub struct Scene {
    pub entities: Vec<SceneEntity>,
    pub lights: Vec<LightSource>,
}
