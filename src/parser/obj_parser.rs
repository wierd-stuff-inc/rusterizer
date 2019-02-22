use crate::parser::obj_line::*;
use std::fs::File;
use std::io::BufReader;

pub struct ObjParser {
    filename: String,
    reader: BufReader<File>,
}

impl ObjParser {
    pub fn create(filename: &str) -> Self {
        let reader = BufReader::new(File::open(filename).expect("Unable to open file"));
        ObjParser {
            filename: filename.to_string(),
            reader,
        }
    }

    pub fn get_filename(&self) -> &str {
        &self.filename
    }
}

impl Iterator for ObjParser {
    type Item = ObjLine;

    fn next(&mut self) -> Option<Self::Item> {
        use std::io::BufRead;
        let mut line = String::new();
        let read_result = self.reader.read_line(&mut line);
        match read_result {
            Ok(len) => {
                if len > 0 {
                    let result = parse_obj_line(&line);
                    match result {
                        Result::Ok(v) => Some(v.1),
                        Result::Err(e) => {
                            println!("{:#?}", e);
                            if e.is_incomplete() {
                                self.next()
                            } else {
                                None
                            }
                        }
                    }
                } else {
                    None
                }
            }
            Err(_o) => None,
        }
    }
}
