#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::intcode::Intcode;

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
    let code = Intcode::parse_file("inputs/day_2.txt");
    let intcode = Intcode::blackbox(code);
    println!("Task A: {}", task_A(intcode.clone(), 12, 2));
    println!("Task B: {}", task_B(intcode, 19690720));
}
