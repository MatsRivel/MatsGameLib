#[macro_export]
macro_rules! impl_vec2_add_assign {
    ($Lhs:ty, $Rhs:ty) => {
        impl AddAssign<$Rhs> for $Lhs
        where
            $Lhs: Deref<Target = Vec2>,
            $Rhs: Deref<Target = Vec2>,
        {
            fn add_assign(&mut self, rhs: $Rhs) {
                *self = *self + *rhs;
            }
        }
    };
}
#[macro_export]
macro_rules! impl_vec2_add {
    ($Lhs:ty, $Rhs:ty, $Out:ty) => {
        impl Add<$Rhs> for $Lhs
        where
            $Lhs: Deref<Target = Vec2>,
            $Rhs: Deref<Target = Vec2>,
        {
            type Output = $Out;
            fn add(self, rhs: $Rhs) -> Self::Output {
                <$Out>::from(*self + *rhs)
            }
        }
    };
}


#[macro_export]
macro_rules! impl_f32_add_assign {
    ($Lhs:ty, $Rhs:ty) => {
        impl AddAssign<$Rhs> for $Lhs
        where
            $Lhs: Deref<Target = f32>,
            $Rhs: Deref<Target = f32>,
        {
            fn add_assign(&mut self, rhs: $Rhs) {
                *self = *self + rhs;
            }
        }
    };
}
#[macro_export]
macro_rules! impl_f32_add {
    ($Lhs:ty, $Rhs:ty, $Out:ty) => {
        impl Add<$Rhs> for $Lhs
        where
            $Lhs: Deref<Target = f32>,
            $Rhs: Deref<Target = f32>,
        {
            type Output = $Out;
            fn add(self, rhs: $Rhs) -> Self::Output {
                <$Out>::from(*self + *rhs)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_vec2_f32_mul_assign {
    ($Lhs:ty, $Rhs:ty) => {
        impl MulAssign<$Rhs> for $Lhs
        where
            $Lhs: Deref<Target = Vec2>,
            $Rhs: Deref<Target = f32>,
        {
            fn mul_assign(&mut self, rhs: $Rhs) {
                *self = *self * rhs;
            }
        }
    };
}
#[macro_export]
macro_rules! impl_vec2_f32_mul {
    ($Lhs:ty, $Rhs:ty, $Out:ty) => {
        impl Mul<$Rhs> for $Lhs
        where
            $Lhs: Deref<Target = Vec2>,
            $Rhs: Deref<Target = f32>,
        {
            type Output = $Out;
            fn mul(self, rhs: $Rhs) -> Self::Output {
                <$Out>::from(*self * *rhs)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_f32_f32_mul_assign {
    ($Lhs:ty, $Rhs:ty) => {
        impl MulAssign<$Rhs> for $Lhs
        where
            $Lhs: Deref<Target = f32>,
            $Rhs: Deref<Target = f32>,
        {
            fn mul_assign(&mut self, rhs: $Rhs) {
                *self = *self * rhs;
            }
        }
    };
}
#[macro_export]
macro_rules! impl_f32_f32_mult {
    ($Lhs:ty, $Rhs:ty, $Out:ty) => {
        impl Mul<$Rhs> for $Lhs
        where
            $Lhs: Deref<Target = f32>,
            $Rhs: Deref<Target = f32>,
        {
            type Output = $Out;
            fn mul(self, rhs: $Rhs) -> Self::Output {
                <$Out>::from(*self * *rhs)
            }
        }
    };
}