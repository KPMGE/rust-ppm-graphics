// AABBGGRR
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

fn fill_image(pixels: &mut Vec<Pixel>, size: usize, color: u32) {
    for _ in 0..size {
        pixels.push(Pixel {
            r: (color & 0xff) as u8,
            g: ((color >> 8) & 0xff) as u8,
            b: ((color >> 16) & 0xff) as u8,
        })
    }
}

fn save_image_as_ppm(file_path: &str, pixels: &Vec<Pixel>, width: usize, height: usize) {
    let mut file = File::create(file_path).unwrap();

    if let Err(e) = write!(file, "P6\n{} {} 255\n", width, height) {
        eprintln!("Couldn't write to file: {}", e);
    }

    for pixel in pixels {
        let bytes: [u8; 3] = [pixel.r, pixel.g, pixel.b];
        if let Err(e) = file.write_all(&bytes) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}

fn main() {
    const WIDTH: usize = 600;
    const HEIGHT: usize = 400;
    const IMAGE_SIZE: usize = WIDTH * HEIGHT;
    let mut pixels: Vec<Pixel> = Vec::with_capacity(IMAGE_SIZE);

    fill_image(&mut pixels, IMAGE_SIZE, 0x00ff);
    save_image_as_ppm("out.ppm", &pixels, WIDTH, HEIGHT);
}
