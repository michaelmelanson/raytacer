mod material;
mod shape;

use serde::{Deserialize, Serialize};

pub use self::{material::Material, shape::Shape};

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct Geometry {
    pub shape: Shape,
    pub material: Material,
}
