/////////////////////////////////////////////////////////////
// 1160. Find Words That Can Be Formed by Characters
// You are given an array of strings words and a string chars.
//
// A string is good if it can be formed by characters from chars (each character can only be used once).
//
// Return the sum of lengths of all good strings in words.
// Example 1:
//
// Input: words = ["cat","bt","hat","tree"], chars = "atach"
// Output: 6
// Explanation: The strings that can be formed are "cat" and "hat" so the answer is 3 + 3 = 6.
// Example 2:
//
// Input: words = ["hello","world","leetcode"], chars = "welldonehoneyr"
// Output: 10
// Explanation: The strings that can be formed are "hello" and "world" so the answer is 5 + 5 = 10.

use std::time;

static MODULE_NAME: &str = "find-words-that-can-be-formed-by-characters-1160";

#[allow(dead_code)]
#[allow(non_snake_case)]

fn main() {
    println!("\n\n<*>-<*>-<*> {}::run() <*>-<*>-<*>", MODULE_NAME);
    let now = time::Instant::now();
    run();
    let elapsed_time = now.elapsed();
    println!("<*>-<*>-<*> Finished in {} microseconds. <*>-<*>-<*>", elapsed_time.as_micros());
}

struct Solution {}

pub fn run() {
    struct TestCase {
        a: Vec<String>,
        b: String,
        result: i32,
    }

    let test_cases = [
        TestCase {
            a: vec!("cat".to_string(),"bt".to_string(),"hat".to_string(),"tree".to_string()),
            b: "atach".to_string(),
            result: 6,
        },
        TestCase {
            a: vec!("hello".to_string(),"world".to_string(),"leetcode".to_string()),
            b: "welldonehoneyr".to_string(),
            result: 10,
        },
    ];

    for i in 0..test_cases.len() {
        if Solution::count_characters(
            test_cases[i].a.clone(),
            test_cases[i].b.clone()) == test_cases[i].result {
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
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut bank = [0; 26];
        for i in chars.as_bytes() {
            bank[*i as usize - 'a' as usize] += 1;
        }
        let bank = bank;    // make immutable
        let mut ans = 0;
        'out: for w in words.iter() {
            let mut bank = bank;
            for i in w.as_bytes() {
                let idx = *i as usize - 'a' as usize;
                bank[idx] -= 1;
                if bank[idx] < 0 {
                    continue 'out;
                }
            }
            ans += w.len();
        }
        ans as i32
    }
}