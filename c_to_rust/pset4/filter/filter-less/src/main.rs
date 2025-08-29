use std::fs;

use anyhow::{Result as AnyhowResult, bail};
use image::{DynamicImage, GenericImage, GenericImageView, ImageReader, Rgba};
use my_library::get_string;

const BLUR_RADIUS: u32 = 1;

fn main() -> AnyhowResult<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        bail!("Invalid input\n\nUsage: filter [filter flag (b, g, r, s)] input.bmp output.bmp\n");
    }
    let filter: Vec<char> = args[1].to_lowercase().chars().collect();
    if filter.len() != 1 {
        bail!("Only one filter allowed.")
    }
    if !['b', 'g', 'r', 's'].contains(&filter[0]) {
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
        'g' => bail!("TODO"),
        'r' => bail!("TODO"),
        's' => bail!("TODO"),
        _ => bail!("Invalid filter"),
    };

    println!("\nSaving...");
    out.save(&args[3])?;

    println!("Done!");
    Ok(())
}

fn blur(src: DynamicImage) -> DynamicImage {
    let mut out = src.clone();

    for (x, y, _) in out.clone().pixels() {
        // println!("\rProgress: ({}, {})", &x + 1, &y + 1);
        // std::io::stdout().flush()?;

        let mut pixel_buf: Vec<Rgba<u8>> = Vec::new();
        for x in x.saturating_sub(BLUR_RADIUS)..=x.saturating_add(BLUR_RADIUS) {
            if !(0..out.width()).contains(&x) {
                continue;
            }
            for y in y.saturating_sub(BLUR_RADIUS)..=y.saturating_add(BLUR_RADIUS) {
                if !(0..out.height()).contains(&y) {
                    continue;
                }
                // println!("buffering: {}, {}", x + 1, y + 1);
                // std::io::stdout().flush()?;
                pixel_buf.push(src.get_pixel(x, y));
            }
        }
        // println!("\n");

        let pixel_count = pixel_buf.len() as f64;
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        let mut a = 0;
        for pixel in pixel_buf {
            r += pixel[0] as u64;
            g += pixel[1] as u64;
            b += pixel[2] as u64;
            a += pixel[3] as u64;
        }
        let (r, g, b, a) = (
            (r as f64 / pixel_count).round() as u8,
            (g as f64 / pixel_count).round() as u8,
            (b as f64 / pixel_count).round() as u8,
            (a as f64 / pixel_count).round() as u8,
        );

        // if x < (out.width() / 2) && y < (out.height() / 2) {
        //     (r, g, b) = (0, 0, 0);
        // }

        // println!("Wrinting: {}, {}\n", x + 1, y + 1);
        // out.set_pixel(x, y, Pixel { r: r, g: g, b: b });
        let pixel = Rgba([r, g, b, a]);
        out.put_pixel(x, y, pixel);
    }
    out
}
