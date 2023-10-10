use clap::builder::TypedValueParser;
use rand::{distributions::Uniform, prelude::Distribution, thread_rng};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Vec3(f64, f64, f64);

impl Vec3 {
    pub fn new(coords: (f64, f64, f64)) -> Self {
        Self(coords.0, coords.1, coords.2)
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    #[allow(unused)]
    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.length()
    }

    fn random(low: f64, high: f64) -> Vec3 {
        let mut rng = thread_rng();
        let distribution = Uniform::new(low, high);

        Vec3::new((
            distribution.sample(&mut rng),
            distribution.sample(&mut rng),
            distribution.sample(&mut rng),
        ))
    }

    pub fn random_in_unit_square() -> Vec3 {
        loop {
            let p = Vec3::random(-1., 1.);
            if p.length_squared() < 1. {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Self::random_in_unit_square().unit()
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Self::random_unit_vector();
        if on_unit_sphere.dot(&normal) > 0.0 {
            // In the same hemisphere as the normal
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn reflect(&self, rhs: &Vec3) -> Vec3 {
        *self - (*rhs) * (self.dot(rhs) * 2.0)
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl clap::builder::ValueParserFactory for Vec3 {
    type Parser = Vec3Parser;

    fn value_parser() -> Self::Parser {
        Vec3Parser
    }
}

#[derive(Clone)]
pub struct Vec3Parser;

impl TypedValueParser for Vec3Parser {
    type Value = Vec3;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let Some(value) = value.to_str() else {
            return Err(clap::Error::new(clap::error::ErrorKind::ValueValidation));
        };

        let mut parts = Vec::new();

        let mut part = String::new();
        for c in value.chars() {
            if c == ',' {
                parts.push(part.clone());
                part.clear();
                continue;
            }

            part.push(c);
        }

        parts.push(part);

        if parts.len() != 3 {
            return Err(clap::Error::new(clap::error::ErrorKind::ValueValidation));
        }

        let Ok(x) = str::parse::<f64>(&parts[0]) else {
            return Err(clap::Error::new(clap::error::ErrorKind::ValueValidation));
        };

        let Ok(y) = str::parse::<f64>(&parts[1]) else {
            return Err(clap::Error::new(clap::error::ErrorKind::ValueValidation));
        };

        let Ok(z) = str::parse::<f64>(&parts[2]) else {
            return Err(clap::Error::new(clap::error::ErrorKind::ValueValidation));
        };

        let vec = Vec3::new((x, y, z));

        Ok(vec)
    }
}
