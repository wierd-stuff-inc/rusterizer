use std::fs::File;
use std::io::BufReader;

struct Parser {
    reader: BufReader<File>,
}

impl Parser {
    fn new(filename: &str) -> Self {
        let reader = BufReader::new(File::open(filename).expect("Unable to open file"));
        Parser { reader }
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
