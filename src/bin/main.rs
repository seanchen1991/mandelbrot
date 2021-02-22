use std::env;
use std::process;
use std::io::{self, Write};

use rayon::prelude::*;

use mandelbrot::{
    parse_pair,
    parse_complex,
    render,
    write_image,
    pixel_to_point
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        writeln!(io::stderr(), "Usage: mandelbrot [file] [pixels] [upper_left] [lower_right]").unwrap();
        writeln!(io::stderr(), "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20", args[0]).unwrap();
        process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("Error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("Error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("Error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];
    
    let bands: Vec<(usize, &mut [u8])> = pixels
        .chunks_mut(bounds.0)
        .enumerate()
        .collect();

    bands.into_par_iter()
        .for_each(|(i, band)| {
            let top = i;
            let band_bounds = (bounds.0, 1);
            let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
            let band_lower_right = pixel_to_point(bounds, (bounds.0, top + 1), upper_left, lower_right);
            render(band, band_bounds, band_upper_left, band_lower_right);
        });

    write_image(&args[1], &pixels, bounds).expect("Error writing PNG file");
}
