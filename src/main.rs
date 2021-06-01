#[macro_use]
extern crate clap;
use clap::{App, Arg};
use image::{ImageFormat, Rgb, RgbImage};
use num_complex::Complex;
use std::{iter, process};

fn in_mandelbrot(c: Complex<f64>, depth: usize) -> bool {
    !iter::successors(Some(Complex::new(0.0, 0.0)), |z| Some(z.powu(2) + c))
        .take(depth)
        .any(|z| z.norm() > 2.0)
}

fn c_from_coords(x: u32, y: u32, w: u32, h: u32) -> Complex<f64> {
    if h >= w {
        Complex::new(
            4.0 * (x as f64 / w as f64 - 0.5),
            2.0 / w as f64 * (h as f64 - 2.0 * y as f64),
        )
    } else {
        Complex::new(
            2.0 / h as f64 * (2.0 * x as f64 - w as f64),
            4.0 * (y as f64 / h as f64 - 0.5),
        )
    }
}

fn main() {
    let matches = App::new("mandelbrot")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Produces pictures of the mandelbrot set.")
        .arg(
            Arg::with_name("HEIGHT")
                .short("h")
                .long("height")
                .takes_value(true)
                .help("The height of the image, in pixels."),
        )
        .arg(
            Arg::with_name("WIDTH")
                .short("w")
                .long("width")
                .takes_value(true)
                .help("The width of the image, in pixels."),
        )
        .arg(
            Arg::with_name("DEPTH")
                .short("d")
                .long("depth")
                .takes_value(true)
                .help("The maximum depth of the recursion."),
        )
        .arg(
            Arg::with_name("OUT")
                .required(true)
                .help("The name of the file created."),
        )
        .get_matches();

    let h = matches.value_of("HEIGHT").map_or_else(
        || 1080,
        |vdim| {
            vdim.parse::<u32>().unwrap_or_else(|_| {
                eprintln!("Error: Unexpected value. Expected [u32], found {}.", vdim);
                process::exit(1);
            })
        },
    );
    let w = matches.value_of("WIDTH").map_or_else(
        || 1080,
        |hdim| {
            hdim.parse::<u32>().unwrap_or_else(|_| {
                eprintln!("Error: Unexpected value. Expected [u32], found {}.", hdim);
                process::exit(1);
            })
        },
    );
    let depth = matches.value_of("DEPTH").map_or_else(
        || 64,
        |depth| {
            depth.parse::<usize>().unwrap_or_else(|_| {
                eprintln!(
                    "Error: Unexpected value. Expected [usize], found {}.",
                    depth
                );
                process::exit(1);
            })
        },
    );
    let out = matches.value_of("OUT").unwrap();
    let mut image = RgbImage::new(w, h);
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        if in_mandelbrot(c_from_coords(x, y, w, h), depth) {
            *pixel = Rgb([255, 255, 255]);
        } else {
            *pixel = Rgb([0, 0, 0]);
        }
    }
    if let Err(e) = image.save_with_format(out, ImageFormat::Png) {
        eprintln!("Error saving image: {}.", e);
        process::exit(1);
    }
}
