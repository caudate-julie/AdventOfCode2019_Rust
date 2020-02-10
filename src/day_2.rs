#![allow(non_snake_case)]
#![allow(dead_code)]

use std::io;
use crate::intcode::Intcode;

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


fn task_A(mut intcode: Intcode, noun: i32, verb: i32) -> i32 {
    intcode.code[1] = noun;
    intcode.code[2] = verb;

    intcode.run();
    intcode.code[0]
}


fn task_B(intcode: Intcode, target: i32) -> i32 {
    for noun in 1..100 {
        for verb in 1..100 {
            if task_A(intcode.clone(), noun, verb) == target {
                return noun * 100 + verb;
            }
        }
    }
    panic!("solution not found");
}

pub fn solve() {
    let xs = read_data();
    let intcode = Intcode::blackbox(xs);
    println!("Task A: {}", task_A(intcode.clone(), 12, 2));
    println!("Task B: {}", task_B(intcode, 19690720));
}
