use std::ops::{Mul, Add, Sub, BitAnd, BitXor, Div};


#[derive(Debug, Clone, Copy)]
pub struct Point<T : Copy> {
    pub x : T,
    pub y : T,
    pub z : T,
}

impl Point<f64> {
    pub fn random(min : f64, max : f64) -> Point<f64> {
        Point {
            x: (rand::random::<f64>()*(max-min))+min,
            y: (rand::random::<f64>()*(max-min))+min,
            z: (rand::random::<f64>()*(max-min))+min,
        }
    }
    pub fn random_in_sphere(radius : f64) -> Point<f64> {
        loop {
            let p = Point::random(-radius, radius);
            if p.norm_squared() >= radius {
                continue;
            }
            return p;
        }
    }
    pub fn random_unit_in_sphere(radius : f64) -> Point<f64> {
        Point::random_in_sphere(radius).unit()
    }
    pub fn unit(&self) -> Point<f64> {
        (*self)/(self.norm())
    }

    pub fn norm_squared(&self) -> f64 {
        (*self)&(*self)
    }

    pub fn norm(&self) -> f64 {
        ((*self)&(*self)).sqrt()
    }

    pub fn near_zero(&self) -> bool {
        let s : f64 = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }
}

impl<T: Mul<Output = T> + Copy> Mul<Self> for Point<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Point{
            x:self.x * rhs.x,
            y:self.y * rhs.y,
            z:self.z * rhs.z,
        }
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Point<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Point{
            x:self.x * rhs,
            y:self.y * rhs,
            z:self.z * rhs,
        }
    }

}

impl<T: Add<Output = T> + Copy> Add for Point<T>{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point{
            x:self.x + rhs.x,
            y:self.y + rhs.y,
            z:self.z + rhs.z,
        }
    }
}

impl<T: Sub<Output = T> + Copy> Sub for Point<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point{
            x:self.x - rhs.x,
            y:self.y - rhs.y,
            z:self.z - rhs.z,
        }
    }
}

impl<T: Mul<Output=T> + Add<Output=T> + Copy> BitAnd for Point<T> {
    type Output = T;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.x*rhs.x + self.y*rhs.y + self.z*rhs.z
    }
}

impl<T: Mul<Output = T> + Sub<Output = T> + Copy> BitXor for Point<T> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Point {
            x:self.y*rhs.z - self.z*rhs.y,
            y:self.z*rhs.x - self.x*rhs.z,
            z:self.x*rhs.y - self.y*rhs.x,
        }
    }
}

impl<T: Div<Output = T> + Mul<Output = T> + Copy> Div<T> for Point<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Point {
            x:self.x/rhs,
            y:self.y/rhs,
            z:self.z/rhs,
        }
    }

}

impl Mul<u32> for Point<f64> {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        Point {
            x:self.x*(rhs as f64),
            y:self.y*(rhs as f64),
            z:self.z*(rhs as f64),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_random() {
        
    }
}
