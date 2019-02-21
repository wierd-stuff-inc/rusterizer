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
    let f_width = WIDTH as f32;
    let f_height = HEIGHT as f32;
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
                            let x0 = (((vert1.x + 1.0) * f_width / 2.0) % f_width) as i32;
                            let y0 = (((vert1.y + 1.0) * f_height / 2.0) % f_height) as i32;
                            let y1 = (((vert2.y + 1.0) * f_height / 2.0) % f_width) as i32;
                            let x1 = (((vert2.x + 1.0) * f_width / 2.0) % f_width) as i32;
                            let y2 = (((vert3.y + 1.0) * f_height / 2.0) % f_width) as i32;
                            let x2 = (((vert3.x + 1.0) * f_width / 2.0) % f_width) as i32;
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
            // }
        }
    }
    for object in suzan.objects {
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
                            let x0 = (((vert1.x + 1.0) * f_width / 2.0) % f_width) as i32;
                            let y0 = (((vert1.y + 1.0) * f_height / 2.0) % f_height) as i32;
                            let y1 = (((vert2.y + 1.0) * f_height / 2.0) % f_width) as i32;
                            let x1 = (((vert2.x + 1.0) * f_width / 2.0) % f_width) as i32;
                            let y2 = (((vert3.y + 1.0) * f_height / 2.0) % f_width) as i32;
                            let x2 = (((vert3.x + 1.0) * f_width / 2.0) % f_width) as i32;
                            img.set_pixel(x0 as u32, y0 as u32, (0, 255, 0));
                            img.set_pixel(x1 as u32, y1 as u32, (0, 255, 0));
                            img.set_pixel(x2 as u32, y2 as u32, (0, 255, 0));
                        }
                    }
                }
            }
            // }
        }
    }
    img.write_to_file("output.ppm");
}
