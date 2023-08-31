mod rgb;
pub use rgb::RGB;

use std::fmt::Debug;

use crate::colour::Colour;

pub trait Pixel: Debug + Copy + From<Colour> {
    const WIDTH: usize;

    fn png_color_type() -> png::ColorType;
    fn png_bit_depth() -> png::BitDepth;

    fn width(&self) -> usize {
        Self::WIDTH
    }

    fn write(&self, target: &mut [u8]);
}
