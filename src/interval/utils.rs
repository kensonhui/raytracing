

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn inclusive_contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn exclusive_contains(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}