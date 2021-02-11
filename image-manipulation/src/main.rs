extern crate clap;
mod blur;
use blur::blur;
use core::f32;
mod brighten;
use brighten::brighten;
mod invert;
use invert::invert;
mod rotate;
use rotate::rotate;
mod grayscale;
use grayscale::grayscale;
mod crop;
use clap::{App, Arg, ArgMatches};
use crop::crop;
use image::DynamicImage;

fn manipulate(matches: &ArgMatches, image: &mut DynamicImage) -> bool {
    let mut did_something = false;

    if matches.is_present("blur") {
        let amount_str = matches.value_of("blur").unwrap();
        match amount_str.parse::<f32>() {
            Ok(amount) => {
                println!("Blurring image by {}...", amount);
                blur(image, amount);
                did_something = true;
            }
            Err(_) => {
                println!("Invalid blur amount: {}", amount_str);
            }
        };
    }

    if matches.is_present("brighten") {
        let amount_str = matches.value_of("brighten").unwrap();
        match amount_str.parse::<i32>() {
            Ok(amount) => {
                println!("Brightening image by {}...", amount);
                brighten(image, amount);
                did_something = true;
            }
            Err(_) => {
                println!("Invalid brighten amount: {}", amount_str);
            }
        };
    }

    if matches.is_present("crop") {
        let v: Vec<u32> = matches
            .values_of("crop")
            .unwrap()
            .map(|val| val.parse::<u32>().unwrap())
            .collect();

        println!(
            "Cropping image from {},{} with dimensions {},{}...",
            v[0], v[1], v[2], v[3]
        );
        crop(image, (v[0], v[1], v[2], v[3]));
        did_something = true;
    }

    if matches.is_present("rotate") {
        let amount_str = matches.value_of("rotate").unwrap();
        match amount_str.parse::<i32>() {
            Ok(mut amount) => {
                if (amount as f32 / 90.0) % 1.0 != 0.0 {
                    println!("Invalid rotate amount: {} (can only go in steps of 90)", amount_str);
                } else {
                    // to keep in bounds of 0 to 360
                    while amount < 0 {
                        amount += 360;
                    }
                    while amount > 360 {
                        amount -= 360;
                    }

                    println!("Rotating image by {}...", amount_str);
                    rotate(image, amount);
                    did_something = true;
                }
            }
            Err(_) => {
                println!(
                    "Invalid rotate amount: {} (can only go in steps of 90)",
                    amount_str
                );
            }
        };
    }

    if matches.is_present("invert") {
        println!("Inverting image...");
        invert(image);
        did_something = true;
    }

    if matches.is_present("grayscale") {
        println!("Grayscaling image...");
        grayscale(image);
        did_something = true;
    }

    return did_something;
}

fn main() {
    let matches = App::new("Maki Image Manipulation Program")
        .version("0.1.0")
        .author("Maki <mxmcube@gmail.com>")
        .about("Does image manipulation!")
        .arg(Arg::with_name("input file").required(true))
        .arg(Arg::with_name("output file").required(true))
        .arg(
            Arg::with_name("blur")
                .long("blur")
                .takes_value(true)
                .help("Blurs the image"),
        )
        .arg(
            Arg::with_name("brighten")
                .long("brighten")
                .takes_value(true)
                .help("Brightens the image"),
        )
        .arg(
            Arg::from_usage("--crop <x> <y> <w> <h>")
                .long("crop")
                .takes_value(true)
                .number_of_values(4)
                .required(false)
                .help("Crops the image"),
        )
        .arg(
            Arg::with_name("rotate")
                .long("rotate")
                .takes_value(true)
                .help("Rotates the image"),
        )
        .arg(
            Arg::with_name("invert")
                .long("invert")
                .help("Inverts the image"),
        )
        .arg(
            Arg::with_name("grayscale")
                .long("grayscale")
                .help("Grayscales the image"),
        )
        .get_matches();

    let input_filename = matches.value_of("input file").unwrap();
    let output_filename = matches.value_of("output file").unwrap();

    let mut image = match image::open(input_filename) {
        Ok(i) => {
            println!("Using input image: {}", input_filename);
            i
        }
        Err(_) => {
            println!("Could not find image: {}", input_filename);
            std::process::exit(1);
        }
    };

    if manipulate(&matches, &mut image) {
        match image.save(output_filename) {
            Ok(_) => {
                println!("Output image saved to: {}", output_filename);
            }
            Err(_) => {
                println!("Failed to save image to: {}", output_filename);
            }
        }
    } else {
        println!("However, there was nothing to do!")
    }
}
