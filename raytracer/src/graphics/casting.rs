use crate::geometry::{Ray, RayIntersection, Sphere};

use super::{ColorRGB, BLACK, WHITE};

pub fn cast_ray(ray: Ray, sphere: Sphere) -> ColorRGB {
    match sphere.intersect_ray(ray) {
        None => BLACK,
        Some(pt) => WHITE,
    }
}
