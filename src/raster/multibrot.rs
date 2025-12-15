use crate::raster::ViewWindow;

use super::RasterGenerator;
use indicatif::{ProgressBar, ProgressStyle};
use num::complex::Complex;
use std::error::Error;
use std::fmt;

/// Multibrot set raster builder, defined as the mandelbrot set for the transformation z => z^n + c
pub struct Multibrot {
    pub max_iter: u32,
    pub n: f64,
    pub view_window: ViewWindow,
}

impl Multibrot {
    pub fn new(n: f64, max_iter: u32) -> Result<Self, Box<dyn Error>> {
        Self::new_with_view(n, max_iter, ViewWindow::full())
    }

    pub fn new_with_view(
        n: f64,
        max_iter: u32,
        view_window: ViewWindow,
    ) -> Result<Self, Box<dyn Error>> {
        if max_iter == 0 {
            return Err("max_iter must be > 0".into());
        }

        Ok(Self {
            max_iter,
            n,
            view_window,
        })
    }
}

impl fmt::Display for Multibrot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Multibrot [n={},max_iter={}]", self.n, self.max_iter)
    }
}

impl RasterGenerator for Multibrot {
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

        let progress_bar = ProgressBar::new(capacity as u64);
        progress_bar.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} {msg} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {percent}% ({pos}/{len}) ETA {eta_precise}"
            )
            .unwrap()
            .progress_chars("=>-"),
        );
        progress_bar.set_message("Building Multibrot raster");

        let mut i = 0;
        for py in 0..h {
            for px in 0..w {
                // Map pixel -> complex plane
                let (a, b) = self.view_window.map_pixel(px, py, w, h);
                let c: Complex<f64> = Complex::new(a, b);

                // Mandel_n iteration: z = z^3 + c, starting z=0, where c=x+iy
                let mut iter = 0u32;
                let mut z: Complex<f64> = Complex::new(0.0f64, 0.0f64);

                while iter < self.max_iter {
                    if z.norm_sqr() > 4.0 {
                        break;
                    }

                    z = z.powf(self.n) + c;

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

                i += 1;
                if i % 10_000 == 0 {
                    progress_bar.set_position(i);
                }
            }
        }

        progress_bar.finish_with_message("Finished building Multibrot raster");
        raster
    }
}
