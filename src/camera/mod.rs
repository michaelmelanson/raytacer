mod len;

use rand::random;
use serde::{Deserialize, Serialize};

use crate::{ray::Ray, vec::Vec3};

pub use self::len::CameraLens;

#[derive(Serialize, Deserialize)]
pub struct Camera {
    eye: Ray,
    lens: CameraLens,
}

impl Camera {
    pub fn orthogonal(eye: Ray, focal_length: f64, image_size: (usize, usize)) -> Self {
        let viewport_height = 2.;
        let viewport_width = viewport_height * (image_size.0 as f64) / (image_size.1 as f64);

        let viewport_u = Vec3::new((viewport_width, 0., 0.));
        let viewport_v = Vec3::new((0., -viewport_height, 0.));

        let pixel_delta_u = viewport_u / (image_size.0 as f64);
        let pixel_delta_v = viewport_v / (image_size.1 as f64);

        let viewport_upper_left =
            eye.origin - Vec3::new((0., 0., focal_length)) - (viewport_u / 2.) - (viewport_v / 2.);

        let pixel0_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let lens = CameraLens::Orthogonal {
            pixel0_loc,
            pixel_delta_u,
            pixel_delta_v,
        };

        Camera { eye, lens }
    }

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
