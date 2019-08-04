use crate::color::Color;
use crate::raytracer::Raytracer;
use crate::scene::Scene;
use crate::vector::Vector;
use image::png::PNGEncoder;
use image::ColorType;
use std::fs::File;

pub mod camera;
pub mod color;
pub mod plane;
pub mod raytracer;
pub mod scene;
pub mod sphere;
pub mod surfaces;
pub mod vector;

pub fn coords_for_angle(radius: f64, angle: f64, z: f64) -> Vector {
    let x = radius * angle.cos() + 0.;
    let y = radius * angle.sin() + 0.;
    Vector(x, y, z)
}

pub fn render_scene(scene: &Scene, fname: &str) -> std::io::Result<()> {
    let raytracer = Raytracer;
    let width = 320;
    let height = 240;
    let colors = raytracer.render(scene, width, height);
    let bytes = colors
        .iter()
        .rev()
        .fold(vec![], |mut acc: Vec<u8>, color: &Color| {
            let rgb: [u8; 3] = (*color).into();
            acc.extend(&rgb);
            acc
        });
    let output = File::create(fname)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(&bytes, width as u32, height as u32, ColorType::RGB(8))
}

fn main() -> Result<(), std::io::Error> {
    for i in 0..359 {
        let camera_vector = coords_for_angle(15., i as f64, 15.);
        let scene = Scene::default(&camera_vector);
        render_scene(&scene, &format!("render-{:03}.png", i))?
    }
    Ok(())
}
