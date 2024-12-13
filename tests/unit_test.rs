extern crate math_lib;

// let e1 = Vector::from([1., 0., 0.]);
// let e2 = Vector::from([0., 1., 0.]);
// let e3 = Vector::from([0., 0., 1.]);
// let v1 = Vector::from([1., 2., 3.]);
// let v2 = Vector::from([0., 10., -100.]);
// println!("{}", linear_combination<Vector<f32>, f32>([e1, e2, e3], [10., -2.,
//   0.5]));
// // [10.]
// // [-2.]
// // [0.5]
// println!("{}", linear_combination<Vector<f32>, f32>([v1, v2], [10., -2.]));
// // [10.]
// // [0.]
// // [230.]

#[cfg(test)]
mod ex00 {
  use math_lib::matrix::Matrix;
  use math_lib::vector::Vector;
  #[test]
  fn test_add_vector() {
    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    u.add(&v);
    assert_eq!(u, Vector::from([7., 10.]));
  }

  #[test]
  fn test_sub_vector() {
    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    u.sub(&v);
    assert_eq!(u, Vector::from([-3., -4.]));
  }

  #[test]
  fn test_mul_vector() {
    let mut u = Vector::from([2., 3.]);
    u.scl(2.);
    assert_eq!(u, Vector::from([4., 6.]));
  }

  #[test]
  fn test_add_matrix() {
    let mut u = Matrix::from([
    [1., 2.],
    [3., 4.]
    ]);
    let v = Matrix::from([
      [7., 4.],
      [-2., 2.]
    ]);
    u.add(&v);
    assert_eq!(u, Matrix::from([
      [8., 6.],
      [1., 6.]
    ]));
  }

  #[test]
  fn test_sub_matrix() {
    let mut u = Matrix::from([
      [1., 2.],
      [3., 4.]
    ]);
    let v = Matrix::from([
      [7., 4.],
      [-2., 2.] ]);
    u.sub(&v);
    assert_eq!(u, Matrix::from([
      [-6., -2.],
      [5., 2.]
    ]));
  }

  #[test]
  fn test_mul_matrix() {
    let mut u = Matrix::from([
      [1., 2.],
      [3., 4.]
    ]);
    u.scl(2.);
    assert_eq!(u, Matrix::from([
      [2., 4.],
      [6., 8.]
    ]));
  }
}

#[cfg(test)]
mod ex01 {
  #[test]
  fn test_linear_combination() {
    use math_lib::vector::Vector;
    use math_lib::utils::linear_combination;
    let e1 = Vector::from([1., 0., 0.]);
    let e2 = Vector::from([0., 1., 0.]);
    let e3 = Vector::from([0., 0., 1.]);
    let v1 = Vector::from([1., 2., 3.]);
    let v2 = Vector::from([0., 10., -100.]);
    assert_eq!(linear_combination(&[e1, e2, e3], &[10., -2., 0.5]), Vector::from([10., -2., 0.5]));
    assert_eq!(e1, Vector::from([1., 0., 0.]));
    assert_eq!(e2, Vector::from([0., 1., 0.]));
    assert_eq!(linear_combination(&[v1, v2], &[10., -2.]), Vector::from([10., 0., 230.]));
  }
}
