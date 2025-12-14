use super::RasterGenerator;
use std::error::Error;
use std::fmt;

/// Mandelbrot set raster builder
pub struct Mandelbrot {
    pub max_iter: u32,
}

impl Mandelbrot {
    pub fn new(max_iter: u32) -> Result<Self, Box<dyn Error>> {
        if max_iter <= 0 {
            return Err("max_iter should be > 0".into());
        }
        Ok(Mandelbrot { max_iter })
    }
}

impl fmt::Display for Mandelbrot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mandelbrot [max_iter={}]", self.max_iter)
    }
}

impl RasterGenerator for Mandelbrot {
    fn generate(&self, width: u16, height: u16) -> Vec<u8> {
        if width == 0 || height == 0 {
            return Vec::new();
        }

        let w = width as usize;
        let h = height as usize;

        let capacity = w
            .checked_mul(h)
            .and_then(|px| px.checked_mul(3))
            .expect("image too large");
        let mut raster = Vec::with_capacity(capacity);

        // view window in the complex plane
        let x_min = -2.0f64;
        let x_max = 1.0f64;
        let y_min = -1.0f64;
        let y_max = 1.0f64;

        let denom_x = (w - 1).max(1) as f64;
        let denom_y = (h - 1).max(1) as f64;

        for py in 0..h {
            for px in 0..w {
                // Map pixel -> complex plane
                let a = x_min + (px as f64 / denom_x) * (x_max - x_min);
                let b = y_min + (py as f64 / denom_y) * (y_max - y_min);

                // Mandelbrot iteration: z = z^2 + c, starting z=0, where c=x+i*y=(x,y)
                let mut zr = 0.0f64;
                let mut zi = 0.0f64;
                let mut iter = 0u32;

                while iter < self.max_iter {
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

                let (r, g, b) = if iter == self.max_iter {
                    (0u8, 0u8, 0u8)
                } else {
                    let t = iter as f64 / self.max_iter as f64;
                    let r = (9.0 * (1.0 - t) * t * t * t * 255.0) as u8;
                    let g = (15.0 * (1.0 - t) * (1.0 - t) * t * t * 255.0) as u8;
                    let b = (8.5 * (1.0 - t) * (1.0 - t) * (1.0 - t) * t * 255.0) as u8;
                    (r, g, b)
                };

                raster.push(r);
                raster.push(g);
                raster.push(b);
            }
        }

        raster
    }
}
