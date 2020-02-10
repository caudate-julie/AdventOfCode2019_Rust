#![allow(non_snake_case)]
#![allow(dead_code)]

use std::io;
use crate::intcode::run;

fn read_data() -> Vec<i32> {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let s: Vec<&str> = s.split(",").collect();

    let mut xs: Vec<i32> = Vec::new();
    for number in s {
        xs.push(number.trim().parse().unwrap());
    }
    xs
}


fn task_A(mut intcode: Vec<i32>, noun: i32, verb: i32) -> i32 {
    intcode[1] = noun;
    intcode[2] = verb;

    run(&mut intcode, 0);
    intcode[0]
}


fn task_B(intcode: Vec<i32>, target: i32) -> i32 {
    for noun in 1..100 {
        for verb in 1..100 {
            if task_A(intcode.clone(), noun, verb) == target {
                return noun * 100 + verb;
            }
        }
    }
    panic!();
}


pub fn solve() {
    let xs = read_data();
    println!("Task A: {}", task_A(xs.clone(), 12, 2));
    println!("Task B: {}", task_B(xs, 19690720));
}
