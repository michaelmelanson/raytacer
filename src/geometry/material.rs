use crate::colour::Colour;

#[derive(Clone, Copy)]
pub enum Material {
    ScreenSpaceGradient,
    NormalSpaceGradient,
    SolidColour(Colour),
}
