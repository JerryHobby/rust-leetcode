
// 289. Game of Life
//
// According to Wikipedia's article: "The Game of Life, also known simply as Life, is a cellular
// automaton devised by the British mathematician John Horton Conway in 1970."
//
// The board is made up of an m x n grid of cells, where each cell has an initial state:
// live (represented by a 1) or dead (represented by a 0). Each cell interacts with its eight
// neighbors (horizontal, vertical, diagonal) using the following four rules (
// taken from the above Wikipedia article):
//
// Any live cell with fewer than two live neighbors dies as if caused by under-population.
// Any live cell with two or three live neighbors lives on to the next generation.
// Any live cell with more than three live neighbors dies, as if by over-population.
// Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
// The next state is created by applying the above rules simultaneously to every cell in the
// current state, where births and deaths occur simultaneously. Given the current state of the
// m x n grid board, return the next state.

// Example 1:
//Input: board = [[0,1,0],[0,0,1],[1,1,1],[0,0,0]]
// Output: [[0,0,0],[1,0,1],[0,1,1],[0,1,0]]

// Example 2:
// Input: board = [[1,1],[1,0]]
// Output: [[1,1],[1,1]]

// Constraints:
// m == board.length
// n == board[i].length
// 1 <= m, n <= 25
// board[i][j] is 0 or 1.
//

use std::time;

struct Solution {}

static MODULE_NAME: &str = "game-of-life-289";
//https://leetcode.com/problems/game-of-life/description/?envType=study-plan-v2&envId=top-interview-150

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
        test: Vec<Vec<i32>>,
        result: Vec<Vec<i32>>,
    }

    let mut test_cases = [
        TestCase {
            test: vec![vec![1,2], vec![3,4]],
            result: vec![vec![1, 2], vec![3, 4]],
        },
        TestCase {
            test: vec![vec![1,2], vec![3,4]],
            result: vec![vec![1, 2], vec![3, 4]],
        },
    ];

    //
    println!("{}::run() <*>-<*>-<*>-<*>-<*>-<*>-<*>", MODULE_NAME);

    for i in 0..test_cases.len() {
        Solution::game_of_life(&mut test_cases[i].test);


        //       {
        //     println!("Test case {} passed", i);
        // } else {
        //     println!("Test case {} FAILED", i);
        // }
    }
}


/////////////////////////////////////////////////////////////
// Solution implementation

#[allow(non_snake_case)]

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        println!("{}", board[0][0]);
    }
}