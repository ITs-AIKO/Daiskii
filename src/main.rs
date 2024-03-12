use clap::{arg, command, ArgAction};
use image::{DynamicImage, GenericImageView, Rgba};
use reqwest::blocking::get;
use std::env;
use std::path::Path;
use url::Url;

fn main() {
    let matches = command!()
        .version("0.1")
        .author("Aiko")
        .about("A way to generate ASCII art from images!")

        .arg(
            arg!(
                [URI] "The URL or the local path of the image you want to create ASCII art of."
            )
            .required(true)
        )

        .arg(
            arg!(
                -q --quality <Quality> "The quality of the generated ASCII art can be 'very low', 'low', 'medium', 'high', or 'very high'."
            )
            .required(false)
        )

        .arg(
            arg!(
                -f --force <ForceBlack> "The generated ASCII will use black instead of gray (Zoom out before using this command for better results.)."
            )
            .required(false)
            .action(ArgAction::SetFalse)
        )

        .arg(
            arg!(
                -p --palette <Palette> "It takes a list of characters, and the generated ASCII art will be based on the provided characters (Zoom out before using this command for better results). You can even use emojis like '😌😊😒' BUT DO IT AT YOUR OWN RISK."
            )
            .required(false)
        )

        .get_matches();

    if let Some(url) = matches.get_one::<String>("URI") {
        if is_it(&url) {
            return;
        }

        let size = match matches.get_one::<String>("quality") {
            Some(quality) => match quality.to_lowercase().as_str() {
                "very_low" | "vl" => 50,
                "low" | "l" => 200,
                "medium" | "med" | "m" => 300,
                "high" | "h" => 500,
                "very_high" | "vh" => 700,
                _ => {
                    println!("DAISKII: Invalid quality option: {}! Please keep it empty or use: 'very_low', 'low', 'medium', 'high', 'very_high' ", quality);
                    return;
                }
            },
            None => 250,
        };

        let black = matches
            .get_one::<bool>("force")
            .map_or(true, |force| !force);

        let ascii_chars: Vec<char> = matches
            .get_one::<String>("palette")
            .map_or(
                if black {
                    String::from("@#S%?+;:,. ") // Use String::from for clarity
                } else {
                    String::from("@#S%?+;:,.") // Add a space for non-black case
                },
                |user_chars| user_chars.to_string(), // Use user input if available
            )
            .chars()
            .collect();

        if is_path(&url) {
            draw_from_path(&url, size, ascii_chars);
        } else if is_url(&url) {
            draw_from_url(&url, size, ascii_chars);
        } else {
            println!("DAISKII: Please check the Path or URL and try again.");
            return;
        }
    }

    0;
}

fn is_url(url: &str) -> bool {
    match Url::parse(url) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn is_path(url: &str) -> bool {
    Path::new(url).exists()
}

fn draw_from_url(url: &String, size: u32, ascii_chars: Vec<char>) {
    let response = match get(url) {
        Ok(response) => response,
        Err(_err) => {
            eprintln!("DAISKII: Invalid URL format! Please check the URL and try again.");
            return;
        }
    };

    let bytes = match response.bytes() {
        Ok(bytes) => bytes,
        Err(_err) => {
            eprintln!("DAISKII: Error reading image bytes! Please check the URL and try again.");
            return;
        }
    };

    let img = match image::load_from_memory(&bytes) {
        Ok(img) => img,
        Err(_err) => {
            eprintln!("DAISKII: Error loading image! Please check the URL and try again.");
            return;
        }
    };

    draw(img, size, ascii_chars)
}

fn draw_from_path(url: &String, size: u32, ascii_chars: Vec<char>) {
    let img = match image::open(url) {
        Ok(img) => img,
        Err(_err) => {
            eprintln!(
                "DAISKII: Error loading image! Please check the provided Path and try again."
            );
            return;
        }
    };

    draw(img, size, ascii_chars);
}

fn draw(img: DynamicImage, size: u32, ascii_chars: Vec<char>) {
    let img = img.resize(size, size, image::imageops::FilterType::Nearest);

    let img = img.grayscale();

    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let brightness = brightness(pixel);

            let ascii_index =
                ((brightness as f32 / 255.0) * (ascii_chars.len() as f32 - 1.0)).round() as usize;
            print!("  {}", ascii_chars[ascii_index]);
        }
        println!();
    }
}

fn brightness(pixel: Rgba<u8>) -> u8 {
    let r = pixel[0] as f32;
    let g = pixel[1] as f32;
    let b = pixel[2] as f32;

    255 - ((0.21 * r + 0.72 * g + 0.07 * b) as u8)
}

fn is_it(url: &String) -> bool {
    if url.to_lowercase() == "lain" {
        let temp = "https://i.imgur.com/GV2Gqfw.png";
        draw_from_url(
            &temp.to_string(),
            300,
            vec!['@', '#', 'S', '%', '?', '+', ';', ':', ',', '.'],
        );
        return true;
    }
    return false;
}
