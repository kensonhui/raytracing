use std::sync::Arc;
use std::thread;
use log::error;
use std::time::Instant;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use rand::Rng;
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
        let progress = Arc::new(AtomicUsize::new(0));
        let total_progress_bars = 20;
        
        // Shared variables
        let self_arc = Arc::new(self.clone());
        let image_height = self.image_height;
        let image_width = self.image_width;
        let samples_per_pixel = self.samples_per_pixel;
        let max_depth = self.max_depth;
        let mut handles: Vec<thread::JoinHandle<()>> = vec![];
        let shared_array = Arc::new(Mutex::new(vec![String::new(); self.image_height as usize]));
        
        for j in 0..image_height {
            let self_arc = Arc::clone(&self_arc);
            let shared_array = Arc::clone(&shared_array);
            let world = Arc::clone(&world);
            let progress = Arc::clone(&progress);
            let handle = thread::spawn(move || {
                let mut row: String = "".to_string();
                for i in 0..image_width {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for _sample in 0..samples_per_pixel {
                        let ray = self_arc.get_ray(i, j);
                        pixel_color += Camera::ray_color(&ray, max_depth, world.clone());
                    }
                    row += &write_color(pixel_color / samples_per_pixel as f64);
                    row += "\n";
                }
                // Copy the row to the shared array
                let mut shared_array = shared_array.lock().unwrap();
                shared_array[j as usize] = row;

                // Update progress bar
                let progress_value = progress.fetch_add(1, Ordering::SeqCst);
                let bars = ">".repeat(progress_value / total_progress_bars as usize) + &" ".repeat(image_height as usize / total_progress_bars - progress_value / total_progress_bars as usize);
                error!("\x1B[1K\rRendering Progress: [{}] - {:.1}%\x1B[F", bars, (progress_value as f64 / image_height as f64) * 100.0);
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        let shared_array = shared_array.lock().unwrap();
        println!("P3\n{} {}\n255", image_width, image_height);
        for row in shared_array.iter() {
            println!("{}", row);
        }
        let duration = start.elapsed();
        error!("\x1B[1K\rDone rendering in {:?}", duration);
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