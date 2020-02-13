#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::intcode::Intcode;
use crate::intcode::MachineState;
use crate::intcode::long;

pub fn solve() {
    let code: Vec<long> = Intcode::parse_file("inputs/day_9.txt");
    
    println!("Task A: {}", task_AB(code.clone(), 1));
    println!("Task B: {}", task_AB(code.clone(), 2));
}

fn task_AB(code: Vec<long>, inp: long) -> long {
    let mut intcode = Intcode::machine(code, vec![inp]);
    let state = intcode.run();
    assert!(state == MachineState::Halt);
    assert_eq!(intcode.output.len(), 1);
    intcode.output[0]
}
