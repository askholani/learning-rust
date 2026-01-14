// Problem 3: Tuple Matrix Operations (Medium-Hard)
/**
 * Description:
 * Given two 2D arrays (3x3 matrices) represented as tuples of tuples, perform element-wise operations:
 *
 * Add the two matrices
 * Subtract the second from the first
 * Multiply each element by a scalar

fn matrix_operations(
    a: ((i32, i32, i32), (i32, i32, i32), (i32, i32, i32)),
    b: ((i32, i32, i32), (i32, i32, i32), (i32, i32, i32)),
    scalar: i32
) -> (
    ((i32, i32, i32), (i32, i32, i32), (i32, i32, i32)),
    ((i32, i32, i32), (i32, i32, i32), (i32, i32, i32)),
    ((i32, i32, i32), (i32, i32, i32), (i32, i32, i32)),
)

    Input:
a: First 3x3 matrix as tuple of tuples
b: Second 3x3 matrix as tuple of tuples
scalar: Integer to multiply with

Output:
Return a tuple of three matrices:
Matrix addition: a + b (element-wise)
Matrix subtraction: a - b (element-wise)
Scalar multiplication: a × scalar
 */

pub fn matrix_operations(
  a: ((i32, i32, i32), (i32, i32, i32), (i32, i32, i32)),
  b: ((i32, i32, i32), (i32, i32, i32), (i32, i32, i32)),
  scalar: i32,
) -> (
  ((i32, i32, i32), (i32, i32, i32), (i32, i32, i32)),
  ((i32, i32, i32), (i32, i32, i32), (i32, i32, i32)),
  ((i32, i32, i32), (i32, i32, i32), (i32, i32, i32)),
) {
  let addition = (
    (a.0 .0 + b.0 .0, a.0 .1 + b.0 .1, a.0 .2 + b.0 .2),
    (a.1 .0 + b.1 .0, a.1 .1 + b.1 .1, a.1 .2 + b.1 .2),
    (a.2 .0 + b.2 .0, a.2 .1 + b.2 .1, a.2 .2 + b.2 .2),
  );
  let subtraction = (
    (a.0 .0 - b.0 .0, a.0 .1 - b.0 .1, a.0 .2 - b.0 .2),
    (a.1 .0 - b.1 .0, a.1 .1 - b.1 .1, a.1 .2 - b.1 .2),
    (a.2 .0 - b.2 .0, a.2 .1 - b.2 .1, a.2 .2 - b.2 .2),
  );
  let scalar_calculated = (
    (a.0 .0 * scalar, a.0 .1 * scalar, a.0 .2 * scalar),
    (a.1 .0 * scalar, a.1 .1 * scalar, a.1 .2 * scalar),
    (a.2 .0 * scalar, a.2 .1 * scalar, a.2 .2 * scalar),
  );

  // println!("a {:?}", a);
  // println!("b {:?}", b);
  // println!("a.0 {:?}",a.0);
  // println!("a.0.0 {:?}",a.0.0);
  // println!("a.0.1 {:?}",a.0.1);
  // println!("scalar {}", scalar);
  (addition, subtraction, scalar_calculated)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_matrix_operations() {
    let a = ((1, 2, 3), (4, 5, 6), (7, 8, 9));
    let b = ((9, 8, 7), (6, 5, 4), (3, 2, 1));
    let c = 4;

    let result = matrix_operations(a, b, c);
    println!("result {:?}", result)
  }
}
