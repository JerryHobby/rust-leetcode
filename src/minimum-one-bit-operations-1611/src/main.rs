// /////////////////////////////////////////////////////////////
// // 1611. Minimum One Bit Operations to Make Integers Zero
// // Hard
// // Given an integer n, you must transform it into 0 using the following operations any number of times:
// //
// // Change the rightmost (0th) bit in the binary representation of n.
// // Change the ith bit in the binary representation of n if the (i-1)th bit is set to 1 and the (i-2)th through 0th bits are set to 0.
// // Return the minimum number of operations to transform n into 0.

use std::time;

struct Solution {}

static MODULE_NAME: &str = "minimum-one-bit-operations-1611";

#[allow(dead_code)]

fn main() {
    println!("\n\n<*>-<*>-<*> {}::run() <*>-<*>-<*>", MODULE_NAME);
    let now = time::Instant::now();
    run();
    let elapsed_time = now.elapsed();
    println!("<*>-<*>-<*> Finished in {} microseconds. <*>-<*>-<*>", elapsed_time.as_micros());
}

pub fn run() {
    struct TestCase {
        a: i32,
        result: i32,
    }

    let test_cases = [
        TestCase {
            a: 3, // 0b00011
            result: 2,
        },
        TestCase {
            a: 6, // 0b00110
            result: 4,
        },
    ];

    println!("{}::run() <*>-<*>-<*>-<*>-<*>-<*>-<*>", MODULE_NAME);

    for i in 0..test_cases.len() {
        if Solution::minimum_one_bit_operations(
            test_cases[i].a) == test_cases[i].result {
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
    pub fn minimum_one_bit_operations(mut n: i32) -> i32 {
        let (mut res, mut curr) = (0, 1);
        while n != 0 {
            if n % 2 == 1 {
                res = curr - res;
            }
            curr = 2 * curr + 1;
            n /= 2;
        }
        return res;
    }
}