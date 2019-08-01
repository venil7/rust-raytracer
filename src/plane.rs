use crate::scene::Intersection;
use crate::scene::Item;
use crate::scene::Ray;
use crate::surfaces::Surface;
use crate::vector::Vector;
use std::rc::Rc;

#[derive(Clone)]
pub struct Plane {
  pub norm: Vector,
  pub offset: f64,
  pub surface: Rc<dyn Surface>,
}

impl Item for Plane {
  fn normal(&self, _pos: Vector) -> Vector {
    self.norm
  }
  fn surface(&self) -> Rc<dyn Surface> {
    self.surface.clone()
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
        item: Rc::new(self.clone()),
      })
    }
  }
}
