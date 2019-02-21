#[macro_use]
pub mod parser;
mod structs;
#[cfg(test)]
mod tests;
#[macro_use]
extern crate nom;

extern crate rand;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;
#[cfg(test)]
extern crate quickcheck;
use crate::parser::obj_line::FaceIndex;
use crate::parser::obj_line::ObjLine::Face;
use crate::parser::obj_line::ObjLine::Vertex;
use crate::structs::types::Point;
use crate::structs::types::Vec3f;
use rand::random;

use structs::Obj;
use structs::PPMImage;
fn main() {
    const WIDTH: u32 = 512;
    const HEIGHT: u32 = 512;
    let suzan = Obj::create("african_head.obj");
    // println!("{:#?}", suzan);
    let mut img = PPMImage::new(WIDTH, HEIGHT);
    let f_width = f64::from(WIDTH);
    let f_height = f64::from(HEIGHT);
    for object in suzan.objects.clone() {
        let vertices = object
            .vertices()
            .iter()
            .map(|vert| match vert {
                Vertex(vec) => Some(vec),
                _ => None,
            })
            .collect::<Vec<Option<&Vec3f>>>();
        for face_line in object.faces() {
            if let Face(face_shit) = face_line {
                // let size = face_shit.len();
                // for v_n in 0..size {
                let FaceIndex(fa1, _, _) = &face_shit[0];
                let FaceIndex(fb1, _, _) = &face_shit[1];
                let FaceIndex(fc1, _, _) = &face_shit[2];
                // println!("{:?}", *fb1 as usize);
                if let Some(vert1) = vertices[(*fa1 - 1) as usize] {
                    if let Some(vert2) = vertices[(*fb1 - 1) as usize] {
                        if let Some(vert3) = vertices[(*fc1 - 1) as usize] {
                            let x0 = ((vert1.x + 1.0) * f_width / 2.0) % f_width;
                            let y0 = ((vert1.y + 1.0) * f_height / 2.0) % f_height;
                            let y1 = ((vert2.y + 1.0) * f_height / 2.0) % f_width;
                            let x1 = ((vert2.x + 1.0) * f_width / 2.0) % f_width;
                            let y2 = ((vert3.y + 1.0) * f_height / 2.0) % f_width;
                            let x2 = ((vert3.x + 1.0) * f_width / 2.0) % f_width;
                            img.draw_triangle(
                                Point::new(x0, y0),
                                Point::new(x1, y1),
                                Point::new(x2, y2),
                                (random(), random(), random()),
                            )
                        }
                    }
                }
            }
        }
    }
    img.write_to_file("output.ppm");
}
