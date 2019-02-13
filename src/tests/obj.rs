use crate::parser::obj_line::face_line;
use crate::parser::obj_line::parse_vertex;
use crate::parser::obj_line::parse_vertex_normal;
use crate::parser::obj_line::parse_vertex_texture;
use crate::parser::obj_line::FaceIndex;
use crate::parser::obj_line::ObjLine;
use crate::vec3f::Vec3f;

#[quickcheck]
fn parse_vertex_test(vec: Vec3f) -> bool {
    let st = format!("v {x} {y} {z}\n", x = vec.x, y = vec.y, z = vec.z);
    parse_vertex(&st) == Ok(("", ObjLine::Vertex(vec)))
}

#[quickcheck]
fn parse_vertex_normals_test(vec: Vec3f) -> bool {
    let st = format!("vn {x} {y} {z}\n", x = vec.x, y = vec.y, z = vec.z);
    parse_vertex_normal(&st) == Ok(("", ObjLine::Normal(vec)))
}

#[quickcheck]
fn parse_triple_faces_line(vec: Vec3f) -> bool {
    let (a, b, c) = (vec.x.abs() as u32, vec.y.abs() as u32, vec.z.abs() as u32);
    let st = format!("f {x}/{y}/{z}\n", x = a, y = b, z = c);
    let ft = face_line(&st);
    println!("{:?}", st);
    println!("{:#?}", ft);
    ft == Ok(("", ObjLine::Face(vec![FaceIndex(a, Some(b), Some(c))])))
}

#[quickcheck]
fn parse_vertex_normals(vec: Vec3f) -> bool {
    let st = format!("vn {x} {y} {z}\n", x = vec.x, y = vec.y, z = vec.z);
    parse_vertex_normal(&st) == Ok(("", ObjLine::Normal(vec)))
}

#[quickcheck]
fn parse_vertex_texture_test(vec: Vec3f) -> bool {
    let (a, b, c) = (vec.x, vec.y, vec.z);
    let st = format!("vt {x} {y} {z}\n", x = a, y = b, z = c);
    println!("{:#?}", parse_vertex_texture(&st));
    parse_vertex_texture(&st) == Ok(("", ObjLine::TextureUVW(a, b, Some(c))))
}
