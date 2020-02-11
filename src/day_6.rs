#![allow(non_snake_case)]
#![allow(dead_code)]

use std::collections::HashMap;
use std::io::Read;

pub fn solve() {
    let mut file = std::fs::File::open("inputs/day_6.txt").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let orbit = parse(&data);
    println!("Task A: {}", task_A(&orbit));
    println!("Task B: {}", task_B(&orbit));
}

fn parse(data: &str) -> HashMap<&str, &str> {
    data.split("\n")
        .map(|x| {
            let mut y = x.trim().split(")");
            let center = y.next().unwrap();    // this is an iterator
            let planet = y.next().unwrap();
            assert!(y.next().is_none());
            (planet, center)
        })
        .collect()
}

fn task_A(orbit: &HashMap<&str, &str>) -> i32 {
    let mut count = 0;
    for mut planet in orbit.keys() {
        while let Some(center) = orbit.get(planet) {
            count += 1;
            planet = center;
        }
    }
    count
}

fn task_B(orbit: &HashMap<&str, &str>) -> i32 {
    let mut first: &str = "YOU";
    let mut second: &str = "SAN";

    let meetpoint = loop {
        if first == second { break first; }
        first = orbit.get(first).unwrap_or(&"SAN");
        second = orbit.get(second).unwrap_or(&"YOU");
    };

    let mut count = 0;
    first = orbit["YOU"];
    second = orbit["SAN"];
    while first != meetpoint {
        first = orbit[first];
        count += 1;
    }
    while second != meetpoint {
        second = orbit[second];
        count += 1;
    }
    count
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orbits_A() {
        let orbit = parse("COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L");
        assert_eq!(task_A(&orbit), 42);
    }

    #[test]
    fn test_orbits_B() {
        let orbit = parse("COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L
        K)YOU
        I)SAN");
        assert_eq!(task_B(&orbit), 4);
    }
}
