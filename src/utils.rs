use crate::vector::Vector;
use crate::Trait;

pub fn linear_combination<T: Trait, const S: usize, const N: usize>(u: &[Vector<T, S>; N], coefs: &[T; N]) -> Vector<T, S> {
  let mut result = Vector::from([T::default(); S]);
  for i in 0..N {
    let mut cpy = u[i];
    cpy.scl(coefs[i]);
    result.add(&cpy);
  }
  result
}