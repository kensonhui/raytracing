mod color;
mod vec3;
mod ray;
mod hittable;
mod interval;
mod camera;

use log::error;

use crate::vec3::utils::{Vec3, dot};
use crate::hittable::utils::{HittableList, Sphere};
use crate::camera::utils::Camera;


fn main() {
    env_logger::init();

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let aspect_ratio = 16.0/9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let camera = Camera::new(aspect_ratio, image_width, samples_per_pixel);

    camera.render(&world);
}
