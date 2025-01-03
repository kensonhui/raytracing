use crate::ray::utils::Ray;
use crate::color::utils::Color;
use crate::hittable::utils::HitRecord;
use crate::vec3::utils::Vec3;

pub trait Material : Sync + Send{
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
    fn clone_box(&self) -> Box<dyn Material>;
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.clone_box()
    }
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(a: Color) -> Lambertian {
        Lambertian { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal() + Vec3::random_on_unit_sphere(&rec.normal());
        
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal();
        }
        *scattered = Ray::new(rec.p(), scatter_direction);
        *attenuation = self.albedo;
        true
    }

    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(*self)
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(a: Color, fuzz: f64) -> Metal {
        Metal {
            albedo: a, 
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = r_in.direction().reflect(rec.normal()) + self.fuzz * Vec3::random_unit_vector_in_unit_sphere();
        *scattered = Ray::new(rec.p(), reflected);
        *attenuation = self.albedo;
        return true;
    }

    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(*self)
    }
}