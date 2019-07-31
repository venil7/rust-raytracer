use std::ops::{Add, Mul, Sub};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vector(pub f64, pub f64, pub f64);

impl Add for Vector {
  type Output = Self;
  fn add(self, rhs: Self) -> Self {
    Vector(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
  }
}
impl Sub for Vector {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self {
    Vector(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
  }
}
impl Mul<f64> for Vector {
  type Output = Self;
  fn mul(self, rhs: f64) -> Self {
    Vector(self.0 * rhs, self.1 * rhs, self.2 * rhs)
  }
}
impl Mul<Vector> for Vector {
  type Output = f64;
  fn mul(self, rhs: Self) -> f64 {
    (self.0 * rhs.0) + (self.1 * rhs.1) + (self.2 * rhs.2)
  }
}

impl Vector {
  pub fn mag(&self) -> f64 {
    ((*self * *self) as f64).sqrt()
  }
  pub fn normalize(&self) -> Vector {
    let mag = self.mag();
    let div = if mag == 0.0 { std::f64::MAX } else { 1.0 / mag };
    *self * div
  }
  pub fn cross(&self, vector: &Vector) -> Vector {
    Vector(
      self.1 * vector.2 - self.2 * vector.1,
      self.2 * vector.0 - self.0 * vector.2,
      self.0 * vector.1 - self.1 * vector.0,
    )
  }
}

#[test]
fn vector_addition() {
  let vec1 = Vector(1.0, 2.0, 3.0);
  let vec2 = Vector(3.0, 2.0, 1.0);
  assert_eq!(vec1 + vec2, Vector(4.0, 4.0, 4.0));
}
#[test]
fn vector_subtraction() {
  let vec1 = Vector(1.0, 2.0, 3.0);
  let vec2 = Vector(3.0, 2.0, 1.0);
  assert_eq!(vec1 - vec2, Vector(-2.0, 0.0, 2.0));
}
#[test]
fn vector_multiply_scalar() {
  let vec1 = Vector(1.0, 2.0, 3.0);
  assert_eq!(vec1 * 5.0, Vector(5.0, 10.0, 15.0));
}
#[test]
fn vector_multiply_vector() {
  let vec1 = Vector(1.0, 2.0, 3.0);
  let vec2 = Vector(3.0, 2.0, 1.0);
  assert_eq!(vec1 * vec2, 10.0);
}
#[test]
fn vector_mag() {
  let vec1 = Vector(1.0, 2.0, 3.0);
  assert_eq!(vec1.mag(), 3.7416573867739413);
}
#[test]
fn vector_normalize() {
  let vec1 = Vector(1.0, 2.0, 3.0);
  assert_eq!(
    vec1.normalize(),
    Vector(0.2672612419124244, 0.5345224838248488, 0.8017837257372732)
  );
}
#[test]
fn vector_cross() {
  let vec1 = Vector(1.0, 2.0, 3.0);
  let vec2 = Vector(3.0, 2.0, 1.0);
  assert_eq!(vec1.cross(&vec2), Vector(-4.0, 8.0, -4.0));
}
