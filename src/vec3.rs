use std::ops::{Sub, Index, Div, Neg};
use std::fmt::{Formatter, Display};

/// 3 dimensional f32 Vector, with some convenience math functions
#[derive(Copy, Clone, Default)]
pub struct Vec3(
    pub f32,
    pub f32,
    pub f32);

impl Vec3 {
    pub const fn x(&self) -> f32 {
        self.0
    }
    pub const fn y(&self) -> f32 {
        self.1
    }
    pub const fn z(&self) -> f32 {
        self.2
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3(
            x,
            y,
            z
        )
    }
    pub fn newi(x: i32, y: i32, z: i32) -> Vec3 {
        Vec3(
            x as f32,
            y as f32,
            z as f32
        )
    }
}
impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}
impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
        )
    }
}
impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
        )
    }
}
impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(
            self.0 * rhs.0,
            self.1 * rhs.1,
            self.2 * rhs.2,
        )
    }
}
impl std::ops::Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3(
            self.0 / rhs.0,
            self.1 / rhs.1,
            self.2 / rhs.2,
        )
    }
}
impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3){
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl std::ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3){
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}
impl std::ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: Vec3){
        self.0 /= rhs.0;
        self.1 /= rhs.1;
        self.2 /= rhs.2;
    }
}


impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3 {
        Vec3(
            self.0 * rhs,
            self.1 * rhs,
            self.2 * rhs,
        )
    }
}
impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3(
            self.0 / rhs,
            self.1 / rhs,
            self.2 / rhs,
        )
    }
}
impl std::ops::Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3(
            self.0 / rhs,
            self.1 / rhs,
            self.2 / rhs,
        )
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs*self
    }
}

impl Vec3 {
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.0*self.0 + self.1*self.1 + self.2*self.2
    }

    pub fn dot(&self, v: &Vec3) -> f32 {
        self.0 * v.0
        + self.1 * v.1
        + self.2 * v.2
    }

    pub fn cross(self, v: &Vec3) -> Vec3 {
        Vec3(
            self.1 * v.2 - self.2 * v.1,
            self.2 * v.0 - self.0 * v.2,
            self.0 * v.1 - self.1 * v.0
        )
    }

    pub fn unit_vector(&self) -> Vec3 {
        return self / self.length();
    }
}

///these type aliases are a bit silly since you cant use type alias in construction, e.g Color(0.0,0.0,0.0) doesn't work
pub type Point = Vec3;
pub type Color = Vec3;
pub type Pixel = Color;

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {} {}",(self.x()*255.999) as u8, (self.y()*255.999) as u8, (self.z()*255.999) as u8))
    }
}
