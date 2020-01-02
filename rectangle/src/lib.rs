#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn larger_can_hold_smaller() {
    let larger = Rectangle { length: 10, width: 12 };
    let smaller = Rectangle { length: 2, width: 4 };

    assert!(larger.can_hold(&smaller));
  }
  
  #[test]
  fn smaller_cannot_hold_larger() {
    let larger = Rectangle { length: 10, width: 12 };
    let smaller = Rectangle { length: 2, width: 4 };

    assert!(!smaller.can_hold(&larger));
  }
}
