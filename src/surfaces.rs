use crate::color::Color;
use crate::vector::Vector;

pub trait Surface {
  fn diffuse(&self, pos: Vector) -> Color;
  fn specular(&self,pos: Vector) -> Color;
  fn reflect(&self,pos: Vector) -> f64;
  fn roughness(&self) -> f64;
}