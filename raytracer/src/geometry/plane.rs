use super::{RayIntersection, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Plane {
    // point is on the plane if (pt . normal + shift) = 0
    pub normal: Vec3,
    pub shift: f32,
}

impl RayIntersection for Plane {
    fn intersect_ray(&self, ray: super::Ray) -> Option<Vec3> {
        let diff = ray.origin.dot(self.normal) + self.shift;
        let t = -diff / ray.direction.dot(self.normal);
        if t.is_finite() && t >= 0.0 {
            Some(ray.origin + ray.direction * t)
        } else {
            None
        }
    }
}
