pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn _default() -> Self {
        Self {
            min: f64::INFINITY,
            max: -f64::INFINITY,
        }
    }
    pub fn new(min: f64, max: f64) -> Self {
        Self {
            min,
            max,
        }
    }
    pub fn _size(&self) -> f64 {
        self.max - self.min
    }
    pub fn _contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
    pub fn _empty() -> Self {
        Interval::new(f64::INFINITY, -f64::INFINITY)
    }
    pub fn _universe() -> Self {
        Interval::new(-f64::INFINITY, f64::INFINITY)
    }
}
