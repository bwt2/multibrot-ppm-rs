use clap::Parser;
use multibrot_ppm_rs::ppm::PPMImageBuffer;
use multibrot_ppm_rs::raster::{multibrot::Multibrot, RasterGenerator, ViewWindow};
use std::time::Instant;

#[derive(Parser)]
#[command(version, about)]
struct Config {
    /// Output image width
    #[arg(long, value_parser = clap::value_parser!(u16).range(1..), default_value_t = 1000)]
    width: u16,

    /// Output image height
    #[arg(long, value_parser = clap::value_parser!(u16).range(1..), default_value_t = 1000)]
    height: u16,

    /// Output file path
    #[arg(long, short = 'o', default_value_t = String::from("output.ppm"))]
    output_path: String,

    /// Output image zoom
    #[arg(long, short = 'z', default_value_t = false)]
    zoomed: bool,

    /// Multibrot n, where z -> z^n + c is used for drawing the set
    #[arg(long, short = 'n', default_value_t = 2.0)]
    n: f64,
}

fn main() {
    let start = Instant::now();
    let config = Config::parse();

    let viewwindow = if config.zoomed {
        ViewWindow::zoomed()
    } else {
        ViewWindow::full()
    };

    let raster = Multibrot::new_with_view(config.n, 100, viewwindow)
        .unwrap()
        .generate(config.width, config.height);

    PPMImageBuffer::new(config.width, config.height, raster)
        .expect("Error creating ppm buffer")
        .write_to_ppm(&config.output_path)
        .expect("Error creating ppm file");

    println!(
        "  Wrote {} x {} multibrot (n={}) image to \"{}\" in {:.2?}",
        config.width,
        config.height,
        config.n,
        config.output_path,
        start.elapsed()
    );
}
