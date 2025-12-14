use clap::Parser;
use ppm_rs::ppm::PPMImageBuffer;
use ppm_rs::raster::{mandelbrot::Mandelbrot, RasterGenerator};
use std::time::Instant;

#[derive(Parser)]
#[command(version, about)]
struct Config {
    /// Output image width
    #[arg(long, value_parser = clap::value_parser!(u16).range(1..), default_value_t = 16*600)]
    width: u16,

    /// Output image height
    #[arg(long, value_parser = clap::value_parser!(u16).range(1..), default_value_t = 9*600)]
    height: u16,

    /// Max iteration to determine if evolution of complex number c under z = z^2 + c is bounded
    #[arg(long, value_parser = clap::value_parser!(u32).range(1..), default_value_t = 100)]
    max_iter: u32,

    /// Output file path
    #[arg(long, short = 'o', default_value_t = String::from("output.ppm"))]
    output_path: String,
}

fn main() {
    let start = Instant::now();
    let config = Config::parse();

    let raster_start = Instant::now();
    let raster_builder = Mandelbrot::new(100).unwrap();
    let raster = raster_builder.generate(config.width, config.height);
    println!(
        "Finished writing {} raster in {:.2?}",
        raster_builder,
        raster_start.elapsed()
    );

    let ppm_buf = PPMImageBuffer::new(config.width, config.height, raster)
        .expect("Error creating ppm buffer");
    ppm_buf.write_to_ppm().expect("Error creating ppm file");

    println!(
        "Wrote {}x{} image to \"{}\" in {:.2?}",
        config.width,
        config.height,
        config.output_path,
        start.elapsed()
    );
}
