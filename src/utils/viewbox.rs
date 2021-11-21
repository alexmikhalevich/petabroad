use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ViewBox {
    pub top_left: Point,
    pub w: u32,
    pub h: u32,
}

impl ViewBox {
    pub fn to_string(&self) -> String {
        format!(
            "{} {} {} {}",
            self.top_left.x.clone(),
            self.top_left.y.clone(),
            self.w.clone(),
            self.h.clone()
        )
    }

    pub fn zoom_to(&mut self, zoom_center: Point, scale: f32) {
        self.w = (self.w as f32 / scale) as u32;
        self.h = (self.h as f32 / scale) as u32;
        self.top_left = zoom_center
            - Point {
                x: (self.w / 2) as i32,
                y: (self.h / 2) as i32,
            };
    }

    pub fn zoom_to_center(&mut self, scale: f32) {
        let zoom_center = self.top_left + Point {
            x: (self.w / 2) as i32,
            y: (self.h / 2) as i32,
        };
        self.zoom_to(zoom_center, scale);
    }
}
