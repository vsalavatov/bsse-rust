use crate::geometry::{Ray, RayIntersection, Vec3};

use super::{ColorRGB, Material, Scene, SceneEntity, SKYISH};

pub struct RayCastingResult {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl RayIntersection<RayCastingResult> for SceneEntity {
    fn intersect_ray(&self, ray: Ray) -> Option<RayCastingResult> {
        match self {
            SceneEntity::Ball { geometry, material } => {
                geometry.intersect_ray(ray).and_then(|pt| {
                    Some(RayCastingResult {
                        point: pt,
                        normal: (pt - geometry.center).normalize(),
                        material: *material,
                    })
                })
            }
        }
    }
}

impl RayIntersection<RayCastingResult> for Scene {
    fn intersect_ray(&self, ray: Ray) -> Option<RayCastingResult> {
        let mut result: Option<RayCastingResult> = None;
        for ele in &self.entities {
            let intersection = ele.intersect_ray(ray);
            if let Some(data) = intersection {
                if result.is_none()
                    || (result.as_ref().unwrap().point - ray.origin).length2()
                        > (data.point - ray.origin).length2()
                {
                    result = Some(data)
                }
            }
        }
        result
    }
}

pub fn cast_ray(ray: Ray, scene: &Scene) -> ColorRGB {
    match scene.intersect_ray(ray) {
        None => SKYISH,
        Some(rcr) => rcr.material.diffuse_color,
    }
}
