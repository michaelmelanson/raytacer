mod lens;

use crate::{ray::Ray, vec::Vec3};
use rand::random;

pub use self::lens::CameraLens;

pub enum CameraConfig {
    Orthogonal {
        look_from: Vec3,
        look_at: Vec3,
        up: Vec3,
        fov_degrees: f64,

        image_width: usize,
        image_height: usize,
    },
}

impl Into<Camera> for CameraConfig {
    fn into(self) -> Camera {
        match self {
            CameraConfig::Orthogonal {
                look_from,
                look_at,
                up,
                fov_degrees,
                image_width,
                image_height,
            } => {
                let focal_length = 1.;
                let theta = fov_degrees.to_radians();
                let h = (theta / 2.).tan();

                let viewport_height = 2. * h * focal_length;
                let viewport_width =
                    viewport_height * ((image_width as f64) / (image_height as f64));

                let w = (look_from - look_at).unit();
                let u = up.cross(&w).unit();
                let v = w.cross(&u);

                let viewport_u = u * viewport_width;
                let viewport_v = -v * viewport_height;

                let viewport_upper_left =
                    look_from - (w * focal_length) - (viewport_u / 2.) - (viewport_v / 2.);

                let pixel_delta_u = viewport_u / (image_width as f64);
                let pixel_delta_v = viewport_v / (image_height as f64);
                let pixel0_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

                let eye = Ray {
                    origin: look_from,
                    direction: (look_at - look_from).unit(),
                };

                let lens = CameraLens::Orthogonal {
                    pixel0_loc,
                    pixel_delta_u,
                    pixel_delta_v,
                };

                Camera { eye, lens }
            }
        }
    }
}

pub struct Camera {
    eye: Ray,
    lens: CameraLens,
}

impl Camera {
    pub fn screen_to_world(&self, coord: (usize, usize)) -> Ray {
        match self.lens {
            CameraLens::Orthogonal {
                pixel0_loc,
                pixel_delta_u,
                pixel_delta_v,
            } => {
                let pixel_centre = pixel0_loc
                    + (pixel_delta_u * coord.0 as f64)
                    + (pixel_delta_v * coord.1 as f64);

                let direction = pixel_centre - self.eye.origin;

                Ray {
                    origin: self.eye.origin,
                    direction,
                }
            }
        }
    }

    pub fn screen_to_world_sampled(&self, coord: (usize, usize)) -> Ray {
        let ray = self.screen_to_world(coord);

        match self.lens {
            CameraLens::Orthogonal {
                pixel_delta_u,
                pixel_delta_v,
                ..
            } => {
                // Returns a random point in the square surrounding a pixel at the origin.
                let px = -0.5 + random::<f64>();
                let py = -0.5 + random::<f64>();
                let sample = (pixel_delta_u * px) + (pixel_delta_v * py);

                Ray {
                    origin: ray.origin,
                    direction: ray.direction + sample,
                }
            }
        }
    }
}
