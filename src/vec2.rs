use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::AddAssign;
use std::ops::MulAssign;

#[derive(Debug, Clone, Copy)]
pub struct Vec2i
{
    pub x : i16,
    pub y : i16,
}

impl Vec2i
{
    pub fn from(tuple : (i16, i16)) -> Vec2i
    {
        Vec2i {x : tuple.0, y : tuple.1}
    }
}

impl Vec2i 
{
    pub fn magnitude(&self) -> i16
    {
        (self.x.pow(2) + self.y.pow(2)).isqrt()
    }
    pub fn normalised(&self) -> Vec2i
    {
        let x = self.x / self.magnitude();
        let y = self.y / self.magnitude();
        Vec2i {x, y}
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