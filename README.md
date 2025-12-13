## PPM File Generator
Draw the mandelbrot set in a PPM file.

```bash
cargo run
```

```bash
cargo build --release
```

```
Draws Mandelbrot set on a .ppm file

Usage: mandelbrot-ppm-rs [OPTIONS]

Options:
      --width <WIDTH>              Output image width [default: 9600]
      --height <HEIGHT>            Output image height [default: 5400]
      --max-iter <MAX_ITER>        Max iteration to determine if evolution of complex number c under z = z^2 + c is bounded [default: 100]
  -o, --output-path <OUTPUT_PATH>  Output file path [default: output.ppm]
  -h, --help                       Print help
  -V, --version                    Print version
```

See the PPM specification here: https://netpbm.sourceforge.net/doc/ppm.html

![Mandelbrot set PPM output](docs/output.webp)