use std::ffi::c_void;
use std::ops;

// same data-layout as engine's vec3_t,
pub struct Vector {
  x: f32,
  y: f32,
  z: f32,
}
impl Vector {
  fn new(&self) -> Self {
    Self {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    }
  }

  pub fn compare(&self, vector: Self) -> bool {
    return self.x == vector.x
      && self.y == vector.y
      && self.z == vector.z;
  }

  pub fn subtract(&mut self, vector: Self) -> &Self {
    self.x -= vector.x;
    self.y -= vector.y;
    self.z -= vector.z;

    return self;
  }

  pub fn copy_to_array(self: &Self, array: &mut [f32]) {
    array[0] = self.x;
    array[1] = self.y;
    array[2] = self.z;
  }

  pub fn length_squared(self: &Self) -> f32 {
    let Self { x, y, z } = self;

    return x * x
      + y * y
      + z * z;
  }

  pub fn length(self: &Self) -> f32 {
    return Self::length_squared(self).sqrt();
  }

  pub fn normalize(self: &Self) -> Self {
    let mut length: f32 = self.length();

    if length == 0.0 {
      return Self {
        x: 0.0,
        y: 0.0,
        z: 1.0 // ???? why z = 1?
      };
    }

    length = 1.0 / length;

    return self * length;
  }

  pub fn make2d(self: &Self) -> Self2d {
    let Self2d { x, y } = self;

    return Self2d { x: *x, y: *y };
  }

  pub fn dot_product(self: &Self, vector: &Self) -> f32 {
    return self.x * vector.x
      + self.y * vector.y
      + self.z * vector.z;
  }

  pub fn cross_product(a: &Self, b: &Self) -> Self {
    return Self {
      x: a.y * b.z - a.z * b.y,
      y: a.z * b.x - a.x * b.z,
      z: a.x * b.y - a.y * b.x
    };
  }
}
trait VectorWithConstructorArgsTrait {
  fn new(vector: Self) -> Self;
}
impl VectorWithConstructorArgsTrait for Vector {
  fn new(vector: Self) -> Self {
    Self { ..vector }
  }
}
impl ops::Add<Self> for Vector {
  type Output = Self;

  fn add(self: &Self, vector: Self) -> Self {
    let result = Self {
      x: self.x + vector.x,
      y: self.y + vector.y,
      z: self.z + vector.z
    };

    return result;
  }
}
impl ops::AddAssign<Self> for Vector {
  fn add_assign(&mut self, vector: Self) {
    self.x += vector.x;
    self.y += vector.y;
    self.z += vector.z;
  }
}
impl ops::Div<Self> for Vector {
  type Output = Self;

  fn div(&mut self, value: f32) -> &Self {
    self.x /= value;
    self.y /= value;
    self.z /= value;

    return self;
  }
}
impl ops::Mul<Self> for Vector {
  type Output = Self;

  fn mul(&mut self, value: f32) -> &Self {
    self.x *= value;
    self.y *= value;
    self.z *= value;

    return self;
  }
}


pub struct Vector2d {
  x: f32,
  y: f32,
}
impl Vector2d {
  pub fn new(x: f32, y: f32) -> Self {
    return Self { x, y };
  }

  pub fn length(self: &Self) -> f32 {
    let Self { x, y } = self;

    return (x * x + y * y).sqrt();
  }

  pub fn normalize(self: &Self) -> Self {
    let mut length: f32 = self.length();

    if length == 0.0 {
      return Self { x: 0.0, y: 0.0 }; // ???? why z = 1?
    }

    length = 1.0 / length;

    return self * length;
  }

  pub fn dot_product(self: &Self, vector: &Self) -> f32 {
    return self.x * vector.x
      + self.y * vector.y;
  }
}
impl ops::Add<Self> for Vector2d {
  type Output = Self;

  fn add(&mut self, vector: Self) -> Self {
    return Self {
      x: self.x + vector.x,
      y: self.y + vector.y
    };
  }
}
impl ops::AddAssign<Self> for Vector2d {
  fn add_assign(&mut self, vector: Self) {
    self.x += vector.x;
    self.y += vector.y;
  }
}
impl ops::Div<Self> for Vector2d {
  type Output = Self;

  fn div(&mut self, value: f32) -> &Self {
    self.x /= value;
    self.y /= value;

    return self;
  }
}
impl ops::Mul<Self> for Vector2d {
  type Output = Self;

  fn mul(&mut self, value: f32) -> &Self {
    self.x *= value;
    self.y *= value;

    return self;
  }
}

