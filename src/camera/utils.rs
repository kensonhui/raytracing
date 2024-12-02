use log::{debug, error, info};
use std::error;
use std::io::Write;
use crate::color::utils::{Color, write_color};
use crate::vec3::utils::{Vec3, Point3, dot};
use crate::hittable::utils::{Hittable, HitRecord};
use crate::interval::utils::{Interval};
use crate::ray::utils::Ray;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    image_height: i32,
    center: Point3,
    pixel_000_loc: Point3,
    pixel_delta_horizontal: Vec3,
    pixel_delta_vertical: Vec3,
}

impl Camera {
    pub fn render(&self, world: &dyn Hittable) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        for j in 0..self.image_height {
            error!("\x1B[1K\rScanlines remaining: {}\x1B[F", self.image_height - j);
            for i in 0..self.image_width {
                let pixel_center = self.pixel_000_loc + self.pixel_delta_horizontal * i as f64 + self.pixel_delta_vertical * j as f64 ;
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction); let pixel_color = Camera::ray_color(&ray, world);
                write_color(pixel_color);
            }
        }
        error!("\x1B[1K\rDone rendering\x1B[F");
        
    }

    pub fn new(aspect_ratio: f64, image_width: i32) -> Camera {
        let image_height = (image_width as f64 / aspect_ratio) as i32;
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
            image_height,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_horizontal,
            pixel_delta_vertical,
            pixel_000_loc,
        }
    }

    pub fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
        let mut record = HitRecord::new();
        let sight_t = Interval::new(0.0, f64::INFINITY);
        if world.hit(ray, &sight_t, &mut record) {
            return 0.5 * Color::new(record.normal().x() + 1.0, record.normal().y() + 1.0, record.normal().z() + 1.0);
        }

        let unit_direction = ray.direction().unit_vector();
        let a = 0.5 * unit_direction.y() + 1.0;
        return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
    }

}