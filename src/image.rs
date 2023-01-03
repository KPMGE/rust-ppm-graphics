use std::fs::File;
use std::io::Write;

use crate::circle::Circle;

struct Pixel {
  pub r: u8,
  pub g: u8,
  pub b: u8,
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

pub struct Image {
  pixels: Vec<Pixel>,
  pub width: usize,
  pub height: usize
}

impl Image {
  pub fn new(width: usize, height: usize) -> Self {
    Image { 
      pixels: Vec::with_capacity(width * height),
      width,
      height
    }
  }

  pub fn size(&self) -> usize {
    self.width * self.height
  }

  pub fn fill(&mut self, color: u32) {
    for _ in 0..self.size() {
      self.pixels.push(Pixel::new(color));
    }
  }

  pub fn draw_circle(&mut self, circle: &Circle, color: u32) {
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

  pub fn save_as_ppm(&self, file_path: &str) {
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
