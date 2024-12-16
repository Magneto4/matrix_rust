use crate::vector::Vector;
use crate::Trait;
use num_traits::MulAdd;

pub fn linear_combination<T: Trait, const S: usize, const N: usize>(
    u: &[Vector<T, S>; N],
    coefs: &[T; N],
) -> Vector<T, S> {
    let mut result = Vector::from([T::default(); S]);
    for i in 0..N {
        result = u[i].mul_add(coefs[i], re);
    }
    result
}

pub fn lerp<V: Trait + MulAdd<V, f32, Output = V>>(u: V, v: V, t: f32) -> V {
    v.sub(u).mul_add(u, t)
}
