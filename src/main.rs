use std::env;
use std::path::Path;
use image::{GenericImageView, Rgba};
use reqwest::blocking::get;
use url::Url;

fn main() {

    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("DAISKII: Please enter a valid Path or URL!");
        return;
    }

    let url = &args[1];

    if is_path(url) {
        draw_from_path(url)
    } else if is_url(url) {
        draw_from_url(url)
    } else {
        println!("DAISKII: Please check the Path or URL and try again.");
        return;
    }
    
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

fn draw_from_url(url: &String) {
    
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

    let size;

    match env::args().nth(2) {
        Some(quality_str) => {
            let quality = quality_str.to_lowercase();
            match quality.as_str() {
                "very_low" => size = 50,
                "low" => size = 200,
                "medium" | "med" => size = 300,
                "high" => size = 500,
                "very_high" => size = 700,
                _ => {
                    println!("DAISKII: Invalid quality option: {}! Please keep it empty or use: 'very_low', 'low', 'medium', 'high', 'very_high' ", quality);
                    return;
                },
            }
        },
        None => size = 100,
    }



    let img = match image::load_from_memory(&bytes) {
        Ok(img) => img,
        Err(_err) => {
            eprintln!("DAISKII: Error loading image! Please check the URL and try again.");
            return;
        }
    };

    let img = img.resize(size, size, image::imageops::FilterType::Nearest);

    let img = img.grayscale();

    let (width, height) = img.dimensions();

    let ascii_chars = ['@', '#', 'S', '%', '?', '*', '+', ';', ':', ',', '.'];

    for y in 0..height {
        for x in 0..width {

            let pixel = img.get_pixel(x, y);
            let brightness = brightness(pixel);

            let ascii_index = ((brightness as f32 / 255.0) * (ascii_chars.len() as f32 - 1.0)).round() as usize;
            print!("  {}", ascii_chars[ascii_index]);

        }
        println!();
    }
}

fn draw_from_path(url: &String){

    let img = match image::open(url) {
        Ok(img) => img,
        Err(_err) => {
            eprintln!("DAISKII: Error loading image! Please check the provided Path and try again.");
            return;
        }
    };

    let size;

    match env::args().nth(2) {
        Some(quality_str) => {
            let quality = quality_str.to_lowercase();
            match quality.as_str() {
                "very_low" => size = 50,
                "low" => size = 200,
                "medium" | "med" => size = 300,
                "high" => size = 500,
                "very_high" => size = 700,
                _ => {
                    println!("DAISKII: Invalid quality option: {}! Please keep it empty or use: 'very_low', 'low', 'medium', 'high', 'very_high' ", quality);
                    return;
                },
            }
        },
        None => size = 100,
    }

    let img = img.resize(size, size, image::imageops::FilterType::Nearest);

    let img = img.grayscale();

    let (width, height) = img.dimensions();

    let ascii_chars = ['@', '#', 'S', '%', '?', '*', '+', ';', ':', ',', '.'];

    for y in 0..height {
        for x in 0..width {

            let pixel = img.get_pixel(x, y);
            let brightness = brightness(pixel);

            let ascii_index = ((brightness as f32 / 255.0) * (ascii_chars.len() as f32 - 1.0)).round() as usize;
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
  