/// ViewWindow acts as the bounding box for the generated image,
/// as if you were "cropping" the image from the plane
///
/// # Examples
/// ```
/// use multibrot_ppm_rs::raster::ViewWindow;
/// 
/// let vw = ViewWindow::new(-1.0, 1.0, -1.0, 1.0);
/// let vw1 = ViewWindow::full().zoom(4.0).pan(-0.5, 0.0);
/// let vw2 = ViewWindow::zoomed();
/// ```
#[derive(Debug)]
pub struct ViewWindow {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
}

impl ViewWindow {
    pub fn new(x_min: f64, x_max: f64, y_min: f64, y_max: f64) -> Self {
        Self {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    pub fn full() -> Self {
        Self::default()
    }

    pub fn zoomed() -> Self {
        Self {
            x_min: -0.75,
            x_max: 0.75,
            y_min: -0.75,
            y_max: 0.75,
        }
    }

    pub fn zoom(&self, factor: f64) -> Self {
        let cx = (self.x_min + self.x_max) / 2.0;
        let cy = (self.y_min + self.y_max) / 2.0;
        let hw = (self.x_max - self.x_min) / 2.0 / factor;
        let hh = (self.y_max - self.y_min) / 2.0 / factor;

        Self {
            x_min: cx - hw,
            x_max: cx + hw,
            y_min: cy - hh,
            y_max: cy + hh,
        }
    }

    pub fn pan(&self, dx: f64, dy: f64) -> Self {
        Self {
            x_min: self.x_min + dx,
            x_max: self.x_max + dx,
            y_min: self.y_min + dy,
            y_max: self.y_max + dy,
        }
    }

    pub fn map_pixel(&self, px: usize, py: usize, w: usize, h: usize) -> (f64, f64) {
        let denom_x = (w - 1).max(1) as f64;
        let denom_y = (h - 1).max(1) as f64;

        let a = self.x_min + (px as f64 / denom_x) * (self.x_max - self.x_min);
        let b = self.y_min + (py as f64 / denom_y) * (self.y_max - self.y_min);
        (a, b)
    }
}

impl Default for ViewWindow {
    fn default() -> Self {
        Self {
            x_min: -2.0,
            x_max: 2.0,
            y_min: -2.0,
            y_max: 2.0,
        }
    }
}
