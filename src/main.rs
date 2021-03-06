#[macro_use]
pub mod parser;
mod structs;
#[cfg(test)]
mod tests;
#[macro_use]
extern crate nom;

// use structs::image::GlobImage;

#[cfg(test)]
// #[macro_use(quickcheck)]
// extern crate quickcheck_macros;
#[cfg(test)]
extern crate quickcheck;
extern crate rand;
use crate::parser::obj_line::FaceIndex;
use crate::parser::obj_line::ObjLine::Face;
use crate::parser::obj_line::ObjLine::Vertex;
use crate::structs::image::GlobImage;
// use crate::structs::types::Vec3f;

use structs::image::PPMImage;
use structs::Obj;
use structs::Renderer;

fn main() {
    const WIDTH: u32 = 512;
    const HEIGHT: u32 = 512;
    let suzan = Obj::create("african_head.obj");
    let mut img = PPMImage::new(WIDTH, HEIGHT);
    let diffuse = PPMImage::load("african_head_diffuse.ppm");
    let mut renderer = Renderer::new(&mut img, &diffuse);
    for object in suzan.objects {
        let vertices: Vec<_> = object
            .vertices()
            .iter()
            .map(|vert| match vert {
                Vertex(vec) => Some(vec),
                _ => None,
            })
            .collect();
        for face_line in object.faces() {
            if let Face(face_shit) = face_line {
                // let size = face_shit.len();
                // for v_n in 0..size {
                let FaceIndex(fa1, diff1_id, _) = &face_shit[0];
                let FaceIndex(fb1, diff2_id, _) = &face_shit[1];
                let FaceIndex(fc1, diff3_id, _) = &face_shit[2];
                let diff1_id = diff1_id.expect("Can't get texture index.");
                let diff2_id = diff2_id.expect("Can't get texture index.");
                let diff3_id = diff3_id.expect("Can't get texture index.");

                let texture_mapping = |barry_x: f64, barry_y: f64, barry_z: f64| {
                    (0.0, 0.0)
                };

                // println!("{:?}", *fb1 as usize);
                if let Some(vert0) = vertices[(*fa1 - 1) as usize] {
                    if let Some(vert1) = vertices[(*fb1 - 1) as usize] {
                        if let Some(vert2) = vertices[(*fc1 - 1) as usize] {
                            renderer.draw_poly(vert0, vert1, vert2, texture_mapping);
                        }
                    }
                }
            }
        }
    }
    renderer.write_to_file("output.ppm");
}
