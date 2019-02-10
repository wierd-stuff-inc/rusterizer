use crate::structs::types::Point;
use crate::vec3f::Vec3f;
use nom::eol;
use nom::float;
use nom::float_s;
use nom::line_ending;
use nom::space;
use std::str::FromStr;
use std::string::String;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Obj {
    pub name: String,
    pub vertexes: Vec<Point>,
    pub normals: Vec<Point>,
}

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
    Face(FaceIndex, FaceIndex, FaceIndex),
    TextureUVW(f32, f32, Option<f32>),
}

impl Obj {
    fn new(name: String) -> Obj {
        Obj {
            name: name,
            vertexes: Vec::new(),
            normals: Vec::new(),
        }
    }
}

named!(
    comment,
    delimited!(tag!("#"), take_until!("\n"), alt!(eof!() | eol))
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

named!(pub end_of_line, alt!(
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

named!(pub parse_obj_name<&str, ObjLine>,
    do_parse!(
        tag!("o") >>
        space >>
        name: complete!(nom::alpha)>>
        (ObjLine::ObjectName(name.to_string()))
    )
);

named!(my_u32(&str) -> u32,
    map_res!(recognize!(nom::digit), u32::from_str)
);

named!(pub face_pair< &str, FaceIndex >, map!(
    separated_pair!(
        my_u32,
        tag!("/"),
        opt!(my_u32)
    ),
    |(v,vt)| FaceIndex(v, vt, None)
));

named!(pub parse_obj_line<&str, ObjLine>, alt!(
    parse_obj_name      |
    parse_vertex_normal |
    parse_vertex
));
