use crate::Trait;
use std::fmt;

pub struct Vector<T, const S: usize> {
  values:[T; S]
}

impl<T: Trait, const S: usize> fmt::Display for Vector<T, S> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut result = String::new();
    result.push_str("[");
    for i in 0..S {
      result.push_str(&format!("{}", self.values[i]));
      if i != S - 1 {
        result.push_str(&format!(", "));
      }
    }
    result.push_str("]");
    write!(f, "{}", result)
  }
}

impl<T: Trait, const S: usize> fmt::Debug for Vector<T, S> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut result = String::new();
    result.push_str("[");
    for i in 0..S {
      result.push_str(&format!("{}", self.values[i]));
      if i != S - 1 {
        result.push_str(&format!(", "));
      }
    }
    result.push_str("]");
    write!(f, "{}", result)
  }
}

impl<T: Trait, const S: usize> PartialEq for Vector<T, S> {
  fn eq(&self, other: &Self) -> bool {
    for i in 0..S {
      if self.values[i] != other.values[i] {
        return false;
      }
    }
    return true;
  }
}

impl<T: Trait, const S: usize> Copy for Vector<T, S> {}

impl<T: Trait, const S: usize> Clone for Vector<T, S> {
  fn clone(&self) -> Vector<T, S> {
    let v = Vector::from(self.values);
    return v;
  }
}

impl<T: Trait, const S: usize> Vector<T, S> { 
  pub fn from(v: [T; S]) -> Vector<T, S> {
    return Vector {
      values: v
    };
  }

  pub fn add(&mut self, v: &Vector<T, S>) {
    for i in 0..S {
      self.values[i] += v.values[i];
    }
  }

  pub fn sub(&mut self, v: &Vector<T, S>) {
    for i in 0..S {
      self.values[i] -= v.values[i];
    }
  }

  pub fn scl(&mut self, a: T) {
    for i in 0..S {
      self.values[i] = self.values[i] * a;
    }
  }
}