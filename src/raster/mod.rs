use std::fmt::Display;

pub trait RasterGenerator: Display {
    fn generate(&self, width: u16, height: u16) -> Vec<u8>;
}

pub mod mandelbrot;
