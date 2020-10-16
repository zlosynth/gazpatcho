// TODO: Turn this into a wrapper over [f32; 2] and introduce deref. That may help to get rid of .into()
use std::convert::From;
use std::ops::{Add, AddAssign};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl Default for Vec2 {
    fn default() -> Self {
        Self::zero()
    }
}

impl From<[f32; 2]> for Vec2 {
    fn from(a: [f32; 2]) -> Self {
        Self { x: a[0], y: a[1] }
    }
}

impl From<Vec2> for [f32; 2] {
    fn from(a: Vec2) -> Self {
        [a.x, a.y]
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<[f32; 2]> for Vec2 {
    type Output = Vec2;

    fn add(self, other: [f32; 2]) -> Self::Output {
        Vec2 {
            x: self.x + other[0],
            y: self.y + other[1],
        }
    }
}

impl AddAssign<[f32; 2]> for Vec2 {
    fn add_assign(&mut self, other: [f32; 2]) {
        *self = *self + other;
    }
}

impl Add<Vec2> for [f32; 2] {
    type Output = [f32; 2];

    fn add(self, other: Vec2) -> Self::Output {
        [self[0] + other.x, self[1] + other.y]
    }
}

pub fn sum(vec2s: &[[f32; 2]]) -> [f32; 2] {
    vec2s
        .iter()
        .fold([0.0, 0.0], |v1, v2| [v1[0] + v2[0], v1[1] + v2[1]])
}

#[cfg(test)]
mod tests {
    #[test]
    fn sum() {
        let vec2_a = [1.0, 2.0];
        let vec2_b = [3.0, 4.0];

        assert_eq!(super::sum(&[vec2_a, vec2_b]), [4.0, 6.0]);
    }
}
