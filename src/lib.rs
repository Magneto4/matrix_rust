use std::fmt::Display;
use std::ops::Add;
use std::ops::{AddAssign, Mul, SubAssign};

pub trait Trait:
    Display + Default + std::cmp::PartialEq + Add<Output = Self> + Mul<Output = Self> + AddAssign + SubAssign + Copy
{
}
impl<T: Display + Default + std::cmp::PartialEq + Add<Output = T> + Mul<Output = T> + AddAssign + SubAssign + Copy> Trait
    for T
{
}

pub mod matrix;
pub mod vector;
pub mod utils;