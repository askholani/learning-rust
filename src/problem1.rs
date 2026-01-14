// Problem 1: Array Statistics (Easy)
/**
 * Description:
Given a fixed-size array of integers, calculate:

1. The sum of all elements

2. The maximum value

3. The minimum value

4. The average (as floating-point)

fn array_stats(arr: [i32; 5]) -> (i32, i32, i32, f64)

 */

pub fn array_stats(arr: [i32; 5]) -> (i32, i32, i32, f64) {
  let mut sum = 0;
  let mut max = 0;
  let mut min = 0;
  let mut counter = 0;

  for i in arr {
    counter += 1;
    sum += i;

    if max > i {
      max = i
    } else if min < i {
      min = i
    }
  }

  let average = sum as f64 / counter as f64;
  return (sum, max, min, average);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_array_stats() {
    let array: [i32; 5] = [1, -2, 3, 4, 5];
    let result = array_stats(array);
    println!("result {:?}", result)
  }
}
