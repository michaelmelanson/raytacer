use crate::{
    camera::Camera,
    colour::Colour,
    geometry::{Geometry, Material},
    hit::Hit,
    ray::Ray,
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
        let mut colour = Colour::default();

        for _ in 0..samples {
            let ray = self.camera.screen_to_world_sampled(coord);
            let hit = self.hit_test(&ray);

            if let Some(hit) = hit {
                colour += match hit.material {
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

                    Material::SolidColour(colour) => colour,
                };
            } else {
                colour += Colour::new(1., 0., 1.);
            }
        }

        colour / (samples as f64)
    }

    pub fn hit_test(&self, ray: &Ray) -> Option<Hit> {
        let mut best_t: f64 = f64::INFINITY;
        let mut best_hit = None;

        for geo in &self.geometries {
            if let Some((t, normal)) = geo.shape.hit_test(ray, (0.)..best_t) {
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
}
