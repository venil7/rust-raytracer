use crate::color::Color;
use crate::vector::Vector;

pub trait Surface {
  fn diffuse(&self, pos: &Vector) -> Color;
  fn specular(&self, pos: &Vector) -> Color;
  fn reflect(&self, pos: &Vector) -> f64;
  fn roughness(&self) -> f64;
}

pub struct Shiny;
impl Surface for Shiny {
  fn diffuse(&self, _pos: &Vector) -> Color {
    Color::white()
  }
  fn specular(&self, _pos: &Vector) -> Color {
    Color::grey()
  }
  fn reflect(&self, _pos: &Vector) -> f64 {
    0.7
  }
  fn roughness(&self) -> f64 {
    250.
  }
}

pub struct Checker;
impl Surface for Checker {
  fn diffuse(&self, pos: &Vector) -> Color {
    if (pos.2.floor() + pos.0.floor()) % 2. != 0. {
      Color::white()
    } else {
      Color::black()
    }
  }
  fn specular(&self, _pos: &Vector) -> Color {
    Color::white()
  }
  fn reflect(&self, pos: &Vector) -> f64 {
    if (pos.2.floor() + pos.0.floor()) % 2. != 0. {
      0.1
    } else {
      0.7
    }
  }
  fn roughness(&self) -> f64 {
    150.
  }
}
