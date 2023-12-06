/////////////////////////////////////////////////////////////
// 1662. Check If Two String Arrays are Equivalent
// Given two string arrays word1 and word2, return true if the two arrays represent the same string, and false otherwise.
//
// A string is represented by an array if the array elements concatenated in order forms the string.
//
//
//
// Example 1:
//
// Input: word1 = ["ab", "c"], word2 = ["a", "bc"]
// Output: true
// Explanation:
// word1 represents string "ab" + "c" -> "abc"
// word2 represents string "a" + "bc" -> "abc"
// The strings are the same, so return true.
// Example 2:
//
// Input: word1 = ["a", "cb"], word2 = ["ab", "c"]
// Output: false
// Example 3:
//
// Input: word1  = ["abc", "d", "defg"], word2 = ["abcddefg"]
// Output: true

#[allow(unused)]
use std::ops::Add;
use std::time;


static MODULE_NAME: &str = "array-strings-are-equal-1662";

struct Solution {}

#[allow(dead_code)]
#[allow(non_snake_case)]

fn main() {
    println!("\n\n<*>-<*>-<*> {}::run() <*>-<*>-<*>", MODULE_NAME);
    let now = time::Instant::now();
    run();
    let elapsed_time = now.elapsed();
    println!("<*>-<*>-<*> Finished in {} microseconds. <*>-<*>-<*>", elapsed_time.as_micros());
}

pub fn run() {
    struct TestCase {
        a: Vec<String>,
        b: Vec<String>,
        result: bool,
    }

    let test_cases = [
        TestCase {
            a: vec!["a".to_string(), "cb".to_string()],
            b: vec!["ab".to_string(), "c".to_string()],
            result: false
        },
        TestCase {
            a: vec!["abc".to_string(), "d".to_string(), "defg".to_string()],
            b: vec!["abcddefg".to_string()],
            result: true
        },
        TestCase {
            a: vec!["a".to_string(), "bc".to_string()],
            b: vec!["ab".to_string(), "c".to_string()],
            result: true
        },
    ];


    for i in 0..test_cases.len() {
        if Solution::array_strings_are_equal(
            &test_cases[i].a,
            &test_cases[i].b) == test_cases[i].result {
            println!("Test case {} passed", i);
        } else {
            println!("Test case {} FAILED", i);
        }
    }
}


/////////////////////////////////////////////////////////////
// Solution implementation

#[allow(non_snake_case)]

impl Solution {
    pub fn array_strings_are_equal(word1: &Vec<String>, word2: &Vec<String>) -> bool {
        // convert word1 into simple string
        // convert word2 into simple string
        // compare strings

        // other coder solution - better than mine.  ONE LINE
        word1.concat() == word2.concat()

        //
        //     let mut word1_string: String = String::new();
        //     let mut word2_string: String = String::new();
        //
        //     for x in word1 {
        //         word1_string = word1_string.add(x);
        //     }
        //     for x in word2 {
        //         word2_string = word2_string.add(x);
        //     }
        //
        //     word1_string.eq(&word2_string)
        // }
    }
}