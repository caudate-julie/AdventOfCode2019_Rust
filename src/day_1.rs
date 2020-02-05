#![allow(non_snake_case)]

use std::io;

fn task_A(xs: &Vec<i32>) -> i32 {
    let mut answer = 0;
    for x in xs {
        answer += x / 3 - 2;
    }
    answer
}

fn task_B(xs: &Vec<i32>) -> i32 {
    let mut answer = 0;
    for x in xs {
        let mut a : i32 = *x;
        loop {
            a = a / 3 - 2;
            if a <= 0 { break; }
            answer += a;
        }
    }
    answer
}

fn read_data() -> Vec<i32> {
    let mut xs = Vec::new();
    loop {
        let mut weight = String::new();
        io::stdin().read_line(&mut weight).unwrap();
        if weight == "" { break; }
        let a: i32 = weight.trim().parse().unwrap();
        xs.push(a);
    }
    xs
}

pub fn solve() {
    let xs = read_data();

    println!("Task A: {}", task_A(&xs));
    println!("Task B: {}", task_B(&xs));
}
