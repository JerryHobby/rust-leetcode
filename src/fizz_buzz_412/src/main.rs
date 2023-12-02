/////////////////////////////////////////////////////////////
// 412. Fizz Buzz
// https://leetcode.com/problems/fizz-buzz/description/

use std::time;

static MODULE_NAME: &str = "fizz_buzz_412";

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
        a: i32,
        result: Vec<String>,
    }

    let test_cases = [
        TestCase {
            a: 3,
            result: ["1".to_string(),"2".to_string(),"Fizz".to_string()].to_vec(),
        },
        TestCase {
            a: 5,
            result: ["1".to_string(),"2".to_string(),"Fizz".to_string(),
                "4".to_string(),"Buzz".to_string()].to_vec(),
        },
        TestCase {
            a: 15,
            result: ["1".to_string(),"2".to_string(),"Fizz".to_string(),
                "4".to_string(),"Buzz".to_string(),"Fizz".to_string(),
                "7".to_string(),"8".to_string(),"Fizz".to_string(),"Buzz".to_string(),
                "11".to_string(),"Fizz".to_string(),"13".to_string(),
                "14".to_string(),"FizzBuzz".to_string()].to_vec(),
        },
    ];

    for i in 0..test_cases.len() {
        // rename solution
        if Solution::fizz_buzz(test_cases[i].a) == test_cases[i].result {
            println!("Test case {} passed", i + 1);
        } else {
            println!("Test case {} FAILED", i + 1);
        }
    }
}


/////////////////////////////////////////////////////////////
// Solution implementation

#[allow(non_snake_case)]

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        for x in 1..=n {
            if x % 3 == 0 && x % 5 == 0 {
                result.push("FizzBuzz".to_string());
            } else if x % 3 == 0 {
                result.push("Fizz".to_string());
            } else if x % 5 == 0 {
                result.push("Buzz".to_string());
            } else {
                result.push(x.to_string());
            }
        }
        result
    }
}