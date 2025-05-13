#![allow(dead_code)]

use std::f32::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    pub fn new(xv: f32, yv: f32) -> Self {
        Self { x: xv, y: yv }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len == 0.0 {
            *self
        } else {
            Self {
                x: self.x / len,
                y: self.y / len,
            }
        }
    }
}

pub fn lerp(a: Vec2, b: Vec2, t: f32) -> Vec2 {
    Vec2 {
        x: a.x + (b.x - a.x) * t,
        y: a.y + (b.y - a.y) * t,
    }
}

pub fn grad_to_rad(ang: f32) -> f32 {
    ang / (180 as f32) * PI
}

pub fn rad_to_grad(ang: f32) -> f32 {
    ang * (180 as f32) / PI
}
