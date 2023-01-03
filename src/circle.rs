pub struct Circle {
  pub x: i32, 
  pub y: i32,
  pub radius: usize
}

impl Circle {
  pub fn new(x: i32, y: i32, radius: usize) -> Self {
    Circle { x, y, radius }
  }
}
