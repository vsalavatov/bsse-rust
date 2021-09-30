use crate::geometry::Sphere;

use super::Material;

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
}
