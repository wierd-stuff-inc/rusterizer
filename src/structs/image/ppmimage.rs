use crate::structs::image::glob_image::GlobImage;
use crate::structs::types::Color;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::string::String;

#[derive(Debug, PartialEq, Clone)]
pub struct PPMImage {
    data: Vec<Color>,
    width: u32,
    height: u32,
}

impl PPMImage {
    pub fn new(width: u32, height: u32) -> Self {
        let black_color = (0, 0, 0);
        PPMImage {
            data: vec![black_color; (width * height) as usize],
            width,
            height,
        }
    }
}

impl GlobImage for PPMImage {
    fn draw_pixel(&mut self, x: u32, y: u32, color: Color) {
        let id = x + y * self.width;
        if (id as usize) < self.data.len() {
            self.data[id as usize] = color;
        }
    }

    fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn load(filename: &str) -> Self {
        let file = File::open(filename).expect("Can't read file.");
        let reader = BufReader::new(file);
        let mut data = reader.lines().flat_map(|line| line);
        assert_eq!(data.next(), Some(String::from("P3")));
        let width = data
            .next()
            .map(|el| el.parse::<u32>().expect("Can't parse width."))
            .expect("Can't read height.");
        let height = data
            .next()
            .map(|el| el.parse::<u32>().expect("Can't parse height."))
            .expect("Can't read number.");

        let _number_of_colors = data.next(); // skipping-peeping number of colors (good example of lexical reduplication);

        let pixels: Vec<_> = data
            .map(|el| el.parse::<u8>().expect("Can't parse pixel."))
            .collect::<Vec<u8>>()
            .chunks(3)
            .map(|el| (el[0], el[1], el[2]))
            .collect();

        assert_eq!(pixels.len(), (width * height) as usize);

        PPMImage {
            data: pixels,
            width,
            height,
        }
    }

    fn get_pixel(&self, x: u32, y: u32) -> Color {
        let id = x + y * self.width;
        self.data[id as usize]
    }

    fn save(self, filename: &str) {
        let file = File::create(filename).expect("Can't open file.");
        let mut buffer = BufWriter::new(file);
        buffer
            .write_fmt(format_args!("P3\n{} {}\n255\n", self.width, self.height))
            .expect("Can't write to file.");
        // write!(buffer, );
        for i in 0..self.height {
            for j in 0..self.width {
                if j > 0 {
                    buffer.write_all(b" ").expect("Can't write to file.");
                }
                let id = j + (self.height - i - 1) * self.width;
                let (r, g, b) = self.data[id as usize];
                buffer
                    .write_fmt(format_args!("{} {} {}", r, g, b))
                    .expect("Can't write to file.");
            }
            buffer.write_all(b"\n").expect("Can't write to file.");
        }
    }
}
