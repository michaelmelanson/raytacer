use serde::{Deserialize, Serialize};

use crate::vec::Vec3;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Colour(Vec3);

impl Colour {
    pub(crate) fn new(r: f64, g: f64, b: f64) -> Colour {
        Colour(Vec3::new((r, g, b)))
    }

    pub fn r(&self) -> f64 {
        self.0.x()
    }

    pub fn g(&self) -> f64 {
        self.0.y()
    }

    pub fn b(&self) -> f64 {
        self.0.z()
    }

    #[allow(unused)]
    pub fn white() -> Colour {
        Colour::new(1., 1., 1.)
    }

    #[allow(unused)]
    pub fn black() -> Colour {
        Colour::new(0., 0., 0.)
    }

    #[allow(unused)]
    pub fn red() -> Colour {
        Colour::new(1., 0., 0.)
    }
}

impl Default for Colour {
    fn default() -> Self {
        Self(Vec3::new((0., 0., 0.)))
    }
}

impl std::fmt::Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(R={}, G={}, B={})", self.r(), self.g(), self.b())
    }
}

impl std::ops::Add<Colour> for Colour {
    type Output = Colour;

    fn add(self, rhs: Colour) -> Self::Output {
        Colour(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign<Colour> for Colour {
    fn add_assign(&mut self, rhs: Colour) {
        *self = Colour(self.0 + rhs.0)
    }
}

impl std::ops::Mul<f64> for Colour {
    type Output = Colour;

    fn mul(self, rhs: f64) -> Self::Output {
        Colour(self.0 * rhs)
    }
}

impl std::ops::Mul<Colour> for Colour {
    type Output = Colour;

    fn mul(self, rhs: Colour) -> Self::Output {
        Colour(self.0 * rhs.0)
    }
}

impl std::ops::Div<f64> for Colour {
    type Output = Colour;

    fn div(self, rhs: f64) -> Self::Output {
        Colour(self.0 / rhs)
    }
}
