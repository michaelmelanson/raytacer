use crate::{
    camera::Camera,
    colour::Colour,
    geometry::{Geometry, Material},
    hit::Hit,
    ray::Ray,
    vec::Vec3,
};

pub struct Scene {
    pub camera: Camera,
    pub geometries: Vec<Geometry>,
}
impl Scene {
    pub(crate) fn render_pixel(
        &self,
        coord: (usize, usize),
        samples: usize,
    ) -> crate::colour::Colour {
        let mut colour = Colour::black();

        for _ in 0..samples {
            let ray = self.camera.screen_to_world_sampled(coord);
            colour += self.ray_colour(&ray, 10);
        }

        colour / (samples as f64)
    }

    pub fn hit_test(&self, ray: &Ray) -> Option<Hit> {
        let mut best_t: f64 = f64::INFINITY;
        let mut best_hit = None;

        for geo in &self.geometries {
            if let Some((t, normal)) = geo.shape.hit_test(ray, (0.001)..best_t) {
                if t <= best_t {
                    best_t = t;
                    best_hit = Some(Hit {
                        material: geo.material,
                        t,
                        point: ray.at(t),
                        normal,
                    });
                }
            }
        }

        best_hit
    }

    fn ray_colour(&self, ray: &Ray, max_bounces: isize) -> Colour {
        if max_bounces < 0 {
            return Colour::black();
        }

        let hit = self.hit_test(&ray);

        if let Some(hit) = hit {
            match hit.material {
                Material::ScreenSpaceGradient => {
                    let a = (ray.direction.unit().y() + 1.0) * 0.5;
                    Colour::new(1.0, 1.0, 1.0) * (1.0 - a) + Colour::new(0.5, 0.7, 1.0) * a
                }

                Material::NormalSpaceGradient => {
                    Colour::new(
                        hit.normal.x() + 1.,
                        hit.normal.y() + 1.,
                        hit.normal.z() + 1.,
                    ) * 0.5
                }

                Material::SolidColour { colour } => colour,
                Material::Diffuse { colour, albedo } => {
                    let reflected_direction = Vec3::random_on_hemisphere(&hit.normal);
                    let reflected_ray = Ray {
                        origin: hit.point,
                        direction: reflected_direction,
                    };
                    let reflected_colour =
                        self.ray_colour(&reflected_ray, max_bounces - 1) * albedo;
                    colour * reflected_colour
                }
                Material::Lambertian { colour, albedo } => {
                    let reflected_direction = Vec3::random_on_hemisphere(&hit.normal);
                    let reflected_ray = Ray {
                        origin: hit.point,
                        direction: reflected_direction + Vec3::random_unit_vector(),
                    };
                    let reflected_colour =
                        self.ray_colour(&reflected_ray, max_bounces - 1) * albedo;
                    colour * reflected_colour
                }
                Material::Metal { tint, scatter } => {
                    let reflected_direction =
                        ray.direction.reflect(&hit.normal) + Vec3::random_unit_vector() * scatter;

                    // absorb rays that get scattered into the material
                    if reflected_direction.dot(&hit.normal) < 0. {
                        return Colour::black();
                    }

                    let reflected_ray = Ray {
                        origin: hit.point,
                        direction: reflected_direction,
                    };
                    let reflected_colour = self.ray_colour(&reflected_ray, max_bounces - 1);
                    tint * reflected_colour
                }
            }
        } else {
            Colour::new(1., 0., 1.)
        }
    }
}
