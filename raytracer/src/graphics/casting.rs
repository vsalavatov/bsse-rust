use crate::geometry::{Ray, RayIntersection, Vec3};

use super::{ColorRGB, Material, Scene, SceneEntity, SKYISH, WHITE};

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

pub fn cast_ray(ray: Ray, scene: &Scene, recursion_limit: usize) -> ColorRGB {
    if recursion_limit == 0 {
        return SKYISH;
    }
    match scene.intersect_ray(ray) {
        None => SKYISH,
        Some(rcr) => {
            let reflect_dir = ray.direction.reflect(rcr.normal);
            let reflect_origin =
                rcr.point + rcr.normal * 1e-3 * reflect_dir.dot(rcr.normal).signum();
            let reflect_color = cast_ray(
                Ray::new(reflect_origin, reflect_dir),
                scene,
                recursion_limit - 1,
            );

            let mut diffuse_light_intensity: f32 = 0.0;
            let mut specular_light_intensity: f32 = 0.0;
            for light in &scene.lights {
                let light_dir = (light.position - rcr.point).normalize();
                let light_distance = (light.position - rcr.point).length();

                let shadow_origin =
                    rcr.point + rcr.normal * 1e-3 * light_dir.dot(rcr.normal).signum();
                if let Some(shadow_intersection) =
                    scene.intersect_ray(Ray::new(shadow_origin, light_dir))
                {
                    if (shadow_intersection.point - shadow_origin).length() < light_distance {
                        continue;
                    }
                }

                diffuse_light_intensity += light.intensity * light_dir.dot(rcr.normal).max(0.0);
                specular_light_intensity += light_dir
                    .reflect(rcr.normal)
                    .dot(ray.direction)
                    .max(0.0)
                    .powf(rcr.material.specular_exponent)
                    * light.intensity;
            }
            rcr.material.diffuse_color * diffuse_light_intensity * rcr.material.diffuse_albedo
                + WHITE * specular_light_intensity * rcr.material.specular_albedo
                + reflect_color * rcr.material.reflection_albedo
        }
    }
}
