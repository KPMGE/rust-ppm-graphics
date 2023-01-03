mod image;
mod circle;

use image::Image;
use circle::Circle;

fn main() {
  const WIDTH: usize = 900;
  const HEIGHT: usize = 600;
  draw_japan_flag(WIDTH, HEIGHT);
}

pub fn draw_japan_flag(width: usize, height: usize) {
  let mut img = Image::new(width, height);

  let white = 0xfffffff;
  let red = 0x00ff;
  let circle_at_center = Circle::new(
    (img.width / 2) as i32, 
    (img.height / 2) as i32, 
    80
  );

  img.fill(white);
  img.draw_circle(&circle_at_center, red);
  img.save_as_ppm("out.ppm");
}
