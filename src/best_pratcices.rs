// Problem 1 - Array Statistics
// fn array_stats(arr: [i32; 5]) -> (i32, i32, i32, f64) {
//     // Initialize with first element to handle all cases correctly
//     let mut sum = arr[0];
//     let mut max = arr[0];
//     let mut min = arr[0];
    
//     // Start from second element since first is already used
//     for &value in &arr[1..] {
//         sum += value;
        
//         if value > max {
//             max = value;
//         }
        
//         if value < min {
//             min = value;
//         }
//     }
    
//     let average = sum as f64 / arr.len() as f64;
//     (sum, max, min, average)
// }

// #[test]
// fn test_array_stats() {
//     // Test basic case
//     let result = array_stats([3, 7, 2, 9, 5]);
//     assert_eq!(result, (26, 9, 2, 5.2));
    
//     // Test with negatives
//     let result = array_stats([-5, -2, -8, -1, -3]);
//     assert_eq!(result, (-19, -1, -8, -3.8));
    
//     // Test mixed positives and negatives
//     let result = array_stats([1, -2, 3, -4, 5]);
//     assert_eq!(result, (3, 5, -4, 0.6));
    
//     // Test all same values
//     let result = array_stats([7, 7, 7, 7, 7]);
//     assert_eq!(result, (35, 7, 7, 7.0));
    
//     // Test with zeros
//     let result = array_stats([0, 0, 0, 0, 0]);
//     assert_eq!(result, (0, 0, 0, 0.0));
// }



// Problem 2: String Character Analysis
// fn analyze_string(s: String) -> (usize, usize, usize, usize) {
//     let mut total_chars = 0;
//     let mut number_alphabetic = 0;
//     let mut number_digit = 0;
//     let mut number_whitespace = 0;
    
//     for ch in s.chars() {
//         total_chars += 1;
        
//         if ch.is_alphabetic() {
//             number_alphabetic += 1;
//         }
        
//         if ch.is_ascii_digit() {  // or ch.is_digit(10) for any radix
//             number_digit += 1;
//         }
        
//         if ch.is_whitespace() {
//             number_whitespace += 1;
//         }
//     }
    
//     (total_chars, number_alphabetic, number_digit, number_whitespace)
// }

// #[test]
// fn test_analyze_string() {
//     // Test basic case
//     let result = analyze_string(String::from("Hello 123 World!"));
//     assert_eq!(result, (15, 10, 3, 2));  // Note: 15 chars total (! is included)
    
//     // Test empty string
//     let result = analyze_string(String::from(""));
//     assert_eq!(result, (0, 0, 0, 0));
    
//     // Test Unicode (character count ≠ byte count)
//     let result = analyze_string(String::from("café 123"));
//     assert_eq!(result, (8, 4, 3, 1));  // "é" is 1 char, 2 bytes
    
//     // Test only special characters
//     let result = analyze_string(String::from("!@#$%^&*()"));
//     assert_eq!(result, (10, 0, 0, 0));
    
//     // Test tabs and newlines
//     let result = analyze_string(String::from("Hello\tWorld\n"));
//     assert_eq!(result, (12, 10, 0, 2));  // \t and \n are whitespace
// }


// Tuple Matrix Operations 
// fn matrix_operations(
//     a: ((i32, i32, i32), (i32, i32, i32), (i32, i32, i32)),
//     b: ((i32, i32, i32), (i32, i32, i32), (i32, i32, i32)),
//     scalar: i32
// ) -> (
//     ((i32, i32, i32), (i32, i32, i32), (i32, i32, i32)),
//     ((i32, i32, i32), (i32, i32, i32), (i32, i32, i32)),
//     ((i32, i32, i32), (i32, i32, i32), (i32, i32, i32)),
// ) {
//     // Destructure for clarity
//     let ((a00, a01, a02), (a10, a11, a12), (a20, a21, a22)) = a;
//     let ((b00, b01, b02), (b10, b11, b12), (b20, b21, b22)) = b;
    
//     let addition = (
//         (a00 + b00, a01 + b01, a02 + b02),
//         (a10 + b10, a11 + b11, a12 + b12),
//         (a20 + b20, a21 + b21, a22 + b22),
//     );
    
//     let subtraction = (
//         (a00 - b00, a01 - b01, a02 - b02),
//         (a10 - b10, a11 - b11, a12 - b12),
//         (a20 - b20, a21 - b21, a22 - b22),
//     );
    
//     let scaled = (
//         (a00 * scalar, a01 * scalar, a02 * scalar),
//         (a10 * scalar, a11 * scalar, a12 * scalar),
//         (a20 * scalar, a21 * scalar, a22 * scalar),
//     );
    
//     (addition, subtraction, scaled)
// }


// Problem 4 Student Grade Management
// use std::collections::HashMap;

// #[derive(Debug, PartialEq)]
// enum Grade {
//     Numeric(u32),
//     Letter(char),
// }

// impl Grade {
//     fn to_numeric(&self) -> Option<u32> {
//         match self {
//             Grade::Numeric(score) => Some(*score),
//             Grade::Letter('A') => Some(90),
//             Grade::Letter('B') => Some(80),
//             Grade::Letter('C') => Some(70),
//             Grade::Letter('D') => Some(60),
//             Grade::Letter('F') => Some(50),
//             _ => None, // Invalid letter grade
//         }
//     }
// }

// struct Student {
//     name: String,
//     courses: HashMap<String, Grade>,
// }

// impl Student {
//     fn new(name: String) -> Self {
//         Student {
//             name,
//             courses: HashMap::new(),
//         }
//     }
    
//     fn add_course(&mut self, course_name: String, grade: Grade) {
//         self.courses.insert(course_name, grade);
//     }
    
//     fn average_grade(&self) -> Option<f64> {
//         if self.courses.is_empty() {
//             return None;
//         }
        
//         let total: Option<u32> = self.courses
//             .values()
//             .map(|grade| grade.to_numeric())
//             .sum();
        
//         total.map(|t| t as f64 / self.courses.len() as f64)
//     }
    
//     fn highest_grade(&self) -> Option<&Grade> {
//         self.courses
//             .values()
//             .max_by_key(|grade| grade.to_numeric().unwrap_or(0))
//     }
    
//     fn lowest_grade(&self) -> Option<&Grade> {
//         self.courses
//             .values()
//             .min_by_key(|grade| grade.to_numeric().unwrap_or(100))
//     }
    
//     fn convert_all_to_numeric(&mut self) {
//         for (course, grade) in self.courses.iter_mut() {
//             if let Some(numeric) = grade.to_numeric() {
//                 *grade = Grade::Numeric(numeric);
//             }
//         }
//     }
// }

// #[test]
// fn test_student_grades() {
//     let mut student = Student::new(String::from("Alice"));
    
//     // Test empty
//     assert_eq!(student.average_grade(), None);
//     assert_eq!(student.highest_grade(), None);
//     assert_eq!(student.lowest_grade(), None);
    
//     // Add courses
//     student.add_course(String::from("Math"), Grade::Numeric(85));
//     student.add_course(String::from("Science"), Grade::Letter('B'));
//     student.add_course(String::from("History"), Grade::Letter('A'));
    
//     // Test calculations
//     assert_eq!(student.average_grade(), Some((85.0 + 80.0 + 90.0) / 3.0));
    
//     // Test conversion
//     student.convert_all_to_numeric();
//     assert!(student.courses.values().all(|g| matches!(g, Grade::Numeric(_))));
// }


// Problem 5: Inventory System with Lifetimes (Advanced)
/**
 * struct Item<'a> {
    name: &'a str,
    days_until_expiry: u32,
    category: &'a str,
}

struct Inventory<'a> {
    items: Vec<Item<'a>>,
    warehouse_name: String,
}

impl<'a> Inventory<'a> {
    fn new(warehouse_name: String) -> Self {
        Inventory {
            items: Vec::new(),
            warehouse_name,
        }
    }

    fn add_item(&mut self, name: &'a str, days_until_expiry: u32, category: &'a str) {
        self.items.push(Item {
            name,
            days_until_expiry,
            category,
        });
    }

    // Return reference with same lifetime as the data in Inventory
    fn find_by_name(&self, name: &str) -> Option<&Item<'a>> {
        self.items.iter().find(|item| item.name == name)
    }

    // Return mutable reference - note the lifetime 'a applies to the Item reference
    fn update_expiry(&mut self, name: &str, new_days: u32) -> bool {
        if let Some(item) = self.items.iter_mut().find(|item| item.name == name) {
            item.days_until_expiry = new_days;
            true
        } else {
            false
        }
    }

    // Return vector of references to items expiring soon
    // The returned references have lifetime 'a (tied to Inventory's data)
    fn get_expiring_soon(&self, threshold: u32) -> Vec<&Item<'a>> {
        self.items
            .iter()
            .filter(|item| item.days_until_expiry <= threshold)
            .collect()
    }

    // Calculate average days until expiry for a category
    fn avg_expiry_by_category(&self, category: &str) -> Option<f64> {
        let (count, sum) = self
            .items
            .iter()
            .filter(|item| item.category == category)
            .map(|item| item.days_until_expiry)
            .fold((0, 0u64), |(count, sum), days| (count + 1, sum + days as u64));

        if count == 0 {
            None
        } else {
            Some(sum as f64 / count as f64)
        }
    }
}

 */