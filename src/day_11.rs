#![allow(non_snake_case)]
#![allow(dead_code)]

use std::collections::HashMap;

use crate::intcode::Intcode;
use crate::intcode::MachineState;
use crate::intcode::long;


pub fn solve() {
    let code = Intcode::parse_file("inputs/day_11.txt");
    println!("Task A: {}", task_A(&code));
    println!("Task B:");
    task_B(&code);
}

fn task_A(code: &[long]) -> long {
    hull_painting_robot(code, 0).len() as long
}

fn task_B(code: &[long]) {
    let painting = hull_painting_robot(code, 1);
    let min_x = painting.keys().map(|x| x.0).min().unwrap();
    let min_y = painting.keys().map(|x| x.1).min().unwrap();
    let max_x = painting.keys().map(|x| x.0).max().unwrap();
    let max_y = painting.keys().map(|x| x.1).max().unwrap();

    let mut canvas: Vec<Vec<long>> = vec![vec![0; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
    for k in painting.keys() {
        canvas[(k.1 - min_x) as usize][(k.0 - min_y) as usize] = painting[k];
    }
    for i in 0..canvas.len() {
        for j in 0..canvas[i].len() {
            print!("{}", if canvas[i][j] == 0 { " " } else { "#" });
        }
        println!();
    }
}

fn hull_painting_robot(code: &[long], starting: long) -> HashMap<(long, long), long> {
    let mut intcode = Intcode::blackbox(code.to_vec());
    let mut painting: HashMap<(long, long), long> = HashMap::new();
    let mut position = (0, 0);
    let mut direction = (0, -1);
    painting.insert(position, starting);
    
    loop {
        intcode.input.push(*painting.entry(position).or_insert(0));
        let result = intcode.run();
        assert_eq!(intcode.output.len(), 2);
        painting.insert(position, intcode.output.remove(0));
        let turn = -2 * intcode.output.remove(0) + 1;

        direction = (direction.1 * turn, -direction.0 * turn);
        position = (position.0 + direction.0, position.1 + direction.1);
        // dbg!(position);
    
        if result == MachineState::Halt { break; }
    }
    painting
}
