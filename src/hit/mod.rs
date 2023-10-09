use crate::{geometry::Material, vec::Vec3};

pub struct Hit {
    pub material: Material,
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}
