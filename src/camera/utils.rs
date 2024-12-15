use std::fmt::format;
use std::sync::Arc;
use std::thread;
use log::error;
use std::time::Instant;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use rand::Rng;
use rayon::prelude::*;
use crate::color::utils::{Color, write_color};
use crate::vec3::utils::{Vec3, Point3};
use crate::hittable::utils::{HitRecord, Hittable, HittableList};
use crate::interval::utils::Interval;
use crate::ray::utils::Ray;

#[derive(Clone)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    image_height: i32,
    pixel_samples_scale: f64,
    center: Point3,
    pixel_000_loc: Point3,
    pixel_delta_horizontal: Vec3,
    pixel_delta_vertical: Vec3,
}

impl Camera {
    pub fn sample_square() -> Point3 {
        // Sample between [-.5, -.5] -[+.5, +.5]
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0.0..1.0);
        let y = rng.gen_range(0.0..1.0);
        Point3::new(x - 0.5, y - 0.5, 0.0)
    }
    pub fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = Camera::sample_square();
        let pixel_center = self.pixel_000_loc +
            self.pixel_delta_horizontal * (i as f64 + offset.x() )  +
            self.pixel_delta_vertical * (j as f64 + offset.y());
        let ray_direction = pixel_center - self.center;
        Ray::new(self.center, ray_direction)
    }
    pub fn render(&self, world: Arc<dyn Hittable>) {
        // Variables for progress bar
        let start = Instant::now();
        let progress_bar = ProgressBar::new((self.image_height * self.image_width) as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                .unwrap()
                .progress_chars("##-"),
        );
        
        let rows: Vec<String> = (0..self.image_height)
            .into_par_iter()
            .map(|j| {
                let mut row: String = "".to_string();
                for i in 0..self.image_width {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for _sample in 0..self.samples_per_pixel {
                        let ray = self.get_ray(i, j);
                        pixel_color += Camera::ray_color(&ray, self.max_depth, world.clone());
                    }
                    row += &write_color(pixel_color / self.samples_per_pixel as f64);
                    row += "\n";
                    progress_bar.inc(1);
                }
                row
            })
            .collect();
        
        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        for row in rows {
            println!("{}", row);
        }
        let duration = start.elapsed();
        progress_bar.finish_with_message(format!("Rendering Complete in {:?}", duration));

    }

    pub fn new(aspect_ratio: f64, image_width: i32, samples_per_pixel: i32, max_depth: i32) -> Camera {
        let image_height = (image_width as f64 / aspect_ratio) as i32;
        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let camera_center = Vec3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, -viewport_height, 0.0);
        let pixel_delta_horizontal = horizontal / image_width as f64;
        let pixel_delta_vertical = vertical / image_height as f64;
        let viewport_upper_left = camera_center 
            - Vec3::new(0.0, 0.0, focal_length) - horizontal / 2.0 - vertical / 2.0;
        let pixel_000_loc = viewport_upper_left + 0.5 * (pixel_delta_horizontal +  pixel_delta_vertical);

        Camera {
            aspect_ratio,
            image_width,
            max_depth,
            samples_per_pixel,
            image_height,
            pixel_samples_scale,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_horizontal,
            pixel_delta_vertical,
            pixel_000_loc,
        }
    }

    pub fn ray_color(ray: &Ray, depth : i32, world: Arc<dyn Hittable>) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        let mut record = HitRecord::new();
        // Sometimes the ray hits the object at t = 0.0, which causes the shadow acne problem
        let sight_t = Interval::new(0.001, f64::INFINITY);
        if world.hit(ray, &sight_t, &mut record) {
            let direction = record.normal() + Vec3::random_unit_vector_in_unit_sphere();
            let mut scattered = Ray::new(record.p(), direction);
            let mut attenuation = Color::new(0.0, 0.0, 0.0);
            
            match record.material() {
                Some(material) => {
                    if material.scatter(ray, &record, &mut attenuation, &mut scattered) {
                        return Camera::ray_color(&scattered, depth - 1, world) * attenuation;
                    }
                    return Color::new(0.0, 0.0, 0.0);
                },
                None => {
                    error!("No material found for an object\n");
                    return Color::new(0.0, 0.0, 0.0);
                }
            }
        }

        let unit_direction = ray.direction().unit_vector();
        let a = 0.5 * unit_direction.y() + 1.0;
        return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
    }



}