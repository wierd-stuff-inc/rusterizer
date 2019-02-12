use crate::parser::obj_line::*;
use crate::parser::ObjParser;
#[derive(Debug)]
pub struct Obj {
    pub filename: String,
    pub comments: Vec<ObjLine>,
    pub objects: Vec<ObjObject>,
}

#[allow(dead_code)]
impl Obj {
    pub fn create(filename: &str) -> Self {
        let mut obj = Obj {
            filename: filename.to_string(),
            comments: Vec::new(),
            objects: Vec::new(),
        };
        let parser = ObjParser::create(filename);

        let mut object = ObjObject::new();
        for line in parser {
            match line {
                ObjLine::ObjectName(name) => {
                    // new object encountered, when multiple objects exist
                    if object.name.is_some() {
                        obj.objects.push(object);
                        object = ObjObject::new();
                    }
                    object.name = Some(name);
                }
                ObjLine::MtlLib(name) => object.material = Some(name),
                ObjLine::Vertex(..) => object.vertices.push(line),
                ObjLine::VertexParam(..) => object.vertex_params.push(line),
                ObjLine::Face(..) => object.faces.push(line),
                ObjLine::Normal(..) => object.normals.push(line),
                ObjLine::TextureUVW(..) => object.texture_coords.push(line),
                _ => {}
            }
        }
        obj.objects.push(object);
        obj
    }
}

#[derive(Debug)]
pub struct ObjObject {
    pub name: Option<String>,
    pub material: Option<String>,
    vertices: Vec<ObjLine>,
    normals: Vec<ObjLine>,
    texture_coords: Vec<ObjLine>,
    vertex_params: Vec<ObjLine>,
    faces: Vec<ObjLine>,
}
#[allow(dead_code)]
impl ObjObject {
    pub fn new() -> Self {
        ObjObject {
            name: None,
            material: None,
            vertices: Vec::new(),
            normals: Vec::new(),
            texture_coords: Vec::new(),
            vertex_params: Vec::new(),
            faces: Vec::new(),
        }
    }
    pub fn vertices(&self) -> &Vec<ObjLine> {
        &self.vertices
    }
    pub fn normals(&self) -> &Vec<ObjLine> {
        &self.normals
    }

    pub fn material(&self) -> Option<String> {
        match &self.material {
            Some(ref value) => Some(value.clone()),
            None => None,
        }
    }
    pub fn name(&self) -> &Option<String> {
        &self.name
    }
    pub fn faces(&self) -> &Vec<ObjLine> {
        &self.faces
    }
}
