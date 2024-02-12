use std::{ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign}, str::FromStr};

use crate::tools::maths::ops::dot::DotProduct;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vector3<T> {
    x: T,
    y: T,
    z: T,
}

pub type Vector3f = Vector3<f32>;
pub type Vector3d = Vector3<f64>;
pub type Location = Vector3<i64>;

impl <T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn y(&self) -> &T {
        &self.y
    }

    pub fn z(&self) -> &T {
        &self.z
    }
}

impl <T: FromStr + Default + Copy> FromStr for Vector3<T> {
    type Err = T::Err;
    /* TODO remake better */
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(' ');
        let mut vec: Vec<T> = Vec::new();
        for elem in split.into_iter() {
            vec.push(elem.parse()?);
        }
        
        let x = if vec.len() < 1 { T::default() } else { vec.clone()[0] };
        let y = if vec.len() < 2 { T::default() } else { vec.clone()[1] };
        let z = if vec.len() < 3 { T::default() } else { vec.clone()[2] };

        Ok(Vector3 { x, y, z })
    }
}

impl <T: Neg<Output = T>> Neg for Vector3<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl <T: AddAssign> AddAssign for Vector3<T> {

    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl <T: Add<Output = T>> Add for Vector3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl <T: Sub<Output = T>> Sub for Vector3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z 
        }
    }
}

impl <T: SubAssign + Copy> SubAssign for Vector3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.x;
        self.z -= rhs.x;
    }
}

impl <T: Add<Output = T> + Mul<Output = T> + Copy> DotProduct for Vector3<T> {
    type Output = T;

    fn dot(&self, rhs: Self) -> Self::Output {
        self.x * rhs.x +
        self.y * rhs.y +
        self.z * rhs.z
    }
}
impl <T: Mul<Output = T> + Copy> Mul<T> for Vector3<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}


impl <T: Mul<Output = T> + Copy> Mul for Vector3<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl <T: MulAssign + Copy> MulAssign<T> for Vector3<T> {

    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}


impl <T: MulAssign> MulAssign for Vector3<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y; 
        self.z *= rhs.z;
    }
}

impl <T: Div<Output = T> + Copy> Div<T> for Vector3<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl <T: DivAssign + Copy> DivAssign<T> for Vector3<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Vector3<f32> {
    pub fn norm(&self) -> f32 {
        self.dot(self.clone()).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let n = self.norm();
        if n == 0f32 {
            return Self { x: 0f32, y: 0f32, z: 0f32 }
        }
        *self / n
    }

    pub fn angle_between(&self, rhs: Self) -> f32 {
        let dot_product = self.dot(rhs);
        let length1 = self.norm();
        let length2 = rhs.norm();

        if length1 == 0f32 || length2 == 0f32 {
            return std::f32::NAN
        }

        let cos_theta = dot_product / (length1 * length2);
        let angle = cos_theta.acos();

        angle
    }
    
}

impl Vector3d {
    pub fn norm(&self) -> f64 {
        self.dot(self.clone()).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let n = self.norm();
        if n == 0f64 {
            return Self { x: 0f64, y: 0f64, z: 0f64 }
        }
        *self / n
    }
    
    pub fn angle_between(&self, rhs: Self) -> f64 {
        let dot_product = self.dot(rhs);
        let length1 = self.norm();
        let length2 = rhs.norm();

        if length1 == 0f64 || length2 == 0f64 {
            return std::f64::NAN
        }

        let cos_theta = dot_product / (length1 * length2);
        let angle = cos_theta.clamp(-1f64, 1f64).acos();
        angle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalization() {
        let a = Vector3d::new(10f64, 0f64, 0f64);
        assert_eq!(a.normalize(), Vector3d::new(1f64, 0f64, 0f64));
    }

    #[test]
    fn test_angle() {
        let a = Vector3d::new(3.8f64, 2.7f64, 9.12f64);
        debug_assert_ne!(a.norm(), 0f64);
        let b = -a;
        debug_assert_ne!(b.norm(), 0f64);
        debug_assert_eq!(b.angle_between(a), std::f64::consts::PI)
    }
}