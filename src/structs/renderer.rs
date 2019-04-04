use crate::structs::image::GlobImage;
use crate::structs::types::Color;
use crate::structs::types::Point;
use nalgebra::geometry::Rotation3;
use nalgebra::Matrix3;
use nalgebra::Matrix3x4;
// use std::fmt::Debug;

use crate::structs::types::Vec3f;
use std::f64;
// use std::fs::File;
// use std::io::prelude::*;
// use std::io::BufWriter;
use std::mem::swap;

// #[derive(Debug)]
/// Структурка, чтобы рисовать всяких 3D хлопчиков.
pub struct Renderer<'a, T> {
    image: &'a mut T,
    z_buffer: Vec<f64>,
    diffuse: &'a T,
    rotate: Matrix3<f64>,
    movement: Vec3f,
}

lazy_static! {
    static ref KMATRIX: Matrix3<f64> = Matrix3::new(1024., 0., 256., 0., -1024., 256., 0., 0., 1.);
}

#[allow(dead_code)]
impl<'a, T: GlobImage> Renderer<'a, T> {
    pub fn new(image: &'a mut T, diffuse: &'a T, angles: Vec3f, movement: Vec3f) -> Self {
        let (width, height) = image.get_size();
        let rotate = Rotation3::from_euler_angles(angles.x, angles.y, angles.z).into_inner();
        Renderer {
            image,
            z_buffer: vec![f64::MIN; (width * height) as usize],
            diffuse,
            rotate,
            movement,
        }
    }
    /// Сохранить изображение в файл.
    /// * `filename` -> Имя файла, куда следует сохранить
    pub fn write_to_file(&self, filename: &str) {
        self.image.clone().save(filename);
    }
    /// Выставить цвет пикселя с координатами x, y,
    /// учитывая значение z-буффера для этой точки.
    ///
    /// * `x` - координата по x.
    /// * `y` - координата по y.
    /// * `z` - новое значение z-буффера.
    /// * `color` - цвет точки.
    fn set_deep_pixel(&mut self, x: u32, y: u32, z: f64, color: Color) {
        let (width, height) = self.image.clone().get_size();
        let id = (x + y * width) as usize;
        if ((id as usize) < ((width * height) as usize)) && (z > self.z_buffer[id]) {
            // let baba = (((z * 126.) as i32) % 255) as u8;
            // let tri_babi = (baba, baba, baba);
            // eprintln!("{:?}", z);
            self.image.draw_pixel(x, y, color);
            self.z_buffer[id] = z;
        }
    }

    /// Нарисовать прямую линию.
    /// * `start` -> Точка начала линии.
    /// * `end`   -> Точка конца линии.
    /// * `color` -> Цвет
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

    /// Нарисовать и закрасить некоторый тряухольник.
    /// * `vert0` -> 3D позиция первой точки.
    /// * `vert1` -> 3D позиция второй точки.
    /// * `vert2` -> 3D позиция третьей точки.
    /// * `texture_mapping` -> Функция для нахождения позиции пикселя с цветом файле текстуры.
    /// * `intensity_mapping` -> Функция вычисления интенсивности освещения.
    pub fn draw_poly(
        &mut self,
        vert0: &Vec3f,
        vert1: &Vec3f,
        vert2: &Vec3f,
        texture_mapping: impl Fn(f64, f64, f64) -> (f64, f64),
        intensity_mapping: impl Fn(f64, f64, f64) -> f64,
    ) {
        let (width, height) = self.image.get_size();
        let kk_vert0 = self.rotate * vert0 + self.movement;
        let kk_vert1 = self.rotate * vert1 + self.movement;
        let kk_vert2 = self.rotate * vert2 + self.movement;
        let k_vert0 = (*KMATRIX) * kk_vert0;
        let x0 = k_vert0.x / k_vert0.z;
        let y0 = k_vert0.y / k_vert0.z;
        let k_vert1 = (*KMATRIX) * kk_vert1;
        let x1 = k_vert1.x / k_vert1.z;
        let y1 = k_vert1.y / k_vert1.z;
        let k_vert2 = (*KMATRIX) * kk_vert2;
        let x2 = k_vert2.x / k_vert2.z;
        let y2 = k_vert2.y / k_vert2.z;
        let a = Vec3f::new(x0, y0, 0.);
        let b = Vec3f::new(x1, y1, 0.);
        let c = Vec3f::new(x2, y2, 0.);
        let xs = vec![a.x, b.x, c.x];
        let ys = vec![a.y, b.y, c.y];
        let y_min = ys.iter().cloned().fold(f64::MAX, f64::min).floor() as i32;
        let y_max = ys.iter().cloned().fold(f64::MIN, f64::max).ceil() as i32;
        let y_min = y_min.max(0).min(height as i32);
        let y_max = y_max.max(0).min(height as i32);
        let x_min = xs.iter().cloned().fold(f64::MAX, f64::min).floor() as i32;
        let x_max = xs.iter().cloned().fold(f64::MIN, f64::max).ceil() as i32;
        let x_min = x_min.max(0).min(width as i32);
        let x_max = x_max.max(0).min(width as i32);
        // eprintln!("{:?}", (y_min, y_max));
        // eprintln!("{:?}", (x_min, x_max));
        for y in y_min..=y_max {
            let y = f64::from(y);
            for x in x_min..=x_max {
                let barry_x = f64::from(x);
                let l0 = ((y - y2) * (x1 - x2) - (barry_x - x2) * (y1 - y2))
                    / ((y0 - y2) * (x1 - x2) - (x0 - x2) * (y1 - y2));
                let l1 = ((y - y0) * (x2 - x0) - (barry_x - x0) * (y2 - y0))
                    / ((y1 - y0) * (x2 - x0) - (x1 - x0) * (y2 - y0));
                let l2 = 1. - l0 - l1;

                let barry_z = k_vert0.z * l0 + k_vert1.z * l1 + k_vert2.z * l2;

                if l0 > 0. && l1 > 0. && l2 > 0. {
                    let (u, v) = texture_mapping(l0, l1, l2);

                    let (diffuse_width, diffuse_height) = self.diffuse.get_size();

                    let diffuse_x = (u * f64::from(diffuse_width)) as u32;
                    let diffuse_y = (v * f64::from(diffuse_height)) as u32;

                    let color = self.diffuse.get_pixel(diffuse_x, diffuse_y);
                    let intensity = intensity_mapping(l0, l1, l2);
                    if intensity > 0. {
                        let color = (
                            (f64::from(color.0) / 255.0 * intensity) as u8,
                            (f64::from(color.1) / 255.0 * intensity) as u8,
                            (f64::from(color.2) / 255.0 * intensity) as u8,
                        );
                        self.set_deep_pixel(barry_x as u32, y as u32, barry_z, color);
                    }
                }
            }
        }
    }
}
