use std::env;
use std::process;
use std::io::{self, Write};

use crossbeam_utils::thread;

use mandelbrot::{
    parse_pair,
    parse_complex,
    pixel_to_point,
    render,
    write_image
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

    let threads = 8;
    let rows_per_band = bounds.1 / threads + 1;

    let bands: Vec<&mut [u8]> = pixels
        .chunks_mut(rows_per_band * bounds.0)
        .collect();

    thread::scope(|s| {
        for (i, band) in bands.into_iter().enumerate() {
            let top = rows_per_band * i;
            let height = band.len() / bounds.0;
            let band_bounds = (bounds.0, height);
            let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
            let band_lower_right = pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

            s.spawn(move |_| {
                render(band, band_bounds, band_upper_left, band_lower_right); 
            });
        }
    }).unwrap();
        
    render(&mut pixels, bounds, upper_left, lower_right);

    write_image(&args[1], &pixels, bounds).expect("Error writing PNG file");
}
