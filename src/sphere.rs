use crate::scene::Intersection;
use crate::scene::Item;
use crate::scene::Ray;
use crate::surfaces::Surface;
use crate::vector::Vector;
use std::rc::Rc;

#[derive(Clone)]
pub struct Sphere {
  pub radius: f64,
  pub center: Vector,
  pub surface: Rc<dyn Surface>,
}

impl Item for Sphere {
  fn normal(&self, pos: Vector) -> Vector {
    (pos - self.center).normalize()
  }
  fn surface(&self) -> Rc<dyn Surface> {
    self.surface.clone()
  }
  fn intersect(&self, ray: Ray) -> Option<Intersection> {
    let eo = self.center - ray.start;
    let v = eo * ray.dir;
    let mut dist = 0.;
    if v >= 0. {
      let disc = self.radius.powi(2) - (eo * eo - v.powi(2));
      if disc >= 0. {
        dist = v - disc.sqrt();
      }
    }
    if dist == 0. {
      None
    } else {
      Some(Intersection {
        ray,
        dist,
        item: Rc::new(self.clone()),
      })
    }
  }
}
