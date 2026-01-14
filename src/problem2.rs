// Problem 2: String Character Analysis (Medium)
/**
 * Description: Given a String, count:
 * Total number of characters
 * Number of alphabetic characters (a-z, A-Z)
 * Number of digit characters (0-9)
 * Number of whitespace characters (space, tab, newline)
 *
 * fn analyze_string(s: String) -> (usize, usize, usize, usize)
 * .is_alphabetic()
 * .is_digit(10)
 * .is_whitespace()
 */
pub fn analyze_string(s: &String) -> (usize, usize, usize, usize) {
  let total_chars = s.len();
  let mut number_alphabetic = 0;
  let mut number_digit = 0;
  let mut number_whitespace = 0;
  let chars = s.chars();
  for value in chars {
    if value.is_alphabetic() {
      number_alphabetic += 1;
    }

    if value.is_whitespace() {
      number_whitespace += 1;
    }

    if value.is_digit(10) {
      number_digit += 1;
    }
  }

  (
    total_chars,
    number_alphabetic,
    number_digit,
    number_whitespace,
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_analyze_string() {
    let hello = String::from("HELO helo guys 321 ");
    let result = analyze_string(&hello);

    println!("{:?}", result);
  }
}
