mod material;
mod shape;

pub use self::{material::Material, shape::Shape};

#[derive(Clone, Copy)]
pub struct Geometry {
    pub shape: Shape,
    pub material: Material,
}
