use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Vec2Ops)]
pub fn derive_vec2_ops(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    // This assumes single unnamed field like: struct Foo(Vec2);
    let expanded = quote! {

        impl Deref for #name {
            type Target = Vec2;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        // impl DerefMut for #name {
        //     type Target = Vec2;
        //     fn deref_mut(&mut self) -> &mut Self::Target {
        //         &mut self.0
        //     }
        // }
        impl From<Vec2> for #name{
            fn from(value: Vec2) -> Self {
                Self(value)
            }
        }

        impl Add<Vec2> for #name{
            type Output = Self;
            fn add(self, rhs: Vec2) -> Self::Output {
                Self(self.0 + rhs)
            }
        }

        impl AddAssign<Vec2> for #name{
            fn add_assign(&mut self, rhs: Vec2) {
                self.0 += rhs
            }
        }

        impl Sub<Vec2> for #name{
            type Output = Self;
            fn sub(self, rhs: Vec2) -> Self::Output {
                Self(self.0 - rhs)
            }
        }

        impl SubAssign<Vec2> for #name{
            fn sub_assign(&mut self, rhs: Vec2) {
                self.0 -= rhs
            }
        }

        impl Mul<Vec2> for #name{
            type Output = Self;
            fn mul(self, rhs: Vec2) -> Self::Output {
                Self(self.0 * rhs)
            }
        }

        impl MulAssign<Vec2> for #name{
            fn mul_assign(&mut self, rhs: Vec2){
                self.0 *= rhs
            }
        }
        impl Mul<f32> for #name{
            type Output = Self;
            fn mul(self, rhs: f32) -> Self::Output {
                Self(self.0 * rhs)
            }
        }

        impl MulAssign<f32> for #name{
            fn mul_assign(&mut self, rhs: f32){
                self.0 *= rhs
            }
        }


    };

    TokenStream::from(expanded)
}


#[proc_macro_derive(f32Deref)]
pub fn derive_f32_deref(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    // This assumes single unnamed field like: struct Foo(f32);
    let expanded = quote! {

        impl Deref for #name {
            type Target = f32;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
    TokenStream::from(expanded)
}
#[proc_macro_derive(f32Ops)]
pub fn derive_f32_ops(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    // This assumes single unnamed field like: struct Foo(f32);
    let expanded = quote! {

        impl Deref for #name {
            type Target = f32;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        // impl DerefMut for #name {
        //     type Target = f32;
        //     fn deref_mut(&mut self) -> &mut Self::Target {
        //         &mut self.0
        //     }
        // }

        impl From<f32> for #name{
            fn from(value: f32) -> Self {
                Self(value)
            }
        }
        impl Add<f32> for #name{
            type Output = Self;
            fn add(self, rhs: f32) -> Self::Output {
                Self(self.0 + rhs)
            }
        }
        impl AddAssign<f32> for #name{
            fn add_assign(&mut self, rhs: f32) {
                self.0 += rhs
            }
        }
        impl Sub<f32> for #name{
            type Output = Self;
            fn sub(self, rhs: f32) -> Self::Output {
                Self(self.0 - rhs)
            }
        }
        impl SubAssign<f32> for #name{
            fn sub_assign(&mut self, rhs: f32) {
                self.0 -= rhs
            }
        }
        impl Mul<f32> for #name{
            type Output = Self;
            fn mul(self, rhs: f32) -> Self::Output {
                Self(self.0 * rhs)
            }
        }
        impl MulAssign<f32> for #name{
            fn mul_assign(&mut self, rhs: f32){
                self.0 *= rhs
            }
        }


    };

    TokenStream::from(expanded)
}
