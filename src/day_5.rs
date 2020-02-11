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

    println!("Task A: {}", task_AB(code.clone(), 1));
    println!("Task B: {}", task_AB(code, 5));
}


fn task_AB(code: Vec<i32>, inp: i32) -> i32 {
    let mut intcode = Intcode::machine(code, vec![inp]);
    intcode.run();
    let (&result, tests) = intcode.output.split_last().unwrap();
    for &x in tests {
        assert_eq!(x, 0);
    }
    result
}
