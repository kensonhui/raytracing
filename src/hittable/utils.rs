use crate::material;
use crate::ray::utils::Ray;
use crate::vec3::utils::{Vec3, dot};
use crate::interval::utils::Interval;
use crate::material::utils::Material;

pub trait Hittable : Sync + Send{
    fn hit(&self, r: &Ray, t: &Interval, hit_record: &mut HitRecord) -> bool;
    fn material(&self) -> Option<Box<dyn Material>>;
} 

pub struct HitRecord{
    p: Vec3, // point of intersection
    normal: Vec3,
    t: f64, // time of intersection
    material: Option<Box<dyn Material>>,
    front_face: bool,
}

impl Clone for HitRecord {
    fn clone(&self) -> Self {
        HitRecord {
            p: self.p,
            normal: self.normal,
            t: self.t,
            material: self.material.clone(),
            front_face: self.front_face,
        }
    }
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord{
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            material: None,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        // ray is inside the sphere if the ray is facing the same way as the outward normal
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }

    pub fn p(&self) -> Vec3 {
        self.p
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }

    pub fn material(&self) -> Option<Box<dyn Material>> {
        self.material.clone()
    }
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: Vec::new() }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t: &Interval, hit_record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_t = Interval::new(t.min(), t.max());
        for object in &self.objects {
            if object.hit(r, &closest_t, &mut temp_record) {
                hit_anything = true;
                closest_t.max = temp_record.t();
                *hit_record = temp_record.clone();
            }
        }

        return hit_anything;
    }

    fn material(&self) -> Option<Box<dyn Material>> {
        None
    }
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Box<dyn Material>
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere { center, radius, material }
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool {
        let oc = self.center - ray.origin();
        let a = ray.direction().length_squared();
        let h = dot(ray.direction(), oc);
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }     

        let discriminant_squareroot = discriminant.sqrt();

        let root = (h - discriminant_squareroot) / a;
        if !ray_t.exclusive_contains(root) {
            let root = (h + discriminant_squareroot) / a;
            if !ray_t.exclusive_contains(root) {
                return false;
            }
        }
        
        hit_record.t = root;
        hit_record.p = ray.at(hit_record.t);
        hit_record.normal = (hit_record.p - self.center) / self.radius;
        hit_record.material = self.material();
        return true;
    }

    fn material(&self) -> Option<Box<dyn Material>> {
        Some(self.material.clone())
    }
}