#[macro_use]
pub mod parser;
mod structs;
#[cfg(test)]
mod tests;
#[macro_use]
extern crate nom;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;
#[cfg(test)]
extern crate quickcheck;

mod vec3f;
use vec3f::Vec3f;

use structs::Obj;
use structs::PPMImage;
fn main() {
    const WIDTH: u32 = 1920;
    const HEIGHT: u32 = 1080;
    let obj = Obj::create("suzan.obj");
    // println!("{:#?}", obj);
    let img = PPMImage::new(WIDTH, HEIGHT);
    // let suzan = parser::obj::parse_obj_file("suzan.obj");
    // println!("{:#?}", suzan);
    // let model = parse_model("syzan.obj"); // easiest part of the project

    // for face in obj.faces.iter() {
    //     for line in face.lines {
    //         let (start, end) = line;
    //         img.draw_line(start, end, (255, 255, 255));
    //     }
    // }

    // img.write_to_file("output.ppm");
}
