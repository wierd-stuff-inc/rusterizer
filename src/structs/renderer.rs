use crate::structs::image::GlobImage;
use crate::structs::types::Color;
use crate::structs::types::Point;
// use std::fmt::Debug;

use crate::structs::types::Vec3f;
use std::f64;
// use std::fs::File;
// use std::io::prelude::*;
// use std::io::BufWriter;
use std::mem::swap;

// #[derive(Debug)]
pub struct Renderer<'a, T> {
    image: &'a mut T,
    z_buffer: Vec<f64>,
}

#[allow(dead_code)]
impl<'a, T: GlobImage> Renderer<'a, T> {
    pub fn new(image: &'a mut T) -> Self {
        let (width, height) = image.get_size();
        Renderer {
            image,
            z_buffer: vec![f64::MIN; (width * height) as usize],
        }
    }

    pub fn write_to_file(&self, filename: &str) {
        self.image.clone().save(filename);
    }

    fn set_deep_pixel(&mut self, x: u32, y: u32, z: f64, color: Color) {
        let (width, height) = self.image.clone().get_size();
        let id = (x + y * width) as usize;
        if ((id as usize) < ((width * height) as usize)) && (z > self.z_buffer[id]) {
            self.image.draw_pixel(x, y, color);
            self.z_buffer[id] = z;
        }
    }

    pub fn draw_line(&mut self, start: Point, end: Point, color: Color) {
        let mut steep = false;

        let (mut x0, mut y0) = (start.x as i32, start.y as i32);
        let (mut x1, mut y1) = (end.x as i32, end.y as i32);

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
                self.image.draw_pixel(y as u32, x as u32, color);
            } else {
                self.image.draw_pixel(x as u32, y as u32, color);
            }

            error += derror;

            if error > dx {
                y += if y1 > y0 { 1 } else { -1 };
                error -= dx * 2;
            }
        }
    }

    pub fn draw_triangle(&mut self, vert0: &Vec3f, vert1: &Vec3f, vert2: &Vec3f, color: Color) {
        let (width, height) = self.image.get_size();
        let f_width = f64::from(width);
        let f_height =f64::from(height);
        let x0 = (vert0.x + 1.0) * f_width / 2.0;
        let y0 = (vert0.y + 1.0) * f_height / 2.0;
        let y1 = (vert1.y + 1.0) * f_height / 2.0;
        let x1 = (vert1.x + 1.0) * f_width / 2.0;
        let y2 = (vert2.y + 1.0) * f_height / 2.0;
        let x2 = (vert2.x + 1.0) * f_width / 2.0;
        let a = Vec3f::new(x0, y0, 0.);
        let b = Vec3f::new(x1, y1, 0.);
        let c = Vec3f::new(x2, y2, 0.);
        let xs = vec![a.x, b.x, c.x];
        let ys = vec![a.y, b.y, c.y];
        let y_min = ys.iter().cloned().fold(f64::MAX, f64::min).floor() as i32;
        let y_max = ys.iter().cloned().fold(f64::MIN, f64::max).ceil() as i32;
        let x_min = xs.iter().cloned().fold(f64::MAX, f64::min).floor() as i32;
        let x_max = xs.iter().cloned().fold(f64::MIN, f64::max).ceil() as i32;
        for y in y_min..=y_max {
            let y = f64::from(y);
            for x in x_min..=x_max {
                let barry_x = f64::from(x);
                let l0 = ((y - y2) * (x1 - x2) - (barry_x - x2) * (y1 - y2))
                    / ((y0 - y2) * (x1 - x2) - (x0 - x2) * (y1 - y2));
                let l1 = ((y - y0) * (x2 - x0) - (barry_x - x0) * (y2 - y0))
                    / ((y1 - y0) * (x2 - x0) - (x1 - x0) * (y2 - y0));
                let l2 = 1. - l0 - l1;

                let barry_z = vert0.z * l0 + vert1.z * l1 + vert2.z * l2;

                if l0 > 0. && l1 > 0. && l2 > 0. {
                    self.set_deep_pixel(barry_x as u32, y as u32, barry_z, color);
                }
            }
        }
    }

    pub fn draw_poly(&mut self, vert0: &Vec3f, vert1: &Vec3f, vert2: &Vec3f, color: Color) {
        let v0 = vert0 - vert1;
        let v1 = vert2 - vert0;
        let n = -v0.cross(&v1).normalize();

        let intensity = n.dot(&Vec3f::new(0., 0., 1.));

        let color = (
            (f64::from(color.0) * intensity) as u8,
            (f64::from(color.1) * intensity) as u8,
            (f64::from(color.2) * intensity) as u8,
        );

        if intensity > 0. {
            self.draw_triangle(vert0, vert1, vert2, color);
        }
    }
}
