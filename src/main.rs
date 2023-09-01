use camera::Camera;
use colour::Colour;
use geometry::{Material, Shape};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use pixel::{Pixel, RGB};
use png::ScaledFloat;
use ray::Ray;
use rayon::prelude::*;
use scene::Scene;
use vec::Vec3;

use crate::geometry::Geometry;

mod camera;
mod colour;
mod geometry;
mod hit;
mod pixel;
mod ray;
mod scene;
mod vec;

fn main() {
    let image_width = 400;
    let image_height = 225;
    let image_size: usize = image_width * image_height;

    let camera = Camera::orthogonal(
        Ray::new(Vec3::new((0., 0., 0.)), Vec3::new((0., 0., -1.))),
        1.,
        (image_width, image_height),
    );

    let mut geometries = Vec::new();

    // ground
    geometries.push(Geometry {
        shape: Shape::Sphere {
            centre: Vec3::new((0., -100.5, -1.)),
            radius: 100.,
        },
        material: Material::Lambertian(Colour::new(0.8, 0.8, 0.0), 0.5),
    });

    // center
    geometries.push(Geometry {
        shape: Shape::Sphere {
            centre: Vec3::new((0., 0., -1.)),
            radius: 0.5,
        },
        material: Material::Lambertian(Colour::new(0.7, 0.3, 0.3), 0.5),
    });

    // left
    geometries.push(Geometry {
        shape: Shape::Sphere {
            centre: Vec3::new((-1., 0., -1.)),
            radius: 0.5,
        },
        material: Material::Metal(Colour::new(0.8, 0.8, 0.8), 0.3),
    });

    // right
    geometries.push(Geometry {
        shape: Shape::Sphere {
            centre: Vec3::new((1., 0., -1.)),
            radius: 0.5,
        },
        material: Material::Metal(Colour::new(0.8, 0.6, 0.2), 1.0),
    });

    // sky
    geometries.push(Geometry {
        shape: Shape::Background,
        material: Material::ScreenSpaceGradient,
    });

    let scene = Scene { camera, geometries };

    let mut pixels = Vec::new();
    pixels.resize(image_size, Colour::default());

    let progress = ProgressBar::new(image_size as u64).with_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {wide_bar:.cyan/blue} {pos:>7}/{len:7} {eta} left {msg}",
        )
        .unwrap(),
    );
    pixels
        .par_iter_mut()
        .enumerate()
        .progress_with(progress)
        .for_each(|(index, pixel)| {
            let x = index % image_width;
            let y = index / image_width;

            *pixel = scene.render_pixel((x, y), 500);
        });

    write_to_png::<RGB>("output.png", &pixels, (image_width, image_height));
}

fn write_to_png<P: Pixel>(path: &str, pixels: &[Colour], dimensions: (usize, usize)) {
    let file = std::fs::File::create(path).unwrap();
    let writer = &mut std::io::BufWriter::new(file);

    let mut encoder = png::Encoder::new(writer, dimensions.0 as u32, dimensions.1 as u32);
    encoder.set_color(P::png_color_type());
    encoder.set_depth(P::png_bit_depth());
    encoder.set_source_gamma(ScaledFloat::new(1.0));

    let mut data = Vec::new();
    data.resize(pixels.len() * P::WIDTH, 0);

    pixels.iter().enumerate().for_each(|(index, colour)| {
        let pixel = P::from(*colour);
        pixel.write(&mut data[index * P::WIDTH..(index * P::WIDTH) + P::WIDTH]);
    });

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&data).unwrap();
    writer.finish().unwrap();
}
