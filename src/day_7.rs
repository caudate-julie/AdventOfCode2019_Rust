#![allow(non_snake_case)]
// #![allow(dead_code)]

use crate::intcode::Intcode;

pub fn solve() {
    let code = Intcode::parse_file("inputs/day_7.txt");
    println!("Task A: {}", task_A(code))
}

fn task_A(code: Vec<i32>) -> i32 {
    try_amplifier(&code, &[0,1,2,3,4], 0)
}

fn try_amplifier(code: &[i32], phases: &[i32], input: i32) -> i32 {
    if phases.len() == 0 { return input; }

    let mut max_signal = i32::min_value();
    let mut left_phases = phases.to_vec();
    for _ in 0..phases.len() {
        let stash = left_phases.remove(0);

        let mut intcode = Intcode::machine(code.to_vec(), vec![stash, input]);
        intcode.run();

        let signal = try_amplifier(code, &left_phases, intcode.output[0]);
        if signal > max_signal { max_signal = signal; }
        left_phases.push(stash);
    }
    max_signal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amplifiers() {
        assert_eq!(task_A(vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0]), 43210);
        assert_eq!(
            task_A(vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0]), 
            54321,
        );
        assert_eq!(
            task_A(vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
                        1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0]),
            65210,
        );
    }
}
