fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test() {
    println!("Hello, test!");
}


// learning rust based on hackrrank's styles
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


fn array_stats (arr : [i32 ; 5]) -> (i32, i32, i32, f64)  {
    let mut sum = 0;
    let mut max = 0;
    let mut min = 0;
    let mut counter = 0; 

    
    for i in arr {
        counter +=1;
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

#[test]
fn test_array_stats (){
        let array : [i32;5] = [1, -2, 3 ,4 ,5];
    let result = array_stats(array);
    println!("result {:?}", result)
}

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
fn analyze_string (s : &String)-> (usize, usize, usize, usize) {
    let total_chars = s.len();
    let mut number_alphabetic = 0;
    let mut number_digit = 0;
    let mut number_whitespace = 0;
    let chars = s.chars();
    for value in chars {
        if value.is_alphabetic() {
            number_alphabetic+=1;
        }

        if value.is_whitespace() {
            number_whitespace += 1;
        }

        if value.is_digit(10) {
            number_digit +=1;
        }
    }

    (total_chars, number_alphabetic, number_digit, number_whitespace)
}


#[test]
fn test_analyze_string (){
    let hello = String::from("HELO helo guys 321 ");
    let result = analyze_string(&hello);

    println!("{:?}",result);
}


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

fn matrix_operations(
    a: ((i32, i32, i32), (i32, i32, i32), (i32, i32, i32)),
    b: ((i32, i32, i32), (i32, i32, i32), (i32, i32, i32)),
    scalar: i32
) 
// -> (
//     ((i32, i32, i32), (i32, i32, i32), (i32, i32, i32)),
//     ((i32, i32, i32), (i32, i32, i32), (i32, i32, i32)),
//     ((i32, i32, i32), (i32, i32, i32), (i32, i32, i32)),
// ) 

{
    println!("a {:?}", a);
    println!("b {:?}", b);
    println!("scalar {}", scalar);
}

#[test]
fn test_matrix_operations () {}