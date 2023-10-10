use std::fs::File;

use clap::{Parser, ValueEnum};
use raytacer::{
    colour::Colour,
    geometry::{Geometry, Material, Shape},
    vec::Vec3,
};

#[derive(Clone, Debug, ValueEnum)]
pub enum StockScene {
    RandomSpheres,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArguments {
    #[arg(
        short = 'o',
        long = "output",
        help = "Path to write the output scene to",
        default_value = "./output.yaml"
    )]
    output_path: String,

    #[arg(
        short = 's',
        long = "scene",
        help = "The stock scene to generate",
        default_value = "random-spheres"
    )]
    scene: StockScene,
}
fn main() -> anyhow::Result<()> {
    let args = CliArguments::parse();

    let geometries = match args.scene {
        StockScene::RandomSpheres => generate_random_spheres(),
    };

    let file = File::create(&args.output_path)?;
    serde_yaml::to_writer(file, &geometries)?;

    Ok(())
}

fn generate_random_spheres() -> Vec<Geometry> {
    let mut geometries = Vec::new();

    // ground
    geometries.push(Geometry {
        shape: Shape::Sphere {
            centre: Vec3::new((0., -1000.0, 0.0)),
            radius: 1000.0,
        },
        material: Material::Lambertian {
            colour: Colour::new(0.5, 0.5, 0.5),
            albedo: 0.3,
        },
    });

    // small spheres
    for a in -11..11 {
        for b in -11..11 {
            let centre = Vec3::new((
                (a as f64) + rand::random::<f64>(),
                0.2,
                (b as f64) + rand::random::<f64>(),
            ));

            if (centre - Vec3::new((4.0, 0.2, 0.0))).length() <= 0.9 {
                continue;
            }

            let material_variate = rand::random::<f64>();
            let material = if material_variate < 0.8 {
                // diffuse
                let colour = Colour::random(0.0, 1.0) * Colour::random(0.0, 1.0);
                Material::Lambertian {
                    colour,
                    albedo: 1.0,
                }
            } else if material_variate < 0.95 {
                // metal
                let tint = Colour::random(0.5, 1.0);
                let scatter = rand::random::<f64>() * 0.5;
                Material::Metal { tint, scatter }
            } else {
                // glass
                Material::Dialectric { ior: 1.5 }
            };

            geometries.push(Geometry {
                shape: Shape::Sphere {
                    centre,
                    radius: 0.2,
                },
                material,
            })
        }
    }

    // large spheres
    geometries.push(Geometry {
        shape: Shape::Sphere {
            centre: Vec3::new((0.0, 1.0, 0.0)),
            radius: 1.0,
        },
        material: Material::Dialectric { ior: 1.5 },
    });

    geometries.push(Geometry {
        shape: Shape::Sphere {
            centre: Vec3::new((-4.0, 1.0, 0.0)),
            radius: 1.0,
        },
        material: Material::Lambertian {
            colour: Colour::new(0.4, 0.2, 0.1),
            albedo: 1.0,
        },
    });

    geometries.push(Geometry {
        shape: Shape::Sphere {
            centre: Vec3::new((4.0, 1.0, 0.0)),
            radius: 1.0,
        },
        material: Material::Metal {
            tint: Colour::new(0.7, 0.6, 0.5),
            scatter: 0.0,
        },
    });

    // background
    geometries.push(Geometry {
        shape: Shape::Background,
        material: Material::ScreenSpaceGradient,
    });

    geometries
}
