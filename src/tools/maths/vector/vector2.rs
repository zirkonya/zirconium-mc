use std::{ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign}, str::FromStr};

use crate::tools::maths::ops::dot::DotProduct;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector2<T> {
    x: T,
    y: T
}

pub type Vector2d = Vector2<f64>;
pub type Vector2f = Vector2<f32>;

impl <T> Vector2<T> {
    #[inline(always)]
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    pub fn x(&self) -> &T {
        &self.x
    }

    #[inline(always)]
    pub fn y(&self) -> &T {
        &self.y
    }
}

impl <T: FromStr + Default + Copy> FromStr for Vector2<T> {
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

        Ok(Vector2 { x, y })
    }
}

impl <T: Neg<Output = T>> Neg for Vector2<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y }
    }
}

impl <T: AddAssign> AddAssign for Vector2<T> {

    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl <T: Add<Output = T>> Add for Vector2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl <T: Sub<Output = T>> Sub for Vector2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl <T: SubAssign + Copy> SubAssign for Vector2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.x;
    }
}

impl <T: Add<Output = T> + Mul<Output = T> + Copy> DotProduct for Vector2<T> {
    type Output = T;

    fn dot(&self, rhs: Self) -> Self::Output {
        self.x * rhs.x +
        self.y * rhs.y
    }
}
impl <T: Mul<Output = T> + Copy> Mul<T> for Vector2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}


impl <T: Mul<Output = T> + Copy> Mul for Vector2<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl <T: MulAssign + Copy> MulAssign<T> for Vector2<T> {

    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}


impl <T: MulAssign> MulAssign for Vector2<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl <T: Div<Output = T> + Copy> Div<T> for Vector2<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl <T: DivAssign + Copy> DivAssign<T> for Vector2<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Vector2f {
    pub fn norm(&self) -> f32 {
        self.dot(self.clone()).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let n = self.norm();
        if n == 0f32 {
            return Self { x: 0f32, y: 0f32 }
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

impl Vector2d {
    pub fn norm(&self) -> f64 {
        self.dot(self.clone()).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let n = self.norm();
        if n == 0f64 {
            return Self { x: 0f64, y: 0f64 }
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
        let angle = cos_theta.acos();
        angle
    }
}

// TODO : Test for vecor2 & 3
#[cfg(test)]
mod tests {
    use std::f64::consts;

    use crate::tools::maths::ops::dot::DotProduct;

    use super::Vector2;

    #[test]
    fn test_adding_vector() {
        let a = Vector2::new(1.0_f64, 1.0_f64);
        let b = Vector2::new(2.0_f64, -1.0_f64);
        let waited = Vector2::new(3.0_f64, 0.0_f64);
        assert_eq!(a+b, waited);
    }

    #[test]
    fn test_substracting_vector() {
        let a = Vector2::new(1.0_f64, 1.0_f64);
        let b = Vector2::new(2.0_f64, -1.0_f64);
        let waited = Vector2::new(-1.0_f64, 2.0_f64);
        assert_eq!(a-b, waited);
    }
    
    #[test]
    fn test_multiplying_vector() {
        let a = Vector2::new(1.0_f64, 1.0_f64);
        let b = Vector2::new(2.0_f64, -1.0_f64);
        let waited = Vector2::new(2.0_f64, -1.0_f64);
        assert_eq!(a*b, waited);
    }

    #[test]
    fn test_dividing_vector() {
        let a = Vector2::new(5.0_f64, 4.0_f64);
        let waited = Vector2::new(2.5_f64, 2.0_f64);
        assert_eq!(a/2.0_f64, waited);
    }

    #[test]
    fn test_dot_vector() {
        let a = Vector2::new(1.0_f64, 1.0_f64);
        let b = Vector2::new(2.0_f64, -1.0_f64);
        let waited = 1_f64;
        assert_eq!(a.dot(b), waited);
    }

    #[test]
    fn test_normalize() {
        let a = Vector2::new(3.0_f64, -6.0_f64);
        let normalized = a.normalize();
        assert_eq!(normalized.norm().round(), 1.0_f64)
    }

    #[test]
    fn test_norm() {
        let a = Vector2::new(10.0_f32, 0.0);
        let norm = a.norm();
        assert_eq!(norm, 10.0)
    }

    #[test]
    fn test_angle() {
        let a = Vector2::new(1.0_f64, 1.0_f64);
        let b = Vector2::new(-1.0_f64, -1.0_f64);
        assert_eq!((b.angle_between(a) * 1_000_000_f64).round(), (consts::PI * 1_000_000_f64).round())
    }
}