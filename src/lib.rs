//! ppm-rs
//!
//! A small ppm library for drawing Mandelbrot and Multibrot fractals.
//!
//! # Example
//! ```rust
//! use multibrot_ppm_rs::raster::{Multibrot, ViewWindow};
//!
//! let mb = Multibrot::new(2.0, 100, ViewWindow::default()).unwrap();
//! let raster = mb.generate(800, 600);
//! ```

pub mod ppm;
pub mod raster;
