use std::{fmt::Display, os::windows::{self, process}};

use super::area::Area;

pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn contains_point(&self, (x, y): (f64, f64)) -> bool {
        return self.x <= x && self.x + self.width >= x && 
                self.y <= y && self.y + self.height >= y;
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        return self.width * self.height;
    }
}

impl Display for Rectangle {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "Rectangle({},{}):{}x{}",
            self.x, self.y, self.height, self.width
        );
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        return Rectangle {
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 5.0,
        };
    }
}

#[derive(Debug)]
pub struct RectIter {
    points: Vec<(f64, f64)>,
    idx: usize,
}

impl RectIter {
    fn new(rec: &Rectangle) -> Self {
        return RectIter {
            points: vec![
                (rec.x, rec.y),
                (rec.x + rec.width, rec.y),
                (rec.x, rec.y + rec.height),
                (rec.x + rec.width, rec.y + rec.height),
            ],
            idx: 0,
        };
    }
}

impl Iterator for RectIter {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx;
        self.idx += 1;

        return self.points.get(idx).map(|x| *x);
    }
}

impl IntoIterator for Rectangle {
    type Item = (f64, f64);
    type IntoIter = RectIter;
    fn into_iter(self) -> Self::IntoIter {
        return RectIter {
            points: vec![
                (self.x, self.y),
                (self.x + self.width, self.y),
                (self.x, self.y + self.height),
                (self.x + self.width, self.y + self.height),
            ],
            idx: 0,
        };
    }
}

impl IntoIterator for &Rectangle {
    type Item = (f64, f64);
    type IntoIter = RectIter;
    fn into_iter(self) -> Self::IntoIter {
        return RectIter::new(self);
    }
}
