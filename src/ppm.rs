use std::{error::Error, fs::File, io::Write};

pub struct PPMImageBuffer {
    width: u16,
    height: u16,
    raster: Vec<u8>,
}

impl PPMImageBuffer {
    pub fn new(width: u16, height: u16, raster: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        if width <= 0 || height <= 0 {
            return Err("width and height must be > 0".into());
        }

        let expected = (width as usize)
            .checked_mul(height as usize)
            .and_then(|px| px.checked_mul(3))
            .ok_or_else(|| "image dimensions too large")?;

        if raster.len() != expected {
            return Err("raster length must be width * height * 3".into());
        }

        Ok(Self {
            width,
            height,
            raster,
        })
    }

    pub fn write_to_ppm(&self) -> Result<(), std::io::Error> {
        let mut f = File::create("output.ppm")?;
        write!(f, "P6\n{} {}\n255\n", self.width, self.height)?;
        f.write_all(&self.raster)?;
        Ok(())
    }
}
