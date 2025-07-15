use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::AddAssign;
use std::ops::MulAssign;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2i
{
    pub x : i16,
    pub y : i16,
}

impl Vec2i
{
    pub fn from(tuple : (i16, i16)) -> Vec2i
    {
        return Vec2i {x : tuple.0, y : tuple.1};
    }
}

impl Vec2i 
{
    pub fn magnitude(&self) -> i16
    {
        return (self.x.pow(2) + self.y.pow(2)).isqrt();
    }
    pub fn fmagnitude(&self) -> f32
    {
        return ((self.x as f32).powf(2.0) + (self.y as f32).powf(2.0)).sqrt();
    }
    pub fn normalised(&self) -> Vec2i
    {
        let x = self.x / self.magnitude();
        let y = self.y / self.magnitude();
        return Vec2i {x, y};
    }

    pub fn add_components(&self) -> i16
    {
        return self.x + self.y;
    }
}

impl Add for Vec2i
{
    type Output = Vec2i;

    fn add(self, other: Vec2i) -> Vec2i {
        Vec2i {x: self.x + other.x, y: self.y + other.y}
    }
}

impl AddAssign for Vec2i
{
    fn add_assign(&mut self, other: Vec2i) {
        self.x += other.x;
        self.y += other.y;
    }
}


impl Sub for Vec2i
{
    type Output = Vec2i;

    fn sub(self, other: Vec2i) -> Vec2i {
        Vec2i {x: self.x - other.x, y: self.y - other.y}
    }
}

impl Mul for Vec2i
{
    type Output = Vec2i;

    fn mul(self, other: Vec2i) -> Vec2i {
        Vec2i {x: self.x * other.x, y: self.y * other.y}
    }
}

impl MulAssign for Vec2i
{
    fn mul_assign(&mut self, other: Vec2i) {
        self.x *= other.x;
        self.y *= other.y;
    }
}