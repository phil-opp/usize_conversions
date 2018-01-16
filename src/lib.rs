//! Conversion traits for conversions between `usize` and fixed sized integers.
//!
//! This crate provides two traits, `UsizeFrom` and `UsizeInto`, that can be used
//! similar to the `From` and `Into` traits of the standard library.
//!
//! Warning: The traits are conditionally implemented based on the target pointer width,
//! so they can make your crate less portable.

#![warn(missing_docs)]

#![no_std]

/// Additional `from` conversions for `usize`.
pub trait UsizeFrom<T>: Sized {
    /// Performs the conversion.
    fn from(value: T) -> Self;
}

/// Additional `into` conversions for `usize`.
pub trait UsizeInto<T>: Sized {
    /// Performs the conversion.
    fn into(self) -> T;
}

impl<T, U> UsizeInto<T> for U
    where T: UsizeFrom<U>
{
    fn into(self) -> T {
        T::from(self)
    }
}

macro_rules! implement {
    ($x:ty, $y:ty) => {
        impl UsizeFrom<$x> for $y {
            fn from(value: $x) -> Self {
                value as $y
            }
        }
    }
}

#[cfg(target_pointer_width = "128")] implement!(usize, u128);
#[cfg(target_pointer_width = "128")] implement!(u128, usize);
#[cfg(target_pointer_width = "128")] implement!(u64, usize);
#[cfg(target_pointer_width = "128")] implement!(u32, usize);
#[cfg(target_pointer_width = "128")] implement!(u16, usize);
#[cfg(target_pointer_width = "128")] implement!(u8, usize);

#[cfg(target_pointer_width = "64")] implement!(usize, u64);
#[cfg(target_pointer_width = "64")] implement!(u64, usize);
#[cfg(target_pointer_width = "64")] implement!(u32, usize);
#[cfg(target_pointer_width = "64")] implement!(u16, usize);
#[cfg(target_pointer_width = "64")] implement!(u8, usize);

#[cfg(target_pointer_width = "32")] implement!(usize, u32);
#[cfg(target_pointer_width = "32")] implement!(u32, usize);
#[cfg(target_pointer_width = "32")] implement!(u16, usize);
#[cfg(target_pointer_width = "32")] implement!(u8, usize);

#[cfg(target_pointer_width = "16")] implement!(usize, u16);
#[cfg(target_pointer_width = "16")] implement!(u16, usize);
#[cfg(target_pointer_width = "16")] implement!(u8, usize);

#[cfg(target_pointer_width = "8")] implement!(usize, u8);
#[cfg(target_pointer_width = "8")] implement!(u8, usize);
