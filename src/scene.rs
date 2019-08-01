use std::rc::Rc;
use crate::surfaces::Surface;
use crate::camera::Camera;
use crate::color::Color;
use crate::vector::Vector;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Ray {
  pub start: Vector,
  pub dir: Vector,
}

pub struct Intersection {
  pub item: Rc<dyn Item>,
  pub ray: Ray,
  pub dist: f64,
}

pub trait Item {
  fn intersect(&self, ray: Ray) -> Option<Intersection>;
  fn normal(&self, pos: Vector) -> Vector;
  fn surface(&self) -> Rc<dyn Surface>;
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Light {
  pub pos: Vector,
  pub color: Color,
}

pub struct Scene {
  pub things: Vec<Box<dyn Item>>,
  pub lights: Vec<Light>,
  pub camera: Camera,
}
