mod color;
mod vec3;
mod ray;
mod hittable;
mod interval;
mod camera;
mod material;

use crate::vec3::utils::{Vec3};
use crate::hittable::utils::{HittableList, Sphere};
use crate::camera::utils::Camera;
use crate::material::utils::{Metal, Lambertian};


fn main() {
    env_logger::init();

    let mut world = HittableList::new();
    let material_ground = Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Box::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_left = Box::new(Metal::new(Vec3::new(0.8, 0.8, 0.8)));
    let material_right = Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2)));
    
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, material_center)));
    world.add(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    // Camera
    let aspect_ratio = 16.0/9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth);

    camera.render(&world);
}
