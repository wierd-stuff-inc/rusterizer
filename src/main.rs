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
use crate::parser::obj_line::ObjLine::TextureUVW;
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
        let texture_coords: Vec<_> = object
            .texture_coords()
            .iter()
            .map(|texture_coord| match texture_coord {
                TextureUVW(u, v, _) => (u, v),
                _ => unreachable!(),
            })
            .collect();
        for face_line in object.faces() {
            if let Face(face_shit) = face_line {
                // let size = face_shit.len();
                // for v_n in 0..size {
                let FaceIndex(fa1, diff1_id, _) = &face_shit[0];
                let FaceIndex(fb1, diff2_id, _) = &face_shit[1];
                let FaceIndex(fc1, diff3_id, _) = &face_shit[2];
                let diff1_id = diff1_id.expect("Can't get texture index.") - 1;
                let diff2_id = diff2_id.expect("Can't get texture index.") - 1;
                let diff3_id = diff3_id.expect("Can't get texture index.") - 1;
                // Мои предки улыбаются, глядя на меня, имперцы.
                // А ваши улыбаются вам?
                // (с) Грязевой краб.

                let texture_mapping = |barry_x: f64, barry_y: f64, barry_z: f64| {
                    let texture_coord_a = texture_coords[diff1_id as usize];
                    let texture_coord_b = texture_coords[diff2_id as usize];
                    let texture_coord_c = texture_coords[diff3_id as usize];

                    let u = texture_coord_a.0 * barry_x
                        + texture_coord_b.0 * barry_y
                        + texture_coord_c.0 * barry_z;
                    let v = texture_coord_a.1 * barry_x
                        + texture_coord_b.1 * barry_y
                        + texture_coord_c.1 * barry_z;

                    (u, (1.0 - v))
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
