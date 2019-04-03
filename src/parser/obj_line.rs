use crate::parser::obj_line::ObjLine::TextureUVW;
use crate::structs::types::Vec3f;
use nom::double;
use nom::eol;
use nom::line_ending;
use nom::space;
use std::str::FromStr;
use std::string::String;

#[derive(PartialEq, Debug, Clone)]
pub struct FaceIndex(pub u32, pub Option<u32>, pub Option<u32>);

#[derive(PartialEq, Debug, Clone)]
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
    TextureUVW(f64, f64, Option<f64>),
    End(String),
}

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
        x: double >>
        space >>
        y: double >>
        space >>
        z: double >>
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
        x: double >>
        space >>
        y: double >>
        space >>
        z: double >>
        line_ending >>
        (ObjLine::Normal(Vec3f::new(x, y, z)))
    )
);

named!(pub parse_vertex_texture<&str, ObjLine>,
    dbg_dmp!(do_parse!(
    tag!("vt") >>
    space >>
    uvw: tuple!(
        double,
        space,
        double,
        opt!(preceded!(space, double))
    ) >>
    end_of_line >>
    (TextureUVW(uvw.0 ,uvw.2, uvw.3))
)));

named!(vertex_texture< &str, (f64, f64, Option<f64>) >, map!(
    tuple!(
        double,
        double,
        opt!(double)
    ),
    |(v, vt, vn)| (v, vt, vn)
));

def_string_line!(s_line, "s", ObjLine, SmoothShading);
def_string_line!(usemtl_line, "usemtl", ObjLine, UseMtl);
def_string_line!(mtl_lib, "mtllib", ObjLine, MtlLib);
def_string_line!(parse_obj_name, "o", ObjLine, ObjectName);
def_string_line!(parse_group_name, "g", ObjLine, GroupName);

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

named!( face_triple< &str, FaceIndex >, map!(
    tuple!(
        my_u32,
        delimited!(tag!("/"), opt!(my_u32), tag!("/")),
        my_u32
    ),
    |(v, vt, vn)| FaceIndex(v, vt, Some(vn))
));

named!(faces_pair_list<&str, ObjLine>,
    dbg!(map!(separated_nonempty_list!(space, face_pair), ObjLine::Face))
);

named!(faces_triple_list<&str, ObjLine>,
    dbg!(map!(separated_nonempty_list!(space, face_triple), ObjLine::Face))
);

named!(faces_uint_list<&str, ObjLine>,
    map!(separated_nonempty_list!(space, my_u32), |vec: Vec<u32>| {
         ObjLine::Face(vec.iter().map(|&v| FaceIndex(v, None, None)).collect::<Vec<_>>())
     })
);

named!(pub face_line<&str, ObjLine>,
        do_parse!(
            ws!(tag!("f")) >>
            list: alt!(
                faces_triple_list
                | faces_pair_list
                | faces_uint_list
            ) >>
            end_of_line >>
            (list)
        )
);

named!(pub obj_end<&str, ObjLine>,
    do_parse!(
        end: end_of_line>>
        (ObjLine::End(end.to_string()))
    )
);

/// Распарсить строку и привести в один из перечисленных ниже типов.
named!(pub parse_obj_line<&str, ObjLine>, alt!(
    parse_obj_name
    | parse_vertex
    | parse_vertex_normal
    | comment_line
    | usemtl_line
    | s_line
    | mtl_lib
    | parse_group_name
    | obj_end
    | face_line
    | parse_vertex_texture
));
