extern crate raytacer;

use clap::Parser;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use png::ScaledFloat;
use rayon::prelude::*;
use raytacer::{
    camera::CameraConfig,
    colour::Colour,
    geometry::Geometry,
    pixel::{Pixel, RGB},
    scene::Scene,
    vec::Vec3,
};
use std::fs::File;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CliArguments {
    geometry_path: String,

    #[arg(
        help_heading = "Image",
        short = 'o',
        long = "output",
        help = "Path to write the output image to",
        default_value = "./output.png"
    )]
    output_path: String,

    #[arg(
        help_heading = "Image",
        short = 'w',
        long = "width",
        help = "Width of the output image in pixels",
        default_value = "400"
    )]
    width: usize,

    #[arg(
        help_heading = "Image",
        short = 'h',
        long = "height",
        help = "Height of the output image in pixels",
        default_value = "255"
    )]
    height: usize,

    #[arg(
        help_heading = "Quality",
        short = 's',
        long = "samples",
        help = "How many rays to trace per pixel",
        default_value = "500"
    )]
    samples_per_pixel: usize,

    #[arg(
        help_heading = "Quality",
        long = "max-bounces",
        help = "How many times a ray can bounce",
        default_value = "20"
    )]
    max_bounces: usize,

    #[arg(
        help_heading = "Camera",
        long = "fov",
        help = "Field of view in degrees",
        default_value = "45"
    )]
    camera_fov: f64,

    #[arg(
        help_heading = "Camera",
        long = "look-from",
        help = "Camera position in comma-separated X,Y,Z coordinates",
        default_value = "-2,2,1"
    )]
    camera_origin: Vec3,

    #[arg(
        help_heading = "Camera",
        long = "look-at",
        help = "Camera target in comma-separated X,Y,Z coordinates",
        default_value = "0,0,-1"
    )]
    camera_look_at: Vec3,

    #[arg(
        help_heading = "Camera",
        long = "look-at",
        help = "Camera up vector in comma-separated X,Y,Z coordinates",
        default_value = "0,1,0"
    )]
    camera_up: Vec3,

    #[arg(
        help_heading = "Camera",
        long = "defocus-angle",
        help = "Angle of dispersion for out of focus objects",
        default_value = "0.6"
    )]
    camera_defocus_angle: f64,

    #[arg(
        help_heading = "Camera",
        long = "focus-distance",
        help = "Fixed focus distance. If not specified, it focuses on 'look-at' point."
    )]
    camera_focus_distance: Option<f64>,
}

fn main() {
    let args = CliArguments::parse();

    let focus_dist = args
        .camera_focus_distance
        .unwrap_or_else(|| (args.camera_look_at - args.camera_origin).length());

    let camera = CameraConfig::Orthogonal {
        look_from: args.camera_origin,
        look_at: args.camera_look_at,
        up: args.camera_up,
        fov_degrees: args.camera_fov,
        defocus_angle: args.camera_defocus_angle,
        focus_dist,
        image_width: args.width,
        image_height: args.height,
    }
    .into();

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

            *pixel = scene.render_pixel((x, y), args.samples_per_pixel, args.max_bounces);
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
    encoder.set_source_gamma(ScaledFloat::new(1.33));

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
