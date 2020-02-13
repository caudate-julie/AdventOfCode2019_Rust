#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::intcode::Intcode;
use crate::intcode::MachineState;
use crate::intcode::long;

pub fn solve() {
    let code = Intcode::parse_file("inputs/day_7.txt");
    println!("Task A: {}", task_A(&code));
    println!("Task B: {}", task_B(&code));
}

fn task_A(code: &[long]) -> long {
    try_amplification(code, &mut vec![0,1,2,3,4], &mut vec![])
}

fn task_B(code: &[long]) -> long {
    try_amplification(code, &mut vec![5,6,7,8,9], &mut vec![])
}

fn try_amplification(code: &[long], phases: &mut Vec<long>, config: &mut Vec<long>) -> long {
    if phases.is_empty() {
        return try_configuration(code, &config);
    }

    let mut max_signal = long::min_value();
    for _ in 0..phases.len() {
        config.push(phases.remove(0));

        let signal = try_amplification(code, phases, config);
        if signal > max_signal { max_signal = signal; }

        phases.push(config.pop().unwrap());
    }
    max_signal
}

fn try_configuration(code: &[long], config: &[long]) -> long {
    let mut machines: Vec<Intcode> = Vec::new();
    for phase in config {
        machines.push(Intcode::machine(code.to_vec(), vec![*phase]));
    }
    let mut input = 0;
    loop {
        let mut halted = false;
        for m in &mut machines {
            m.input.push(input);
            if m.run() == MachineState::Halt {
                halted = true;
            }
            assert_eq!(m.output.len(), 1);
            input = m.output.remove(0);
        }
        if halted { break; }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amplifiers() {
        assert_eq!(
            task_A(&[3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0]), 
            43210,
        );
        assert_eq!(
            task_A(&[3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0]), 
            54321,
        );
        assert_eq!(
            task_A(&[3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
                     1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0]),
            65210,
        );
    }

    #[test]
    fn test_loops() {
        assert_eq!(
            task_B(&[3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
                     27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5]),
            139629729,
        );
        assert_eq!(
            task_B(&[3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
                     -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
                     53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10]),
            18216,
        );
    }
}
