use std::ops::{Add, Mul};

#[derive(PartialEq, Debug, Clone,Copy, Default)]
pub struct Color(pub f64, pub f64, pub f64);

impl Add for Color {
  type Output = Self;
  fn add(self, rhs: Self) -> Self {
    Color(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
  }
}
impl Mul<f64> for Color {
  type Output = Self;
  fn mul(self, scale: f64) -> Self {
    Color(self.0 * scale, self.1 * scale, self.2 * scale)
  }
}
impl Mul<Color> for Color {
  type Output = Color;
  fn mul(self, rhs: Self) -> Self {
    Color(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
  }
}
impl Color {
  pub fn white() -> Color {
    Color(1., 1., 1.)
  }
  pub fn grey() -> Color {
    Color(0.5, 0.5, 0.5)
  }
  pub fn black() -> Color {
    Color::default()
  }
}

#[test]
fn color_addition() {
  let c1 = Color(0.5, 0.5, 0.5);
  let c2 = Color(0.5, 0.5, 0.5);
  assert_eq!(c1 + c2, Color(1.0, 1.0, 1.0));
}
#[test]
fn color_multiply_color() {
  let c1 = Color(0.5, 0.5, 0.5);
  let c2 = Color(0.5, 0.5, 0.5);
  assert_eq!(c1 * c2, Color(0.25, 0.25, 0.25));
}
#[test]
fn color_multiply_scalar() {
  let c1 = Color(0.5, 0.5, 0.5);
  assert_eq!(c1 * 0.5, Color(0.25, 0.25, 0.25));
}
