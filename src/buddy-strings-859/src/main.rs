/////////////////////////////////////////////////////////////
// 859. Buddy Strings
// Difficulty: Easy

// Given two strings s and goal, return true if you can swap two letters in s so the result
// is equal to goal, otherwise, return false.

// Swapping letters is defined as taking two indices i and j (0-indexed) such that i != j
// and swapping the characters at s[i] and s[j].
//
// For example, swapping at indices 0 and 2 in "abcd" results in "cbad".

use std::time;

struct Solution {}

static MODULE_NAME: &str = "buddy-strings-859";

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
        test: String,
        goal: String,
        result: bool,
    }

    let test_cases = [
        TestCase {
            test: "ab".to_string(),
            goal: "ba".to_string(),
            result: true,
        },
        TestCase {
            test: "aa".to_string(),
            goal: "aa".to_string(),
            result: true,
        },
        TestCase {
            test: "ab".to_string(),
            goal: "ab".to_string(),
            result: false,
        },
    ];

    println!("{}::run() <*>-<*>-<*>-<*>-<*>-<*>-<*>", MODULE_NAME);

    for i in 0..test_cases.len() {
        if Solution::buddy_strings(
            test_cases[i].test.to_string(),
            test_cases[i].goal.to_string()) == test_cases[i].result {
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
    pub fn buddy_strings(s: String, goal: String) -> bool {
        // different lengths - return false
        // index all mismatched chars
        // if there are more than 2, return false
        // if there are 2, swap them and compare

        if s.len() != goal.len() {
            return false;
        }

        let mut mismatched = Vec::new();
        let mut s_chars = s.chars();
        let mut g_chars = goal.chars();

        for i in 0..s.len() {
            let s_char = s_chars.next().unwrap();
            let g_char = g_chars.next().unwrap();
            if s_char != g_char {
                mismatched.push(i);
            }
        }

        if mismatched.len() == 0 {
            let mut s_chars = s.chars().collect::<Vec<char>>();
            s_chars.sort();
            for i in 0..s.len() - 1 {
                if s_chars[i] == s_chars[i + 1] {
                    return true;
                }
            }
            return false;
        }
        if mismatched.len() != 2 {
            return false;
        }

        if s.chars().nth(mismatched[0]).unwrap() == goal.chars().nth(mismatched[1]).unwrap() &&
            s.chars().nth(mismatched[1]).unwrap() == goal.chars().nth(mismatched[0]).unwrap() {
            return true;
        }
        false
    }
}