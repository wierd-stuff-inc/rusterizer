use crate::structs::types::Line;

#[derive(Debug, PartialEq)]
pub struct Face {
    lines: [Line; 3],
}
