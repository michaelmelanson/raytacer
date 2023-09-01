use crate::colour::Colour;

#[derive(Clone, Copy)]
#[allow(unused)]
pub enum Material {
    // debugging
    ScreenSpaceGradient,
    NormalSpaceGradient,
    SolidColour(Colour),

    // diffuse models
    Diffuse(Colour, f64),
    Lambertian(Colour, f64),

    // reflective models
    Metal(Colour, f64),
}
