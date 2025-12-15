## Multibrot PPM File Generator
Draws a multibrot image into a .ppm file.

```bash
cargo run
```

```bash
cargo build --release
```

```
Usage: multibrot-ppm-rs [OPTIONS]

Options:
      --width <WIDTH>              Output image width [default: 1000]
      --height <HEIGHT>            Output image height [default: 1000]
  -o, --output-path <OUTPUT_PATH>  Output file path [default: output.ppm]
  -z, --zoomed                     Render a zoomed-in view centered on the origin
  -n, --n <N>                      Multibrot n, where z -> z^n + c is used for drawing the set [default: 2]
  -h, --help                       Print help
  -V, --version                    Print version
```

## Library Usage
You can also use the Multibrot raster generator directly from Rust.

To use the multibrot set raster generator, build the raster and use the `PPMImageBuffer` to write to a .ppm file.

```rust
use multibrot_ppm_rs::ppm::PPMImageBuffer;
use multibrot_ppm_rs::raster::{multibrot::Multibrot, RasterGenerator, ViewWindow};

fn main() {
  let viewwindow = ViewWindow::full();

  let raster = Multibrot::new_with_view(3.0, 100, viewwindow)
      .unwrap()
      .generate(1000, 1000);

  PPMImageBuffer::new(1000, 1000, raster)
      .expect("Error creating ppm buffer")
      .write_to_ppm("output.ppm")
      .expect("Error creating ppm file");
}
```

![Multibrot set PPM output](docs/output.webp)

## References
- PPM specification: https://netpbm.sourceforge.net/doc/ppm.html
- Escape time algorithm for drawing multibrot set: https://en.wikipedia.org/wiki/Multibrot_set#Rendering_images
