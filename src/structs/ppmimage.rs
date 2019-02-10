use crate::structs::types::Color;
use crate::structs::types::Point;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::mem::swap;

#[derive(Debug, PartialEq)]
pub struct PPMImage {
    data: Vec<Color>,
    width: u32,
    height: u32,
}

impl PPMImage {
    pub fn new(width: u32, height: u32) -> PPMImage {
        PPMImage {
            data: vec![(0, 0, 0); (width * height) as usize],
            width: width,
            height: height,
        }
    }

    pub fn write_to_file(&self, filename: &str) {
        let file = File::create(filename).unwrap();
        let mut buffer = BufWriter::new(file);
        write!(buffer, "P3\n{} {}\n255\n", self.width, self.height);
        for i in 0..self.height {
            for j in 0..self.width {
                if j > 0 {
                    write!(buffer, " ");
                }
                let id = j + (self.height - i - 1) * self.width;
                let (r, g, b) = self.data[id as usize];
                write!(buffer, "{} {} {}", r, g, b);
            }
            write!(buffer, "\n");
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let id = x + y * self.width;
        self.data[id as usize] = color;
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        let id = x + y * self.width;
        self.data[id as usize]
    }

    pub fn draw_line(&mut self, start: Point, end: Point, color: Color) {
        let mut steep = false;

        let (mut x0, mut y0) = start;
        let (mut x1, mut y1) = end;

        if (x0 - x1).abs() < (y0 - y1).abs() {
            swap(&mut x0, &mut y0);
            swap(&mut x1, &mut y1);
            steep = true;
        }

        if x0 > x1 {
            swap(&mut x0, &mut x1);
            swap(&mut y0, &mut y1);
        }
        let dx = x1 - x0;
        let dy = y1 - y0;
        let derror = dy.abs() * 2;
        let mut error = 0;
        let mut y = y0;
        for x in x0..=x1 {
            if steep {
                self.set_pixel(y as u32, x as u32, color);
            } else {
                self.set_pixel(x as u32, y as u32, color);
            }

            error += derror;

            if error > dx {
                y += if y1 > y0 { 1 } else { -1 };
                error -= dx * 2;
            }
        }
    }
}
