mod color;
mod vec3;
mod ray;
mod hittable;

use log::error;

use crate::color::utils::{Color, write_color};
use crate::vec3::utils::{Vec3, dot};
use crate::ray::utils::{Ray};
use crate::hittable::utils::{Hittable, HittableList, HitRecord, Sphere};

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    let mut record = HitRecord::new();
    if world.hit(ray, 0.0, f64::INFINITY, &mut record) {
        return 0.5 * Color::new(record.normal().x() + 1.0, record.normal().y() + 1.0, record.normal().z() + 1.0);
    }

    let unit_direction = ray.direction().unit_vector();
    let a = 0.5 * unit_direction.y() + 1.0;
    return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);

}

fn main() {
    env_logger::init();
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // World

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
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

    println!("P3\n{} {}\n255", image_width, image_height);
    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center = pixel_000_loc + pixel_delta_horizontal * i as f64 + pixel_delta_vertical * j as f64 ;
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&ray, &world);
            write_color(pixel_color);
        }
    }
}
