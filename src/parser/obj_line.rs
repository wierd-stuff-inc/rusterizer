use crate::structs::types::Point;
use crate::vec3f::Vec3f;
use nom::eol;
use nom::float;
use nom::float_s;
use nom::line_ending;
use nom::space;
use std::str::FromStr;
use std::string::String;

// #[allow(dead_code)]
// #[derive(Debug)]
// pub struct Obj {
//     pub name: String,
//     pub vertexes: Vec<Point>,
//     pub normals: Vec<Point>,
// }

#[derive(PartialEq, Debug)]
pub struct FaceIndex(pub u32, pub Option<u32>, pub Option<u32>);

#[derive(PartialEq, Debug)]
pub enum ObjLine {
    Comment(String),
    ObjectName(String),
    GroupName(String),
    MtlLib(String),
    UseMtl(String),
    SmoothShading(String),
    Vertex(Vec3f),
    VertexParam(Vec3f),
    Normal(Vec3f),
    Face(Vec<FaceIndex>),
    TextureUVW(f32, f32, Option<f32>),
}

// impl Obj {
//     fn new(name: String) -> Obj {
//         Obj {
//             name: name,
//             vertexes: Vec::new(),
//             normals: Vec::new(),
//         }
//     }
// }

macro_rules! sp (
   ($i:expr, $($args:tt)*) => (
     {
       sep!($i, space, $($args)*)
     }
   )
);

#[macro_export]
macro_rules! def_string_line (
   ($id:ident, $i:expr, $tt:tt, $ty:ident) => (
       named!( $id< &str, $tt >, map!(
           delimited!(tag!($i), take_until!("\n"), end_of_line),
           |s| $tt :: $ty(s.to_string())
       ));
   )
);

named!(
    comment<&str, &str>,
    delimited!(tag!("#"), take_until!("\n"), alt!(eof!() | eol))
);

named!(comment_line<&str, ObjLine>,
    do_parse!(
        comm: comment >>
        (ObjLine::Comment(comm.to_string()))
    )
);

named!(pub parse_vertex<&str, ObjLine>,
    do_parse!(
        tag!("v") >>
        space >>
        x: float_s >>
        space >>
        y: float_s >>
        space >>
        z: float_s >>
        line_ending >>
        (ObjLine::Vertex(Vec3f::new(x, y, z)))
    )
);

named!(pub end_of_line<&str, &str>, alt!(
    eof!()|eol|comment
));

named!(pub parse_vertex_normal<&str, ObjLine>,
    do_parse!(
        tag!("vn") >>
        space >>
        x: float >>
        space >>
        y: float >>
        space >>
        z: float >>
        line_ending >>
        (ObjLine::Normal(Vec3f::new(x, y, z)))
    )
);

def_string_line!(s_line, "s", ObjLine, SmoothShading);
def_string_line!(usemtl_line, "usemtl", ObjLine, UseMtl);
def_string_line!(parse_obj_name, "o", ObjLine, ObjectName);

// named!(pub usemtl_line<&str, ObjLine>,
//     do_parse!(
//         tag!("usemtl") >>
//         space >>
//         name: complete!(nom::alpha)>>
//         (ObjLine::UseMtl(name.to_string()))
//     )
// );

named!(my_u32(&str) -> u32,
    map_res!(recognize!(nom::digit), u32::from_str)
);

named!(pub face_pair< &str, FaceIndex >, map!(
    separated_pair!(
        my_u32,
        tag!("/"),
        my_u32
    ),
    |(v,vt)| FaceIndex(v, Some(vt), None)
));

named!( face_triple< &str, FaceIndex >, map!(
    tuple!(
        my_u32,
        delimited!(tag!("/"), opt!(my_u32), tag!("/")),
        my_u32
    ),
    |(v, vt, vn)| FaceIndex(v, vt, Some(vn))
));

named!(face_line<&str, ObjLine>, dbg_dmp!(delimited!(
        ws!(tag!("f")),
        alt!(
            separated_nonempty_list_complete!(space, face_pair) => {|vec| ObjLine::Face(vec)}
            |
            separated_nonempty_list_complete!(space, face_triple) =>  {|vec| ObjLine::Face(vec)}
            |
            separated_nonempty_list!(space, my_u32) => {|vec: Vec<u32>| {
                 ObjLine::Face(vec.iter().map(|&v| FaceIndex(v, None, None)).collect::<Vec<_>>())
             }
         }
        ),
        end_of_line
    ))
);

// named!(face_line<&str, ObjLine>, dbg!(do_parse!(
//     tag!("f")>>
//     list : alt!(
//         separated_list!(space, my_u32) => {|vec: Vec<u32>| vec.iter().map(|&v| FaceIndex(v, None, None)).collect::<Vec<_>>()}
//         |
//         separated_list!(space, face_pair) => {|vec| vec}
//         |
//         separated_list!(space, face_triple) =>  {|vec| vec}
//     )>>
//     end_of_line >>
//     (ObjLine::Face(list))
// )));

named!(pub mtl_lib<&str, ObjLine>,
    do_parse!(
        tag!("mtllib") >>
        space >>
        lib: complete!(nom::alpha)>>
        (ObjLine::MtlLib(lib.to_string()))
    )
);

named!(pub parse_obj_line<&str, ObjLine>, alt!(
    parse_obj_name
    | parse_vertex
    | parse_vertex_normal
    | comment_line
    | usemtl_line
    | s_line
    | mtl_lib
    | face_line
));
