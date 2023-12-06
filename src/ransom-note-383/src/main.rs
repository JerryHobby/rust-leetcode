/////////////////////////////////////////////////////////////
// 383. Ransom Note
// https://leetcode.com/problems/ransom-note/description/

use std::time;

static MODULE_NAME: &str = "ransom-note-383";

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
        a: String,
        b: String,
        result: bool,
    }

    let test_cases = [
        TestCase {
            a: "a".to_string(),
            b: "b".to_string(),
            result: false,
        },
        TestCase {
            a: "aa".to_string(),
            b: "ab".to_string(),
            result: false,
        },
        TestCase {
            a: "aa".to_string(),
            b: "aab".to_string(),
            result: true,
        },
    ];

    for i in 0..test_cases.len() {
        // rename solution
        if Solution::can_construct(
            test_cases[i].a.to_string(),
            test_cases[i].b.to_string()) == test_cases[i].result {
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
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut magazine_counts: [u16; 26] = [0; 26];

        for ch in magazine.bytes() {
            magazine_counts[(ch - b'a') as usize] += 1;
        }
        for ch in ransom_note.bytes() {
            let index = (ch - b'a') as usize;
            match magazine_counts[index].checked_sub(1) {
                Some(count) => magazine_counts[index] = count,
                None => return false,
            }
        }
        true
    }
}