use std::ops::Range;

use serde::{Deserialize, Serialize};

use crate::{ray::Ray, vec::Vec3};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Shape {
    Background,
    Sphere { centre: Vec3, radius: f64 },
}
impl Shape {
    pub fn hit_test(&self, ray: &Ray, t_range: Range<f64>) -> Option<(f64, Vec3)> {
        match self {
            Shape::Background => Some((f64::INFINITY, Vec3::new((0., 0., 0.)))),
            Shape::Sphere { centre, radius } => {
                let oc = ray.origin - *centre;
                let a = ray.direction.length_squared();
                let half_b = oc.dot(&ray.direction);
                let c = oc.length_squared() - radius * radius;

                let discriminant = half_b * half_b - a * c;
                if discriminant < 0. {
                    return None;
                }

                let sqrt_d = discriminant.sqrt();
                let mut t = (-half_b - sqrt_d) / a;
                if !t_range.contains(&t) {
                    t = (-half_b + sqrt_d) / a;

                    if !t_range.contains(&t) {
                        return None;
                    }
                }

                let normal = (ray.at(t) - *centre) / *radius;
                Some((t, normal))
            }
        }
    }
}
