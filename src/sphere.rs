use crate::scene::Intersection;
use crate::scene::Item;
use crate::scene::Ray;
use crate::scene::Surface;
use crate::vector::Vector;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Sphere {
  radius: f64,
  center: Vector,
  surface: Surface,
}

impl Item for Sphere {
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
        item: Box::new(*self),
      })
    }
  }
  fn normal(&self, pos: Vector) -> Vector {
    (pos - self.center).normalize()
  }
  fn surface(&self) -> Surface {
    self.surface
  }
}
