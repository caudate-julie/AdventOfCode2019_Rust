#![allow(non_snake_case)]

use std::io;

pub fn task_A(xs: &Vec<i32>) -> i32 {
    let mut answer = 0;
    for x in xs {
        answer += x / 3 - 2;
    }
    answer
}

pub fn task_B(xs: &Vec<i32>) -> i32 {
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

pub fn read_data() -> Vec<i32> {
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