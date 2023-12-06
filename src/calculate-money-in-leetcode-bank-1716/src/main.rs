/////////////////////////////////////////////////////////////
// 1716. Calculate Money in Leetcode Bank
// https://leetcode.com/problems/calculate-money-in-leetcode-bank/description/

use std::time;

static MODULE_NAME: &str = "calculate-money-in-leetcode-bank-1716";

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
        result: i32,
    }

    let test_cases = [

        TestCase {
            a: 10,
            result: 37,
        },
        TestCase {
            a: 20,
            result: 96,
        },
        TestCase {
            a: 4,
            result: 10,
        },
    ];

    for i in 0..test_cases.len() {
        // rename solution
        if Solution::total_money(
            test_cases[i].a) == test_cases[i].result {
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
    pub fn total_money(n: i32) -> i32 {
        let mut bank:i32 = 0;
        let mut save = 1;

        for day in 0..n {
            if day % 7 == 0 {
                save = day/7 + 1;
            } else {
                save +=  1;
            }
            bank += save;
        }
        bank
    }
}

// impl Solution {
//     pub fn total_money(n: i32) -> i32 {
//         let weeks = n / 7;
//         let current_day = n % 7;
//
//         let x1 = 28 * weeks;
//         let x2 = 7 * weeks * (weeks - 1) / 2;
//         let x3 = weeks * current_day + (current_day * (current_day + 1) / 2);
//
//         x1 + x2 + x3
//     }
// }