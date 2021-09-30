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
        Some(rcr) => {
            let mut diffuse_light_intensity: f32 = 0.0;
            for light in &scene.lights {
                let light_dir = (light.position - rcr.point).normalize();
                diffuse_light_intensity += light.intensity * light_dir.dot(rcr.normal).max(0.0)
            }
            rcr.material.diffuse_color * diffuse_light_intensity
        }
    }
}
