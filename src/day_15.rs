#![allow(non_snake_case)]
#![allow(dead_code)]

use std::collections::HashMap;

use crate::intcode::Intcode;
use crate::intcode::long;

pub fn solve() {
    let code = Intcode::parse_file("inputs/day_15.txt");
    let mut intcode = Intcode::blackbox(code.to_vec());
    println!("Task A: {}", task_AB(&mut intcode));
    println!("Task B: {}", task_AB(&mut intcode));
}

fn task_AB(intcode: &mut Intcode) -> usize {
    let mut steps = 0;
    let mut area: HashMap<(long, long), (long, long)> = HashMap::new();
    let mut position = (0, 0);
    area.insert(position, position);

    let mut frontier: Vec<(long, long)> = Vec::new();
    frontier.push(position);

    while frontier.len() > 0 {
        steps += 1;
        let mut frontier2: Vec<(long, long)> = Vec::new();
        for p in frontier.iter() {
            for d in 1..5 {
                let dir = direction(d);
                let q = (p.0 + dir.0, p.1 + dir.1);
                if area.contains_key(&q) { continue; }

                let x = navigate(intcode, &area, &mut position, *p, q);
                match x {
                    0 => {
                        area.insert(q, q);
                    }
                    1 => {
                        area.insert(q, *p);
                        frontier2.push(q);
                    }
                    2 => {
                        area.insert(q, *p);
                        return steps;
                    }
                    _ => panic!(),
                }
            }
        }
        frontier = frontier2;
    }
    println!("All searched!");
    steps - 1
}

fn direction(d: long) -> (long, long) {
    match d {
        1 => (0, 1),
        2 => (0, -1),
        3 => (-1, 0),
        4 => (1, 0),
        _ => panic!(),
    }
}

fn cmd(d: (long, long)) -> long {
    match d {
        (0, 1) => 1,
        (0, -1) => 2,
        (-1, 0) => 3,
        (1, 0) => 4,
        _ => panic!(),
    }
}

fn navigate(intcode: &mut Intcode, 
         area: &HashMap<(long, long), (long, long)>, 
         position: &mut (long, long),
         prev: (long, long),
         dest: (long, long)) -> long {
    let mut path1 = pathfind(&area, *position);
    let mut path2 = pathfind(&area, prev);
    let mut center = (0, 0);

    while path1.len() > 0 && path2.len() > 0 && path1[path1.len() - 1] == path2[path2.len() - 1] {
        center = path1[path1.len() - 1];
        path1.pop();
        path2.pop();
    }

    path1.push(center);
    for p in path2.iter().rev() { path1.push(*p); }
    path1.push(dest);

    let mut result = 1;
    for i in 1..path1.len() {
        if result != 1 { panic!(); }
        let d = cmd((path1[i].0 - path1[i - 1].0, path1[i].1 - path1[i - 1].1));
        intcode.input.push(d);
    }

    intcode.run();
    result = intcode.output.pop().unwrap();
    *position = if result == 0 { path1[path1.len() - 2] } else { path1[path1.len() - 1] };

    result
}

fn pathfind(area: &HashMap<(long, long), (long, long)>, mut position: (long, long)) -> Vec<(long, long)> {
    let mut path: Vec<(long, long)> = Vec::new();
    while position != (0, 0) {
        path.push(position);
        position = area[&position];
    }

    path
}
