use crate::interval::utils::Interval;
use crate::vec3::utils::Vec3;

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

pub fn linear_to_gamma(value: f64) -> f64 {
    if value < 0.0 {
        return 0.0;
    } else {
        return value.sqrt();
    }
}

pub fn write_color(pixel_color: Color) -> String {
    let intensity = Interval::new(0.0, 0.999);
    return format!("{} {} {}", 
        (255.0 * intensity.clamp(linear_to_gamma(pixel_color.r()))), 
        (255.0 * intensity.clamp(linear_to_gamma(pixel_color.g()))), 
        (255.0 * intensity.clamp(linear_to_gamma(pixel_color.b())))
    );
}

