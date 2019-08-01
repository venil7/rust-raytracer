use crate::camera::Camera;
use crate::color::Color;
use crate::plane::Plane;
use crate::sphere::Sphere;
use crate::surfaces::Checker;
use crate::surfaces::Shiny;
use crate::surfaces::Surface;
use crate::vector::Vector;
use std::rc::Rc;

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
  pub items: Vec<Box<dyn Item>>,
  pub lights: Vec<Light>,
  pub camera: Camera,
}

impl Default for Scene {
  fn default() -> Scene {
    Scene {
      camera: Camera::new(Vector(3., 2., 4.), Vector(-1., 0.5, 0.)),
      lights: vec![
        Light {
          pos: Vector(-2.0, 2.5, 0.),
          color: Color(0.49, 0.07, 0.07),
        },
        Light {
          pos: Vector(1.5, 2.5, 1.5),
          color: Color(0.07, 0.07, 0.49),
        },
        Light {
          pos: Vector(1.5, 2.5, -1.5),
          color: Color(0.07, 0.49, 0.071),
        },
        Light {
          pos: Vector(0.0, 3.5, 0.0),
          color: Color(0.21, 0.21, 0.35),
        },
      ],
      items: vec![
        Box::new(Plane {
          norm: Vector(0.0, 1., 0.),
          offset: 0.,
          surface: Rc::new(Checker),
        }),
        Box::new(Sphere {
          center: Vector(0.0, 1.0, -0.25),
          radius: 1.,
          surface: Rc::new(Shiny),
        }),
        Box::new(Sphere {
          center: Vector(-1.0, 0.5, 1.5),
          radius: 0.5,
          surface: Rc::new(Shiny),
        }),
      ],
    }
  }
}
