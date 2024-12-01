use std::ops::{Add, Sub, Mul, Div};
use crate::vec3::utils::Vec3;
use crate::ray::utils::Ray;

pub type Color = Vec3;

impl Color {

    pub fn r(&self) -> f64 {
        self.x
    }

    pub fn g(&self) -> f64 {
        self.y
    }

    pub fn b(&self) -> f64 {
        self.z
    }
}

pub fn write_color(pixel_color: Color) {
    println!("{} {} {}", 
        (255.999 * pixel_color.r()) as u8, 
        (255.999 * pixel_color.g()) as u8, 
        (255.999 * pixel_color.b()) as u8
    );
}

