use std::fs::File;
use std::str::FromStr;

use num::Complex;
use image::ColorType;
use image::png::PNGEncoder;

/// Determine if `c` is in the Mandelbrot set or not, based in part
/// on the `limit` parameter, which specifies how many "attempts"
/// the program gets to figure it out.
/// 
/// If `c` is not a member, returns `Some(i)` where `i` is the number
/// of iterations it took for `c` to leave the circle of radius 2 centered
/// at the origin. If `c` is a member of the set, i.e., if we reached the 
/// iteration limit without being able to prove that `c` is _not_ a member,
/// return `None`
fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };

    for i in 0..limit {
        z = z * z + c;

        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None
}

/// Parse string `s` as a coordinate pair, e.g., `"400x600"` or `"1.0,0.5"`.
/// 
/// Specifically, `s` should have the form <left><sep><right>, where <sep> is
/// the character given by the `separator` argument, and <left> and <right> are
/// both strings that can be parsed by `T::from_str`.
/// 
/// If `s` has the proper form, return `Some(x, y)`. If it doesn't, return None.
pub fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    if let Some(index) = s.find(separator) {
        match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        }
    } else {
        None
    }
}

/// Parse a pair of floating-point numbers separated by a comma as a
/// complex number.
pub fn parse_complex(s: &str) -> Option<Complex<f64>> {
    if let Some((re, im)) = parse_pair(s, ',') {
        Some(Complex { re, im })
    } else {
        None
    }
}

/// Given the row and column of a pixel in the output image, return the
/// corresponding point on the complex plane.
///
/// `bounds` is a pair giving the width and height of the image in pixels.
/// `pixel` is a (column, row) pair indicating a particular pixel in that image.
/// The `upper_left` and `lower_right` parameters are points on the complex
/// plane designating the area our image covers.
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>
) -> Complex<f64> {
    let (width, height) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

/// Render a rectangle of the Mandelbrot set into a buffer of pixels.
/// 
/// The `bounds` arguments gives the width and height of the `pixels` buffer,
/// which holds one grayscale pixel per byte. The `upper_left` and `lower_right`
/// arguments specify points on the complex plane corresponding to the upper-
/// left and lower-right corners of the pixel buffer.
pub fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>
) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for col in 0..bounds.0 {
            let point = pixel_to_point(bounds, (col, row), upper_left, lower_right);

            pixels[row * bounds.0 + col] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8
            };
        }
    }
}

/// Write the contents of the `pixel` buffer, whose dimensions are given
/// by `bounds`, to the specified file.
pub fn write_image(
    filename: &str, 
    pixels: &[u8], 
    bounds: (usize, usize)
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);

    encoder.encode(
        &pixels, 
        bounds.0 as u32, 
        bounds.1 as u32, 
        ColorType::Gray(8)
    )?;

    Ok(())
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("",        ','), None);
    assert_eq!(parse_pair::<i32>("10,",     ','), None);
    assert_eq!(parse_pair::<i32>(",10",     ','), None);
    assert_eq!(parse_pair::<i32>("10,20",   ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x",    'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));   
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"), Some(Complex { re: 1.25, im: -0.0625 }));
    assert_eq!(parse_complex(",-0.0625"), None);
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100, 100), (25, 75),
                              Complex { re: -1.0, im:  1.0 },
                              Complex { re:  1.0, im: -1.0 }),
               Complex { re: -0.5, im: -0.5 }); 
}
