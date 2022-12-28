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

  fn fill(&mut self, color: u32) {
    for _ in 0..self.size() {
      self.pixels.push(Pixel::new(color));
    }
  }

  fn draw_circle(&mut self, circle: &Circle, color: u32) {
    for i in 0..self.width {
      for j in 0..self.height {
        let x = (i as i32) - circle.x;
        let y = (j as i32) - circle.y;

        if circle.radius.pow(2) as i32 > x.pow(2) + y.pow(2) {
          let pos = j * self.width + i;
          if pos < self.pixels.len() {
            self.pixels.remove(pos as usize);
            self.pixels.insert(pos as usize, Pixel::new(color));
          }
        }
      }
    }
  }

  fn save_as_ppm(&self, file_path: &str) {
    let mut file = File::create(file_path).unwrap();

    if let Err(e) = write!(file, "P6\n{} {} 255\n", self.width, self.height) {
      eprintln!("Couldn't write to file: {}", e);
    }

    for pixel in self.pixels.as_slice() {
      if let Err(e) = file.write_all(&pixel.bytes()) {
        eprintln!("Couldn't write to file: {}", e);
      }
    }
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

  draw_japan_flag(WIDTH, HEIGHT);
}

fn draw_japan_flag(width: usize, height: usize) {
  let mut img = Image::new(width, height);

  let white = 0xfffffff;
  let red = 0x00ff;
  let circle_at_center = Circle::new((img.width / 2) as i32, (img.height / 2) as i32, 80);

  img.fill(white);
  img.draw_circle(&circle_at_center, red);
  img.save_as_ppm("out.ppm");
}
