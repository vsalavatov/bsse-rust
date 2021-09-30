use super::{Vec3, Vec3Projection};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3, // invariant: |direction| = 1.0
    _private: (),        // disable direct construction
}
// P.S is there a better way to forbid constructor? I've taken it from there: https://stackoverflow.com/questions/53588819/how-to-restrict-the-construction-of-struct

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin: origin,
            direction: direction.normalize(),
            _private: (),
        }
    }

    pub fn flip(&self) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.direction * -1.0,
            _private: (),
        }
    }
}

impl Vec3Projection for Ray {
    fn project(&self, pt: Vec3) -> Option<Vec3> {
        let orig2pt = pt - self.origin;
        let dot = orig2pt.dot(self.direction);
        if dot < 0.0 {
            None
        } else {
            Some(self.origin + self.direction * dot)
        }
    }
}

pub trait RayIntersection<Out = Vec3> {
    fn intersect_ray(&self, ray: Ray) -> Option<Out>;
}
