mod color;
mod vec3;

use crate::color::utils::write_color;
use crate::vec3::utils::Vec3;

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n255", image_width, image_height);
    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_color = Vec3::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                1.0,
            );
            write_color(pixel_color);
        }
    }
}
