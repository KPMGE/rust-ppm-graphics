// AABBGGRR
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Pixel {
  r: u8,
  g: u8,
  b: u8,
}

#[derive(Debug)]
struct Image {
  pixels: Vec<Pixel>,
  width: usize,
  height: usize
}

impl Image {
  fn new(width: usize, height: usize) -> Self {
    Image { 
      pixels: Vec::with_capacity(width * height),
      width,
      height
    }
  }

  fn size(&self) -> usize {
    self.width * self.height
  }
}

#[derive(Debug)]
struct Circle {
  x: i32, 
  y: i32,
  radius: usize
}

impl Circle {
  fn new(x: i32, y: i32, radius: usize) -> Self {
    Circle { x, y, radius }
  }
}

fn main() {
  const WIDTH: usize = 600;
  const HEIGHT: usize = 400;

  let mut img = Image::new(WIDTH, HEIGHT);

  fill_image(&mut img, 0xfffffff);
  draw_circle(&mut img, Circle::new((WIDTH / 2) as i32, (HEIGHT / 2) as i32, 80), 0x00ff);
  save_image_as_ppm("out.ppm", img);
}

fn draw_circle(img: &mut Image, c: Circle, color: u32) {
  for i in 0..img.width {
    for j in 0..img.height {
      let x = (i as i32) - c.x;
      let y = (j as i32) - c.y;

      if ((c.radius*c.radius) as i32) > x*x + y*y {
        let pos = j * img.width + i;
        img.pixels.remove(pos as usize);
        img.pixels.insert(pos as usize,  Pixel{
          r: (color & 0xff) as u8,
          g: ((color >> 8) & 0xff) as u8,
          b: ((color >> 16) & 0xff) as u8,
        });
      }
    }
  }
}

fn fill_image(img: &mut Image, color: u32) {
  for _ in 0..img.size() {
    img.pixels.push(Pixel {
      r: (color & 0xff) as u8,
      g: ((color >> 8) & 0xff) as u8,
      b: ((color >> 16) & 0xff) as u8,
    })
  }
}

fn save_image_as_ppm(file_path: &str, img: Image) {
  let mut file = File::create(file_path).unwrap();

  if let Err(e) = write!(file, "P6\n{} {} 255\n", img.width, img.height) {
    eprintln!("Couldn't write to file: {}", e);
  }

  for pixel in img.pixels {
    let bytes: [u8; 3] = [pixel.r, pixel.g, pixel.b];
    if let Err(e) = file.write_all(&bytes) {
      eprintln!("Couldn't write to file: {}", e);
    }
  }
}
