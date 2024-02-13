                    // Rust Developer Profile Set-1 /*/


// 1.Implement a function that checks whether a given string is a palindrome or not.
fn is_palindrome(s: &str) -> bool {
    // Convert the string to lowercase for case-insensitive comparison
    let s = s.to_lowercase();

    // Convert the string to a vector of characters
    let chars: Vec<char> = s.chars().collect();

    // Initialize pointers for the start and end of the string
    let mut start = 0;
    let mut end = chars.len() - 1;

    // Iterate until the pointers meet in the middle
    while start < end {
        // Compare characters at both ends
        if chars[start] != chars[end] {
            // If characters don't match, return false
            return false;
        }
        // Move pointers closer to the middle
        start += 1;
        end -= 1;
    }
    // If all characters match, return true
    true
}

fn main() {
    // Test cases
    let test_cases = vec!["radar", "level", "hello", "A man, a plan, a canal, Panama"];
    for &test_case in &test_cases {
        println!("Is '{}' a palindrome? {}", test_case, is_palindrome(test_case));
    }
}



// 2.Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.
fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    // Initialize pointers for binary search
    let mut left = 0;
    let mut right = arr.len();

    // Perform binary search
    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    // Check if the target is found at the left pointer
    if left < arr.len() && arr[left] == target {
        Some(left)
    } else {
        None
    }
}

fn main() {
    // Sorted array
    let arr = vec![1, 3, 5, 5, 7, 9, 9, 11];

    // Test cases
    let targets = vec![5, 9, 2, 11];
    for target in targets {
        if let Some(index) = find_first_occurrence(&arr, target) {
            println!("First occurrence of {} is at index {}", target, index);
        } else {
            println!("{} not found in the array", target);
        }
    }
}


// 3.Given a string of words, implement a function that returns the shortest word in the string.
fn find_shortest_word(s: &str) -> Option<&str> {
    // Split the string into words
    let words: Vec<&str> = s.split_whitespace().collect();

    // Find the shortest word
    words.iter().min_by_key(|&word| word.len()).cloned()
}

fn main() {
    // Test string
    let s = "They qui brown fox jumps over the lazy dog";

    // Find the shortest word
    if let Some(shortest_word) = find_shortest_word(s) {
        println!("Shortest word: {}", shortest_word);
    } else {
        println!("No words found in the string");
    }
}



// 4.Implement a function that checks whether a given number is prime or not.
fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn main() {
    let num = 17; // Change the number to test here
    if is_prime(num) {
        println!("{} is prime", num);
    } else {
        println!("{} is not prime", num);
    }
}



// 5. Given a sorted array of integers, implement a function that returns the median of the array.
fn median(nums: &[i32]) -> f64 {
    let len = nums.len();
    if len % 2 == 0 {
        let mid = len / 2;
        let median_value = (nums[mid - 1] + nums[mid]) as f64 / 2.0;
        median_value
    } else {
        let mid = len / 2;
        let median_value = nums[mid] as f64;
        median_value
    }
}

fn main() {
    let arr = vec![1,2,3,4,5]; // Change the array here
    let result = median(&arr);
    println!("Median of the array is: {}", result);
}


// 6.Implement a function that finds the longest common prefix of a given set of strings.
fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let first_string = &strings[0];
    let mut longest_prefix = String::new();

    'outer: for (i, c) in first_string.chars().enumerate() {
        for string in strings.iter().skip(1) {
            if let Some(sc) = string.chars().nth(i) {
                if sc != c {
                    break 'outer;
                }
            } else {
                break 'outer;
            }
        }
        longest_prefix.push(c);
    }

    longest_prefix
}

fn main() {
    let strings = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ]; // Change the strings here
    let result = longest_common_prefix(&strings);
    println!("Longest common prefix is: {}", result);
}



// 7.Implement a function that returns the kth smallest element in a given array.
fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > arr.len() {
        return None; // Invalid k value
    }

    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();

    Some(sorted_arr[k - 1])
}

fn main() {
    let arr = [7, 10, 4, 3, 20, 15];
    let k = 3;
    match kth_smallest(&arr, k) {
        Some(smallest) => println!("The {}th smallest element is: {}", k, smallest),
        None => println!("Invalid k value"),
    }
}



// 8.Given a binary tree, implement a function that returns the maximum depth of the tree.
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;

fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(node) => {
            let node_ref = node.borrow();
            let left_depth = max_depth(node_ref.left.clone());
            let right_depth = max_depth(node_ref.right.clone());
            1 + std::cmp::max(left_depth, right_depth)
        }
        None => 0,
    }
}

fn main() {
    // Example usage:
    // Create a binary tree
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));

    // Calculate the maximum depth of the tree
    let depth = max_depth(root);
    println!("Maximum depth of the tree: {}", depth);
}



// 9.Reverse a string in Rust
fn reverse_string(input: &str) -> String {
    let mut result = String::new();
    for ch in input.chars().rev() {
        result.push(ch);
    }
    result
}

fn main() {
    let input_string = "Hello, world!";
    let reversed_string = reverse_string(input_string);
    println!("Reversed string: {}", reversed_string);
}



// 10.Check if a number is prime in Rust
fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn main() {
    let num = 10;
    if is_prime(num) {
        println!("{} is prime", num);
    } else {
        println!("{} is not prime", num);
    }
}


// 11.Merge two sorted arrays in Rust
fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let (mut i, mut j) = (0, 0);

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            result.push(arr1[i]);
            i += 1;
        } else {
            result.push(arr2[j]);
            j += 1;
        }
    }

    result.extend_from_slice(&arr1[i..]);
    result.extend_from_slice(&arr2[j..]);

    result
}

fn main() {
    let arr1 = [10, 30, 15, 7, 9];
    let arr2 = [2, 4, 60, 8, 10];
    let merged = merge_sorted_arrays(&arr1, &arr2);
    println!("Merged array: {:?}", merged);
}





// 12.Find the maximum subarray sum in Rust
fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let (mut i, mut j) = (0, 0);

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            result.push(arr1[i]);
            i += 1;
        } else {
            result.push(arr2[j]);
            j += 1;
        }
    }

    result.extend_from_slice(&arr1[i..]);
    result.extend_from_slice(&arr2[j..]);

    result
}

fn main() {
    let arr1 = [10, 30, 15, 7, 9];
    let arr2 = [2, 4, 60, 8, 10];
    let merged = merge_sorted_arrays(&arr1, &arr2);
    println!("Merged array: {:?}", merged);
}





















