#![allow(non_snake_case)]
#![allow(dead_code)]

use std::fs::File;
use std::io::Read;
use std::iter::Iterator;

pub fn solve() {
    let mut file = File::open("inputs/day_8.txt").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    println!("Task A: {}", task_A(data.trim(), 25, 6));
    task_B(data.trim(), 25, 6);
}

fn task_A(data: &str, width: usize, height: usize) -> i32 {
    assert!(data.len() % (width * height) == 0);
    data.as_bytes()
        .chunks(width * height)
        .map(|layer| (layer.iter().filter(|x| **x == b'0').count(),
                      layer.iter().filter(|x| **x == b'1').count() * layer.iter().filter(|x| **x == b'2').count()))
        .min_by_key(|x| x.0)
        .unwrap()
        .1 as i32
}

fn task_B(data: &str, width: usize, height: usize) {
    let data = data.as_bytes();
    for i in 0..height {
        for j in 0..width {
            let mut layer = 0;
            let color = loop {
                let c = data[layer * width * height + i * width + j];
                if c != b'2' { break c; }
                layer += 1;
            };
            print!("{}", match color {
                b'0' => ' ',
                b'1' => '*',
                _ => panic!(),
            });
        }
        println!();
    }
}
