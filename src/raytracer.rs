use crate::scene::Scene;
use crate::scene::Ray;
use crate::scene::Intersection;

pub struct Raytracer;
impl Raystracer {
  fn intersections(&self, ray: &Ray, scene: &Scene) -> Option<Intersection> {
    let mut closest :f64 = std::f64::MAX;
    let mut closest_inter = None;
    for item in scene.items {
      let iter = item.intersect(ray)
      

    }

    closest_inter
  }

  fn test_ray(&self, ray: &Ray, scene: &Scene) -> {

  }
}