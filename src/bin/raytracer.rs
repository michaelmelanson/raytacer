extern crate raytacer;

use clap::Parser;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use png::ScaledFloat;
use rayon::prelude::*;
use raytacer::{
    camera::Camera,
    colour::Colour,
    geometry::Geometry,
    pixel::{Pixel, RGB},
    ray::Ray,
    scene::Scene,
    vec::Vec3,
};
use std::fs::File;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CliArguments {
    geometry_path: String,

    #[arg(
        short = 'o',
        long = "output",
        help = "Path to write the output image to",
        default_value = "./output.png"
    )]
    output_path: String,

    #[arg(
        short = 'w',
        long = "width",
        help = "Width of the output image in pixels",
        default_value = "400"
    )]
    width: usize,

    #[arg(
        short = 'h',
        long = "height",
        help = "Height of the output image in pixels",
        default_value = "255"
    )]
    height: usize,

    #[arg(
        short = 's',
        long = "samples",
        help = "How many rays to trace per pixel",
        default_value = "500"
    )]
    samples_per_pixel: usize,
}

fn main() {
    let args = CliArguments::parse();
    let camera = Camera::orthogonal(
        Ray::new(Vec3::new((0., 0., 0.)), Vec3::new((0., 0., -1.))),
        1.,
        (args.width, args.height),
    );

    let geometries = load_geometries(&args.geometry_path).expect(&format!(
        "failed to load geometries from '{}'",
        args.geometry_path
    ));

    let scene = Scene { camera, geometries };
    let image_size = args.width * args.height;

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
            let x = index % args.width;
            let y = index / args.width;

            *pixel = scene.render_pixel((x, y), args.samples_per_pixel);
        });

    write_to_png::<RGB>(&args.output_path, &pixels, (args.width, args.height));
}

fn load_geometries(path: &str) -> anyhow::Result<Vec<Geometry>> {
    let file = File::open(path)?;
    let geometries = serde_yaml::from_reader(file)?;

    Ok(geometries)
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
