use crate::vector::Vector;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Camera {
  pub position: Vector,
  pub forward: Vector,
  pub right: Vector,
  pub up: Vector,
}

impl Camera {
  pub fn new(position: Vector, look_at: Vector) -> Camera {
    let down = Vector(0.0, -1.0, 0.0);
    let forward = (look_at - position).normalize();
    let right = forward.cross(&down).normalize() * 1.5;
    let up = forward.cross(&right).normalize() * 1.5;
    Camera {
      position,
      forward,
      right,
      up,
    }
  }
}

#[test]
fn new_camera() {
  let position = Vector(1., 2., 3.);
  let look_at = Vector(3., 2., 1.);
  let camera = Camera::new(position, look_at);
  assert_eq!(
    camera,
    Camera {
      position: Vector(1., 2., 3.),
      forward: Vector(0.7071067811865475, 0., -0.7071067811865475),
      right: Vector(-1.0606601717798214, 0., -1.0606601717798214),
      up: Vector(0., 1.5, 0.)
    }
  )
}
