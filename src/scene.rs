use crate::camera::Camera;
use crate::color::Color;
use crate::vector::Vector;

#[derive(PartialEq, Debug, Clone,Copy)]
pub struct Ray {
  pub start: Vector,
  pub dir: Vector,
}

// #[derive(PartialEq, Debug, Clone)]
pub struct Intersection {
  pub item: Box<dyn Item>,
  pub ray: Ray,
  pub dist: f64,
}

#[derive(PartialEq, Debug, Clone,Copy)]
pub struct Surface;
// impl Surface {
//   fn diffuse(pos: Vector) -> Color;
//   fn specular(pos: Vector) -> Color;
//   fn reflect(pos: Vector) -> f64;
//   fn roughness(&self) -> f64;
// }

pub trait Item {
  fn intersect(&self, ray: Ray) -> Option<Intersection>;
  fn normal(&self, pos: Vector) -> Vector;
  fn surface(&self) -> Surface;
}

#[derive(PartialEq, Debug, Clone,Copy)]
pub struct Light {
  pub pos: Vector,
  pub color: Color,
}

pub struct Scene {
  pub things: Vec<Box<dyn Item>>,
  pub lights: Vec<Light>,
  pub camera: Camera,
}
