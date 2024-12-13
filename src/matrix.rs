use crate::Trait;
use std::fmt;

pub struct Matrix<T, const R: usize, const C: usize> {
  values:[[T; C]; R]
}

impl<T: Trait, const R: usize, const C: usize> fmt::Display for Matrix<T, R, C> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut result = String::new();
    for i in 0..R {
      result.push_str("[");
      for j in 0..C {
        result.push_str(&format!("{}", self.values[i][j]));
        if j != C - 1 {
          result.push_str(&format!(", "));
        }
      }
      result.push_str("]\n");
    }
    write!(f, "{}", result)
  }
}

impl<T: Trait, const R: usize, const C: usize> fmt::Debug for Matrix<T, R, C> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut result = String::new();
    for i in 0..R {
      result.push_str("[");
      for j in 0..C {
        result.push_str(&format!("{}", self.values[i][j]));
        if j != C - 1 {
          result.push_str(&format!(", "));
        }
      }
      result.push_str("]\n");
    }
    write!(f, "{}", result)
  }
}


impl<T: Trait, const R: usize, const C: usize> PartialEq for Matrix<T, R, C> {
  fn eq(&self, other: &Self) -> bool {
    for i in 0..R {
      for j in 0..C {
        if self.values[i][j] != other.values[i][j] {
          return false;
        }
      }
    }
    return true;
  }
}


impl<T: Trait, const R: usize, const C: usize> Matrix<T, R, C> { 
  pub fn from(v:[[T; C]; R]) -> Matrix<T, R, C> {
    return Matrix {
      values: v
    }
  }

  pub fn add(&mut self, m: &Matrix<T, R, C>) {
    for i in 0..R {
      for j in 0..C {
        self.values[i][j] += m.values[i][j];
      }
    }
  }

  pub fn sub(&mut self, m: &Matrix<T, R, C>) {
    for i in 0..R {
      for j in 0..C {
        self.values[i][j] -= m.values[i][j];
      }
    }
  }

  pub fn scl(&mut self, a: T) {
    for i in 0..R {
      for j in 0..C {
        self.values[i][j] = self.values[i][j] * a;
      }
    }
  }
}