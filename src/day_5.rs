#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::intcode::Intcode;
use std::fs::File;
use std::io::Read;

pub fn solve() {
    let mut file = File::open("inputs/day_5.txt").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let code: Vec<i32> = data.split(",").map(|x| x.trim().parse().unwrap()).collect();

    println!("Task A: {}", task_A(code));
}


fn task_A(code: Vec<i32>) -> i32 {
    let mut intcode = Intcode::machine(code, vec![1]);
    intcode.run();
    let (&result, tests) = intcode.output.split_last().unwrap();
    for &x in tests {
        assert_eq!(x, 0);
    }
    result
}