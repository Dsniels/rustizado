use std::f64::consts::PI;
use super::collisions::Colliadable;

use super::area::Area;
use super::rectangle::Rectangle;

pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

impl Circle {
    pub fn contains_point(&self, (x, y): (f64, f64)) -> bool {
        let x = self.x - x;
        let y = self.y - y;
        return x * x + y * y <= self.radius * self.radius;
    }

}

impl Colliadable<Rectangle> for Rectangle {

    fn collide(&self, other: &Rectangle)->bool {
        for point in other{
            if self.contains_point(point){
                return true;
            }
        }
        return false;
    }
    
}
impl Area for Circle {
    fn area(&self) -> f64 {
        return self.radius * self.radius * PI;
    }
}
