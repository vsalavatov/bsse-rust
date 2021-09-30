use super::{RayIntersection, Vec3, Vec3Projection};

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl RayIntersection for Sphere {
    fn intersect_ray(&self, ray: super::Ray) -> Option<Vec3> {
        // http://www.lighthouse3d.com/tutorials/maths/ray-sphere-intersection/
        let orig2center = self.center - ray.origin;
        let dot = orig2center.dot(ray.direction);
        if dot < 0.0 {
            if orig2center.length() > self.radius {
                None
            } else {
                let center_proj = ray.flip().project(self.center).unwrap();
                let dist =
                    (self.radius * self.radius - (self.center - center_proj).length2()).sqrt();
                Some(ray.origin + ray.direction * (dist + dot)) // dot < 0
            }
        } else {
            let center_proj = ray.project(self.center).unwrap();
            let proj = self.center - center_proj;
            if proj.length() > self.radius {
                None
            } else {
                let dist = (self.radius * self.radius - proj.length2()).sqrt();
                if orig2center.length() > self.radius {
                    Some(ray.origin + ray.direction * (dot - dist))
                } else {
                    Some(ray.origin + ray.direction * (dot + dist))
                }
            }
        }
    }
}
