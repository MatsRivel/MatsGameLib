use super::*;

macro_rules! vec2_wrapper {
    ($t:ty) => {
        impl Add<Vec2> for &t{
            type Output = Self;
            fn add(self, rhs: Vec2) -> Self::Output {
                Self(self.0 + rhs)
            }
        }
        impl AddAssign<Vec2> for &t{
            fn add_assign(&mut self, rhs: Vec2) {
                self.0 += rhs
            }
        }
        impl Sub<Vec2> for &t{
            type Output = Self;
            fn sub(self, rhs: Vec2) -> Self::Output {
                Self(self.0 - rhs)
            }
        }
        impl SubAssign<Vec2> for &t{
            fn sub_assign(&mut self, rhs: Vec2) {
                self.0 -= rhs
            }
        }
        impl Mul<Vec2> for &t{
            type Output = Self;
            fn mul(self, rhs: Vec2) -> Self::Output {
                Self(self.0 * rhs)
            }
        }
        impl MulAssign<Vec2> for &t{
            fn mul_assign(&mut self, rhs: Vec2){
                self.0 *= rhs
            }
        }
        impl Mul<f32> for &t{
            type Output = Self;
            fn mul(self, rhs: f32) -> Self::Output {
                Self(self.0 * rhs)
            }
        }
        impl MulAssign<f32> for &t{
            fn mul_assign(&mut self, rhs: f32){
                self.0 *= rhs
            }
        }
    }
}
