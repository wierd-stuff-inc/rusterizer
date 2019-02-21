use crate::structs::types::Color;
use crate::structs::types::Point;

use crate::structs::types::Vec3i;
use std::fs::File;
use std::i32;
use std::io::prelude::*;
use std::io::BufWriter;
use std::mem::swap;

#[derive(Debug, PartialEq)]
pub struct PPMImage {
    data: Vec<Color>,
    width: u32,
    height: u32,
}
#[allow(dead_code)]
impl PPMImage {
    pub fn new(width: u32, height: u32) -> PPMImage {
        PPMImage {
            data: vec![(0, 0, 0); (width * height) as usize],
            width,
            height,
        }
    }

    pub fn write_to_file(&self, filename: &str) {
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

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let id = x + y * self.width;
        if (id as usize) < self.data.len() {
            self.data[id as usize] = color;
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        let id = x + y * self.width;
        self.data[id as usize]
    }

    pub fn draw_line(&mut self, start: Point, end: Point, color: Color) {
        let mut steep = false;

        let (mut x0, mut y0) = (start.x, start.y);
        let (mut x1, mut y1) = (end.x, end.y);

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

    pub fn draw_triangle(&mut self, p0: Point, p1: Point, p2: Point, color: Color) {
        let a = Vec3i::new(p0.x, p0.y, 0);
        let b = Vec3i::new(p1.x, p1.y, 0);
        let c = Vec3i::new(p2.x, p2.y, 0);
        let xs = vec![a.x, b.x, c.x];
        let ys = vec![a.y, b.y, c.y];
        let y_min = ys.iter().cloned().fold(i32::MAX, i32::min);
        let y_max = ys.iter().cloned().fold(i32::MIN, i32::max);
        let x_min = xs.iter().cloned().fold(i32::MAX, i32::min);
        let x_max = xs.iter().cloned().fold(i32::MIN, i32::max);
        let v0 = b - a;
        let v1 = c - a;
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                let p = Vec3i::new(x, y, 0);
                let v2 = p - a;
                let d00 = v0.dot(&v0) as f32;
                let d01 = v0.dot(&v1) as f32;
                let d11 = v1.dot(&v1) as f32;
                let d20 = v2.dot(&v0) as f32;
                let d21 = v2.dot(&v1) as f32;
                let denom = d00 * d11 - d01 * d01;
                let v = (d11 * d20 - d01 * d21) / denom;
                let w = (d00 * d21 - d01 * d20) / denom;
                let u = 1.0 - v - w;
                // println!("v = {:?}, w = {:?}, u = {:?};", v, w, u);
                // assert_eq!(v + w + u < 1. + 1e-6, true);
                if u > 0. && w > 0. && v > 0.{
                    self.set_pixel(x as u32, y as u32, color);
                }
            }
        }
    }
}
