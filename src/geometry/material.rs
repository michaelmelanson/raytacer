use serde::{Deserialize, Serialize};

use crate::colour::Colour;

#[derive(Clone, Copy, Serialize, Deserialize)]
#[allow(unused)]
pub enum Material {
    // debugging
    ScreenSpaceGradient,
    NormalSpaceGradient,
    SolidColour { colour: Colour },

    // diffuse models
    Diffuse { colour: Colour, albedo: f64 },
    Lambertian { colour: Colour, albedo: f64 },

    // reflective models
    Metal { tint: Colour, scatter: f64 },
}
