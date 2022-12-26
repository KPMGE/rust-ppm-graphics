// AABBGGRR
use std::fs::File;
use std::io::prelude::*;

struct Pixel {
  r: u8,
  g: u8,
  b: u8,
}

impl Pixel {
  fn new(color: u32) -> Self {
    Pixel {
      r: (color & 0xff) as u8,
      g: ((color >> 8) & 0xff) as u8,
      b: ((color >> 16) & 0xff) as u8,
    }
  }

  fn bytes(&self) -> [u8; 3]  {
    [self.r, self.g, self.b]
  } 
}

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
  draw_japan_flag(&mut img);
}

fn draw_circle(img: &mut Image, c: Circle, color: u32) {
  for i in 0..img.width {
    for j in 0..img.height {
      let x = (i as i32) - c.x;
      let y = (j as i32) - c.y;

      if c.radius.pow(2) as i32 > x.pow(2) + y.pow(2) {
        let pos = j * img.width + i;
        img.pixels.remove(pos as usize);
        img.pixels.insert(pos as usize, Pixel::new(color));
      }
    }
  }
}

fn fill_image(img: &mut Image, color: u32) {
  for _ in 0..img.size() {
    img.pixels.push(Pixel::new(color));
  }
}

fn save_image_as_ppm(file_path: &str, img: &mut Image) {
  let mut file = File::create(file_path).unwrap();

  if let Err(e) = write!(file, "P6\n{} {} 255\n", img.width, img.height) {
    eprintln!("Couldn't write to file: {}", e);
  }

  for pixel in img.pixels.as_slice() {
    if let Err(e) = file.write_all(&pixel.bytes()) {
      eprintln!("Couldn't write to file: {}", e);
    }
  }
}

fn draw_japan_flag(img: &mut Image) {
  fill_image(img, 0xfffffff);
  draw_circle(img, Circle::new((img.width / 2) as i32, (img.height / 2) as i32, 80), 0x00ff);
  save_image_as_ppm("out.ppm", img);
}
