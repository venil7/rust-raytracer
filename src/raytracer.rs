use crate::scene::Intersection;
use crate::scene::Ray;
use crate::scene::Scene;

pub struct Raytracer;
impl Raytracer {
  pub fn intersections(&self, ray: &Ray, scene: &Scene) -> Option<Intersection> {
    let init = (None, std::f64::MAX);
    let (inter, _) = scene
      .items
      .iter()
      .fold(init, |acc, item| match item.intersect(ray) {
        Some(ref intersect) if intersect.dist < acc.1 => (Some(intersect.clone()), intersect.dist),
        _ => acc,
      });

    inter
  }

  // fn test_ray(&self, ray: &Ray, scene: &Scene) -> {

  // }
}
