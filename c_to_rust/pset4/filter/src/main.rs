use std::fs;

use anyhow::{Result as AnyhowResult, bail};
use image::{DynamicImage, GenericImage, GenericImageView, ImageReader, Rgba};
use rust50::get_string;

const BOX_BLUR_RADIUS: i32 = 1;

fn main() -> AnyhowResult<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        bail!("Invalid input\n\nUsage: filter [filter flag (b, g, r, s)] input.bmp output.bmp\n");
    }
    let filter: Vec<char> = args[1].to_lowercase().chars().collect();
    if filter.len() != 1 {
        bail!("Only one filter allowed.")
    }
    if !['b', 'g', 'r', 's', 'e'].contains(&filter[0]) {
        bail!("Invalid filter.")
    }

    let src = ImageReader::open(&args[2])?.decode()?;

    match fs::exists(&args[3]) {
        Ok(true) => {
            println!(
                "File {} already exists. Do you want to overwrite? [y/N]",
                &args[3]
            );
            if !["y", "yes"].contains(&get_string("").to_lowercase().as_str()) {
                println!("Canceled");
                return Ok(());
            }
        }
        Err(e) => bail!(e),
        _ => (),
    }

    println!("\nProcessing...");

    let out = match filter[0] {
        'b' => blur(src),
        'g' => grayscale(src),
        'r' => reflect(src),
        's' => sepia(src),
        'e' => edge(src),
        _ => bail!("Invalid filter"),
    };

    println!("\nSaving...");
    out.save(&args[3])?;

    println!("Done!");
    Ok(())
}

fn reflect(src: DynamicImage) -> DynamicImage {
    let mut out = src.clone();

    for (x, y, pixel) in src.pixels() {
        out.put_pixel((src.width() - 1) - x, y, pixel);
    }
    out
}

fn grayscale(src: DynamicImage) -> DynamicImage {
    let mut out = src.clone();

    for (x, y, pixel) in src.pixels() {
        let average =
            ((pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32) as f64 / 3.0).round() as u8;
        let pixel = Rgba([average, average, average, pixel[3]]);
        out.put_pixel(x, y, pixel);
    }
    out
}

fn sepia(src: DynamicImage) -> DynamicImage {
    let mut out = src.clone();

    for (x, y, pixel) in src.pixels() {
        let original_red = pixel[0] as f64;
        let original_green = pixel[1] as f64;
        let original_blue = pixel[2] as f64;
        let sepia_red =
            (0.393 * original_red + 0.769 * original_green + 0.189 * original_blue).round() as u8;
        let sepia_green =
            (0.349 * original_red + 0.686 * original_green + 0.168 * original_blue).round() as u8;
        let sepia_blue =
            (0.272 * original_red + 0.534 * original_green + 0.131 * original_blue).round() as u8;

        let pixel = Rgba([sepia_red, sepia_green, sepia_blue, pixel[3]]);
        out.put_pixel(x, y, pixel);
    }
    out
}

fn blur(src: DynamicImage) -> DynamicImage {
    let mut out = src.clone();

    for (x, y, _) in src.pixels() {
        let pixel_box = get_box(&src, x, y, BOX_BLUR_RADIUS, false);

        let pixel_count = pixel_box.len() as f64;
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        let mut a = 0;
        for pixel in pixel_box {
            r += pixel[0] as u32;
            g += pixel[1] as u32;
            b += pixel[2] as u32;
            a += pixel[3] as u32;
        }
        let (r, g, b, a) = (
            (r as f64 / pixel_count).round() as u8,
            (g as f64 / pixel_count).round() as u8,
            (b as f64 / pixel_count).round() as u8,
            (a as f64 / pixel_count).round() as u8,
        );

        let pixel = Rgba([r, g, b, a]);
        out.put_pixel(x, y, pixel);
    }
    out
}

fn get_box(
    src: &DynamicImage,
    x: u32,
    y: u32,
    dist_from_center: i32,
    include_outside: bool, // Includes pixels outside the image as black, transparent pixels.
) -> Vec<Rgba<u8>> {
    let mut pixel_box: Vec<Rgba<u8>> = Vec::new();

    for y in (y as i32 - dist_from_center)..=(y as i32 + dist_from_center) {
        for x in (x as i32 - dist_from_center)..=(x as i32 + dist_from_center) {
            if !(0..src.width()).contains(&(x as u32)) || !(0..src.height()).contains(&(y as u32)) {
                if include_outside {
                    pixel_box.push(Rgba([0; 4]));
                }
                continue;
            }
            pixel_box.push(src.get_pixel(x as u32, y as u32));
        }
    }

    pixel_box
}

// const GX: [i32; 9] = [1,1,1,0,0,0,1,1,1];
const GX: [i32; 9] = [-1, 0, 1, -2, 0, 2, -1, 0, 1];

// const GY: [i32; 9] = [1,1,1,0,0,0,1,1,1];
const GY: [i32; 9] = [-1, -2, -1, 0, 0, 0, 1, 2, 1];

fn edge(src: DynamicImage) -> DynamicImage {
    let mut out = src.clone();

    for (x, y, _) in src.pixels() {
        // TODO: include pixels outside the image as black pixels.
        let pixel_box = get_box(&src, x, y, 1, true);

        let sobel_pixel = get_sobel_pixel(
            get_sobel_sum(&pixel_box, &GX),
            get_sobel_sum(&pixel_box, &GY),
        );
        out.put_pixel(x, y, sobel_pixel);
    }
    out
}

fn get_sobel_sum(pixel_box: &Vec<Rgba<u8>>, weights: &[i32; 9]) -> Vec<i32> {
    let mut r_sum = 0;
    let mut g_sum = 0;
    let mut b_sum = 0;

    for (index, pixel) in pixel_box.iter().enumerate() {
        r_sum += pixel[0] as i32 * weights[index];
        g_sum += pixel[1] as i32 * weights[index];
        b_sum += pixel[2] as i32 * weights[index];
    }

    vec![r_sum, g_sum, b_sum]
}

fn get_sobel_pixel(sobel_x: Vec<i32>, sobel_y: Vec<i32>) -> Rgba<u8> {
    let mut pixel = [
        (((sobel_x[0]).pow(2) + (sobel_y[0]).pow(2)) as f64).sqrt(),
        (((sobel_x[1]).pow(2) + (sobel_y[1]).pow(2)) as f64).sqrt(),
        (((sobel_x[2]).pow(2) + (sobel_y[2]).pow(2)) as f64).sqrt(),
    ];

    for color in &mut pixel {
        if *color > 255.0 {
            *color = 255.0;
            continue;
        }
        *color = color.round();
    }

    Rgba([pixel[0] as u8, pixel[1] as u8, pixel[2] as u8, 255])
}
