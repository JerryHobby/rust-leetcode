/////////////////////////////////////////////////////////////
// 1688. Count of Matches in Tournament
// https://leetcode.com/problems/count-of-matches-in-tournament/description/

use std::time;

static MODULE_NAME: &str = "count-of-matches-in-tournament-1688";

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
            a: 7,
            result: 6,
        },
        TestCase {
            a: 14,
            result: 13,
        },
    ];

    for i in 0..test_cases.len() {
        // rename solution
        if Solution::number_of_matches(
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
    pub fn number_of_matches(n: i32) -> i32 {
// You are given an integer n, the number of teams in a tournament that has strange rules:
//
// If the current number of teams is even, each team gets paired with another team.
// A total of n / 2 matches are played, and n / 2 teams advance to the next round.
// If the current number of teams is odd, one team randomly advances in the tournament, and the rest gets paired.
// A total of (n - 1) / 2 matches are played, and (n - 1) / 2 + 1 teams advance to the next round.
// Return the number of matches played in the tournament until a winner is decided.

        let mut result = 0;
        let mut teams = n;
        let mut matches: i32;
        let mut carry: i32;

        //println!("total teams: {}", teams);

        while teams > 1 {
            //println!("teams in round: {}", teams);

            if teams % 2 == 0 {
                matches = teams / 2;
                carry = 0;
            } else {
                matches = (teams - 1) / 2;
                carry = 1;
            }
            teams = matches + carry;
            result += matches;
            // println!("teams advancing: {}", teams);
            // println!("matches in round: {}", matches);
            // println!("total matches so far: {}", result);
        }
        //println!("total matches: {}", result);

        result
    }
}