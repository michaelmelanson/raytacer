use crate::colour::Colour;

#[derive(Debug, Clone, Copy)]
pub struct RGB(pub [u8; 3]);

impl super::Pixel for RGB {
    const WIDTH: usize = 3;

    fn write(&self, target: &mut [u8]) {
        target.copy_from_slice(&self.0);
    }

    fn png_color_type() -> png::ColorType {
        png::ColorType::Rgb
    }

    fn png_bit_depth() -> png::BitDepth {
        png::BitDepth::Eight
    }
}

impl From<Colour> for RGB {
    fn from(value: Colour) -> Self {
        Self([
            (value.r() * 256.0) as u8,
            (value.g() * 256.0) as u8,
            (value.b() * 256.0) as u8,
        ])
    }
}
