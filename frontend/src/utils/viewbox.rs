use std::ops::{Add, Sub};
use std::cmp::{min, max};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ViewBox {
    pub top_left: Point,
    pub w: u32,
    pub h: u32,
    pub zoom_in_limit: u32,
    pub zoom_out_limit: u32,
}

impl ViewBox {
    fn check_zoom_limits(&self, scale: f32) -> bool{
        if self.zoom_in_limit == 0 && self.zoom_out_limit == 0 {
            return true;
        }
        if min(self.w, self.h) <= self.zoom_in_limit && scale > 1.0 {
            return false;
        }
        if max(self.w, self.h) >= self.zoom_out_limit && scale < 1.0 {
            return false;
        }
        true
    }

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
        if !self.check_zoom_limits(scale) {
            return;
        }

        self.w = (self.w as f32 / scale) as u32;
        self.h = (self.h as f32 / scale) as u32;
        self.top_left = zoom_center
            - Point {
                x: (self.w / 2) as i32,
                y: (self.h / 2) as i32,
            };
    }

    pub fn zoom_to_center(&mut self, scale: f32) {
        let zoom_center = self.top_left
            + Point {
                x: (self.w / 2) as i32,
                y: (self.h / 2) as i32,
            };
        self.zoom_to(zoom_center, scale);
    }

    pub fn drag(&mut self, delta: Point) {
        self.top_left = self.top_left + delta;
    }
}
