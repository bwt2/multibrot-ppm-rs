use clap::Parser;
use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

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
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let config = Config::parse();

    let magic_number = "P6";
    let width = config.width as usize;
    let height = config.height as usize;
    let maxval = 255u8;
    let output_path = config.output_path;

    let header = format!("{magic_number}\n{width} {height}\n{maxval}\n");

    let mut bytes = Vec::new();
    bytes.extend_from_slice(header.as_bytes());

    let mut raster: Vec<u8> = Vec::with_capacity(width * height * 3);

    let max_iter: u32 = config.max_iter;

    // view window in the complex plane
    let x_min = -2.0f64;
    let x_max = 1.0f64;
    let y_min = -1.0f64;
    let y_max = 1.0f64;

    for py in 0..height {
        for px in 0..width {
            // Map pixel -> complex plane
            let a = x_min + (px as f64 / (width - 1) as f64) * (x_max - x_min);
            let b = y_min + (py as f64 / (height - 1) as f64) * (y_max - y_min);

            // Mandelbrot iteration: z = z^2 + c, starting z=0, where c=x+i*y=(x,y)
            let mut zr = 0.0f64;
            let mut zi = 0.0f64;
            let mut iter = 0u32;

            while iter < max_iter {
                // z^2 = (zr + i * zi)^2 = (zr^2 - zi^2) + i(2*zr*zi)
                let zr2 = zr * zr;
                let zi2 = zi * zi;

                if zr2 + zi2 > 4.0 {
                    break;
                }

                zi = 2.0 * zr * zi + b;
                zr = zr2 - zi2 + a;

                iter += 1;
            }

            let (r, g, b) = if iter == max_iter {
                (0, 0, 0)
            } else {
                let t = iter as f64 / max_iter as f64;
                let r = (9.0 * (1.0 - t) * t * t * t * 255.0) as u8;
                let g = (15.0 * (1.0 - t) * (1.0 - t) * t * t * 255.0) as u8;
                let b = (8.5 * (1.0 - t) * (1.0 - t) * (1.0 - t) * t * 255.0) as u8;
                (r, g, b)
            };

            raster.extend_from_slice(&[r, g, b]);
        }
    }

    bytes.extend_from_slice(&raster);

    fs::write(&output_path, bytes).expect("Something went wrong");
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let elapsed = start.abs_diff(end);
    println!("Wrote {width}x{height} image to \"{output_path}\" in {elapsed:.2?}",);
}
