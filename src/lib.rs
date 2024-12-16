use num_traits::ops::mul_add::MulAdd;
use std::fmt::Display;
use std::ops::Add;
use std::ops::{AddAssign, Mul, Sub, SubAssign};

pub trait Trait:
    Display
    + Default
    + std::cmp::PartialEq
    + Add<Output = Self>
    + Mul<Output = Self>
    + AddAssign
    + SubAssign
    + Copy
    + MulAdd<Output = Self>
    + Sub<Output = Self>
{
}
impl<
        T: Display
            + Default
            + std::cmp::PartialEq
            + Add<Output = T>
            + Mul<Output = T>
            + AddAssign
            + SubAssign
            + Copy
            + MulAdd<Output = Self>
            + Sub<Output = Self>,
    > Trait for T
{
}

pub mod matrix;
pub mod utils;
pub mod vector;
