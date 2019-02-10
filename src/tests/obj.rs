use crate::parser::obj_line::parse_vertex;
use crate::parser::obj_line::parse_vertex_normal;
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
