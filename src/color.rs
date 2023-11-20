use std::ops::{Mul, Add};

pub struct Color<T : Copy> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl Color<f64> {
    pub fn write(self, sample_number : u32){
        println!("{} {} {}", ((self.r/sample_number as f64)*255.0) as u16, ((self.g/sample_number as f64)*255.0) as u16, ((self.b/sample_number as f64)*255.0) as u16);
    }
}

impl<T : Copy + Mul<Output = T> > Mul<T> for Color<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Color {
            r: self.r*rhs,
            g: self.g*rhs,
            b: self.b*rhs,
        }
    }
}

impl Mul<Color<f64>> for f64 {
    type Output = Color<f64>;

    fn mul(self, rhs: Color<f64>) -> Self::Output {
        Color {
            r: self*rhs.r,
            g: self*rhs.g,
            b: self*rhs.b,
        }
    }
}

impl Add<Color<f64>> for Color<f64> {
    type Output = Color<f64>;

    fn add(self, rhs: Color<f64>) -> Self::Output {
        Color {
            r: self.r+rhs.r,
            g: self.g+rhs.g,
            b: self.b+rhs.b,
        }
    }

}
