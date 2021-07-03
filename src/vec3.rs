use std::ops::{Sub, Index, Div};
use std::fmt::{Formatter, Display};

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3{
            x,
            y,
            z
        }
    }
    pub fn newi(x: i32, y: i32, z: i32) -> Vec3 {
        Vec3{
            x: x as f32,
            y: y as f32,
            z: z as f32
        }
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3{
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
impl std::ops::Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3{
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}
impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3){
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3){
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}
impl std::ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: Vec3){
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}


impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3 {
        Vec3{
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3{
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl std::ops::Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3{
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs*self
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {} {}", self.x, self.y, self.z))
    }
}


impl Vec3 {
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn dot(&self, v: &Vec3) -> f32 {
        self.x * v.x
        + self.y * v.y
        + self.z * v.z
    }

    pub fn cross(self, v: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        return self / self.length();
    }
}


pub type Point = Vec3;

#[derive(Copy, Clone)]
pub struct Color(pub Vec3);

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {} {}",(self.0.x*255.999) as u8, (self.0.y*255.999) as u8, (self.0.z*255.999) as u8))
    }
}
impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color(Vec3{
            x: r,
            y: g,
            z: b
        })
    }

    pub fn newi(r: i32, g: i32, b: i32) -> Color {
        Color::new(
            r as f32,
            g as f32,
            b as f32
        )
    }
}
