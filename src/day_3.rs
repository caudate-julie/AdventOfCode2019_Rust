#![allow(non_snake_case)]
#![allow(dead_code)]

use std::io;

fn read_data() -> (Vec<String>, Vec<String>) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let w1 = parse_data(&input);
    
    input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let w2 = parse_data(&input);
    
    (w1, w2)
}

fn parse_data(raw: &str) -> Vec<String> {
    raw.split(",").map(|x| String::from(x.trim())).collect()
}

fn direct(code: &str) -> (i32, i32) {
    let result = match code.as_bytes()[0] {
        b'U' => (0, 1),
        b'D' => (0, -1),
        b'L' => (-1, 0),
        b'R' => (1, 0),
        _ => panic!("{}", code.as_bytes()[0]),
    };
    result
}

fn dist(code: &str) -> i32 {
    code[1..].parse().unwrap()
}

fn boundaries(wire: &[String], min_x: &mut i32, max_x: &mut i32, min_y: &mut i32, max_y: &mut i32) {
    let mut x = 0;
    let mut y = 0;
    for cmd in wire {
        let (dx, dy) = direct(cmd);
        x += dx * dist(cmd);
        y += dy * dist(cmd);
        if x < *min_x { *min_x = x; }
        if x > *max_x { *max_x = x; }
        if y < *min_y { *min_y = y; }
        if y > *max_y { *max_y = y; }
    }
}

fn task_AB(wire1: &[String], wire2: &[String]) -> (i32, i32) {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    boundaries(wire1, &mut min_x, &mut max_x, &mut min_y, &mut max_y);
    boundaries(wire2, &mut min_x, &mut max_x, &mut min_y, &mut max_y);
    let mut platter: Vec<Vec<i32>> = vec![vec![0; (max_y - min_y + 1) as usize]; (max_x - min_x + 1) as usize];

    let mut x = -min_x;
    let mut y = -min_y;
    let mut steps = 0;
    for cmd in wire1 {
        let (dx, dy) = direct(cmd);
        for _ in 0..dist(cmd) {
            x += dx;
            y += dy;
            steps += 1;
            if platter[x as usize][y as usize] == 0 {
                platter[x as usize][y as usize] = steps;
            }
        }
    }

    x = -min_x;
    y = -min_y;
    steps = 0;
    let mut min_d = max_x - min_x + max_y - min_y + 2;
    let mut min_t = min_d * min_d;
    for cmd in wire2 {
        let (dx, dy) = direct(cmd);
        for _ in 0..dist(cmd) {
            x += dx;
            y += dy;
            steps += 1;
            if platter[x as usize][y as usize] != 0 {
                let d = (x + min_x).abs() + (y + min_y).abs();
                if d != 0 && d < min_d { min_d = d; }

                let t = platter[x as usize][y as usize] + steps;
                if t < min_t { min_t = t; }
            }
        }
    }
    (min_d, min_t)
}

pub fn solve() {
    let (wire1, wire2) = read_data();
    let (d, t) = task_AB(&wire1, &wire2);
    println!("Task A: {}", d);
    println!("Task B: {}", t);
}

// ================== TESTS ======================

#[cfg(test)]
fn test_one_A(w1: &str, w2: &str, result: i32) {
    let (d, _) = task_AB(&parse_data(w1), &parse_data(w2));
    assert_eq!(d, result);
}

#[test]
fn test_all_A() {
    test_one_A("R8,U5,L5,D3", "U7,R6,D4,L4", 6);
    test_one_A("R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83", 159);
    test_one_A("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7", 135);
}

#[cfg(test)]
fn test_one_B(w1: &str, w2: &str, result: i32) {
    let (_, t) = task_AB(&parse_data(w1), &parse_data(w2));
    assert_eq!(t, result);
}

#[test]
fn test_all_B() {
    test_one_B("R8,U5,L5,D3", "U7,R6,D4,L4", 30);
    test_one_B("R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83", 610);
    test_one_B("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7", 410);
}
