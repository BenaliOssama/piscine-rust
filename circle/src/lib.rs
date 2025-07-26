use std::f64::consts;

const PI : f64  = consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
	 pub center: Point,
	 pub radius: f64,
}

impl Circle {
    pub fn new(x : f64, y : f64 , r : f64)-> Self {
        Self{
            center : Point(x, y), 
            radius : r,
        } 
    }

    pub fn diameter(self) -> f64 {
       2.0 * self.radius * PI
    }

    pub fn area(self) -> f64 {
       self.radius.powf(2.0) * PI
    }

    pub fn intersect(self, other: Circle) -> bool {
       self.center.distance(other.center) < self.radius + other.radius
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64,pub f64);

impl Point {
    pub fn distance(self, other: Point) -> f64 {
        ((self.0 - other.0).powf(2.0) + (self.1 - self.1).powf(2.0)).sqrt()
    }
}
