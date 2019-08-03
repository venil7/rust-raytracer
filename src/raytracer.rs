use crate::color::Color;
use crate::scene::Intersection;
use crate::scene::Item;
use crate::scene::Ray;
use crate::scene::Scene;
use crate::vector::Vector;

const MAX_DEPTH: u8 = 5;

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

  pub fn test_ray(&self, ray: &Ray, scene: &Scene) -> Option<f64> {
    let isect = self.intersections(ray, scene);
    Some(isect?.dist)
  }

  pub fn trace_ray(&self, ray: &Ray, scene: &Scene, depth: u8) -> Color {
    match self.intersections(ray, scene) {
      Some(ref isect) => self.shade(isect, scene, depth),
      _ => Color::black(),
    }
  }

  fn shade(&self, isect: &Intersection, scene: &Scene, depth: u8) -> Color {
    let pos = (isect.ray.dir * isect.dist) + isect.ray.start;
    let normal = isect.item.normal(&pos);
    let reflect_dir = isect.ray.dir - (normal * (normal * isect.ray.dir) * 2.);
    let natural_color =
      Color::background() + self.natural_color(&*isect.item, &pos, &normal, &reflect_dir, scene);
    let reflected_color = if depth >= MAX_DEPTH {
      Color::grey()
    } else {
      self.reflection_color(
        &*isect.item,
        &pos,
        /*&normal,*/ &reflect_dir,
        scene,
        depth,
      )
    };

    natural_color + reflected_color
  }

  fn reflection_color(
    &self,
    item: &dyn Item,
    pos: &Vector,
    // norm: &Vector,
    reflect_dir: &Vector,
    scene: &Scene,
    depth: u8,
  ) -> Color {
    let surface = item.surface();
    let factor = surface.reflect(pos);
    let color = self.trace_ray(
      &Ray {
        start: *pos,
        dir: *reflect_dir,
      },
      scene,
      depth + 1,
    );
    color * factor
  }

  fn natural_color(
    &self,
    item: &dyn Item,
    pos: &Vector,
    norm: &Vector,
    reflect_dir: &Vector,
    scene: &Scene,
  ) -> Color {
    scene.lights.iter().fold(Color::default(), |color, light| {
      let ldis = light.pos - *pos;
      let livec = ldis.normalize();
      let neat_isect = self.test_ray(
        &Ray {
          start: *pos,
          dir: livec,
        },
        scene,
      );
      match neat_isect {
        Some(v) if v <= ldis.mag() => {
          let illum = livec * *norm;
          let lcolor = if illum > 0. {
            light.color * illum
          } else {
            Color::default()
          };
          let specular = livec * reflect_dir.normalize();
          let scolor = if specular > 0. {
            light.color * specular.powf(item.surface().roughness())
          } else {
            Color::default()
          };
          (lcolor * item.surface().diffuse(pos)) + (scolor * item.surface().specular(pos)) + color
        }
        _ => color,
      }
    })
  }

  pub fn render(&self, scene: &Scene, width: usize, height: usize) -> Vec<Color> {
    (0..width * height)
      .map(|idx| {
        let camera = scene.camera;
        let x = idx % width;
        let y = idx / width;
        let recenter_x = (x as f64 - (width as f64 / 2.)) / 2. / width as f64;
        let recenter_y = (y as f64 - (height as f64 / 2.)) / 2. / height as f64;
        let dir =
          (camera.right * recenter_x + (camera.up * recenter_y) + camera.forward).normalize();
        let ray = Ray {
          start: camera.position,
          dir: dir,
        };
        self.trace_ray(&ray, scene, 0)
      })
      .collect::<Vec<Color>>()
  }
}
