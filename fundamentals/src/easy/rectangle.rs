/*
  Problem 12: Rectangle Area (Structs)

  Define a struct Rectangle with width and height fields (both f64).
  Implement a method area() that returns the area, and a method is_square()
  that returns true if width equals height.

  Run the tests for this problem with:
    cargo test --test rectangle_test
*/

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn area(&self) -> f64 {
        let area: f64 = self.width * self.height;
        return area;
    }

    pub fn is_square(&self) -> bool {
        if (self.height - self.width).abs() < 1e-9 {
            return  true;
        } else {
            return false;
        }
    }
}
