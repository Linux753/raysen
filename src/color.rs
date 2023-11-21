use std::ops::{Mul, Add};

pub struct Color<T : Copy> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl Color<f64> {
    pub fn write(self, sample_number : u32){
        let scale : f64 = 1./sample_number as f64;
        //Applying the Gamma correction
        let r = (scale*self.r).sqrt();
        let g = (scale*self.g).sqrt();
        let b = (scale*self.b).sqrt();
        
        //Printing
        println!("{} {} {}", (r*255.0) as u16, (g*255.0) as u16, (b*255.0) as u16);
    }

    pub fn random() -> Color<f64> {
        Color { r: rand::random(), g: rand::random(), b: rand::random() }
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

impl<T : Mul<Output = T> + Copy> Mul<Self> for Color<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r*rhs.r,
            g: self.g*rhs.g,
            b: self.b*rhs.b,
        }
    }
}

impl<T : Mul<Output = T> + Copy> Mul<&Self> for Color<T> {
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self::Output {
        Color {
            r: self.r*rhs.r,
            g: self.g*rhs.g,
            b: self.b*rhs.b,
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

impl<T : Add<Output = T> + Copy> Add<Color<T>> for Color<T> {
    type Output = Color<T>;

    fn add(self, rhs: Color<T>) -> Self::Output {
        Color {
            r: self.r+rhs.r,
            g: self.g+rhs.g,
            b: self.b+rhs.b,
        }
    }

}
