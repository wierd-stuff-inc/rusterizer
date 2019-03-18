use crate::structs::types::Color;
// use std::fmt::Debug;
//
// pub enum ImageError {
//     IOError,
// }

// pub type ImageResult = Result<GlobImage, ImageError>;

pub trait GlobImage: Sized + Clone {
    fn draw_pixel(&mut self, x: u32, y: u32, color: Color);
    fn get_pixel(&self, x: u32, y: u32) -> Color;
    fn save(self, filename: &str);
    fn load(filename: &str) -> Self;
    /// Return tuple (Width, Height)
    fn get_size(&self) -> (u32, u32);
}
