use crate::scene::Intersection;
use crate::scene::Item;
use crate::scene::Ray;
use crate::scene::Surface;
use crate::vector::Vector;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Plane {
  norm: Vector,
  offset: f64,
  surface: Surface,
}

impl Item for Plane {
  fn normal(&self, _pos: Vector) -> Vector {
    self.norm
  }
  fn surface(&self) -> Surface {
    self.surface
  }
  fn intersect(&self, ray: Ray) -> Option<Intersection> {
    let denom = self.norm * ray.dir;
    if denom > 0. {
      None
    } else {
      let dist = ((self.norm * ray.start) + self.offset) / (-1. * denom);
      Some(Intersection {
        ray,
        dist,
        item: Box::new(*self),
      })
    }
  }
}
