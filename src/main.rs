use crate::color::Color;
use crate::raytracer::Raytracer;
use crate::scene::Scene;
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

fn main() -> Result<(), std::io::Error> {
    let raytracer = Raytracer;
    let width = 800;
    let height = 600;
    let colors = raytracer.render(&Scene::default(), width, height);
    let bytes = colors
        .iter()
        .rev()
        .fold(vec![], |mut acc: Vec<u8>, color: &Color| {
            let rgb: [u8; 3] = (*color).into();
            acc.extend(&rgb);
            acc
        });
    let output = File::create("render.png")?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(&bytes, width as u32, height as u32, ColorType::RGB(8))
}
