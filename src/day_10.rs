#![allow(non_snake_case)]
#![allow(dead_code)]

use std::fs::File;
use std::io::Read;
use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::collections::HashMap;
use std::ops;

pub fn solve() {
    let mut file = File::open("inputs/day_10.txt").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let data: Vec<&str> = data.split('\n').map(|x| x.trim()).collect();
    let (result, center) = task_A(&data);
    println!("Task A: {}", result);
    let target = task_B(&data, center, 200);
    println!("Task B: {}", target.x * 100 + target.y);
}

// ----------- Coord struct ----------------

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 && a == 0 { return 1; }
    if b == 0 { a } else { gcd(b, a % b) }
}

#[derive(PartialEq, Eq)]
#[derive(Clone, Copy)]
#[derive(Hash)]
#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32,
}

impl ops::Add for Coord {
    type Output = Coord;
    fn add(self, other: Coord) -> Coord {
        Coord { x: self.x + other.x, y: self.y + other.y }
    }
}

impl ops::Sub for Coord {
    type Output = Coord;
    fn sub(self, other: Coord) -> Coord {
        Coord { x: self.x - other.x, y: self.y - other.y }
    }
}

impl ops::Mul<i32> for Coord {
    type Output = Coord;
    fn mul(self, other: i32) -> Coord {
        Coord { x: self.x * other, y: self.y * other }
    }
}

impl ops::Div<i32> for Coord {
    type Output = Coord;
    fn div(self, other: i32) -> Coord {
        assert!(other != 0 && self.x % other == 0 && self.y % other == 0);
        Coord { x: self.x / other, y: self.y / other }
    }
}

impl Ord for Coord {
    fn cmp(&self, other: &Coord) -> Ordering {
        let Coord { x: ax, y: ay } = *self;
        let Coord { x: bx, y: by } = *other;

        if ax == 0 && bx == 0 && ay < 0 && by < 0 { return Ordering::Equal; }
        if ax == 0 && ay < 0 { return Ordering::Less; }
        if bx == 0 && by < 0 { return Ordering::Greater; }

        if ax * bx < 0 { 
            return if ax < 0 { Ordering::Greater } else { Ordering::Less };
        }

        if ay * by < 0 {
            return if ay < 0 {
                if ax >= 0 { Ordering::Less } else { Ordering::Greater }
            }
            else {
                if bx >= 0 { Ordering::Greater } else { Ordering::Less }
            }
        }

        (bx * ay).cmp(&(ax * by))
    }
}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Coord) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Coord {
    fn gcd(&self) -> i32 {
        return gcd(self.x.abs(), self.y.abs());
    }

    fn angle(&self) -> Coord {
        *self / self.gcd()
    }

    fn new(i: i32, j: i32) -> Coord {
        Coord { x: j, y: i }
    }
}

// --------- Task A: Work with asteroid map ------------

fn get(map: &[&str], p: Coord) -> u8 {
    return map[p.y as usize].as_bytes()[p.x as usize]
}

fn is_visible(map: &[&str], a: Coord, b: Coord) -> bool {
    let delta = a - b;
    let t = delta.gcd();
    let delta = delta / t;

    for coeff in 1..t {
        let c = b + delta * coeff;
        if get(map, c) == b'#' {
            return false;
        }
    }
    true
}

fn task_A(map: &[&str]) -> (i32, Coord) {
    let mut max = 0;
    let mut coord = Coord::new(0, 0);
    let height = map.len() as i32;
    let width = map[0].len() as i32;

    for i in 0..height {
        for j in 0..width {
            let current = Coord::new(i, j);
            if get(map, current) == b'.' { continue; }
            
            let mut count = -1;
            for ii in 0..height {
                for jj in 0..width {
                    let observed = Coord::new(ii, jj);
                    if get(map, observed) == b'.' { continue; }
                    if is_visible(map, current, observed) {
                        count += 1;
                    }
                }
            }
            if count > max { 
                max = count;
                coord = current;
            }
        }
    }
    (max, coord)
}

fn get_asteroid_list(map: &[&str], center: Coord) -> Vec<Coord> {
    let mut asts: Vec<Coord> = Vec::new();
    let height = map.len() as i32;
    let width = map[0].len() as i32;

    for i in 0..height {
        for j in 0..width {
            let c = Coord::new(i, j);
            if get(map, c) == b'#' {
                asts.push(c - center);
            }
        }
    }
    asts
}

fn task_B(map: &[&str], center: Coord, target: i32) -> Coord {
    let asteroids = get_asteroid_list(map, center);
    let mut coords: HashMap<Coord, Vec<Coord>> = HashMap::new();
    for asteroid in asteroids {
        if asteroid == Coord::new(0, 0) { continue; }
        let a = asteroid.angle();
        coords.entry(a).or_default().push(asteroid);
    }

    let mut angle_list: Vec<Coord> = coords.keys().cloned().collect();
    angle_list.sort();
    let mut list: Vec<Vec<Coord>> = angle_list.iter().map(|x| coords[x].clone()).collect();

    let mut count = 0;
    loop {
        for candidates in &mut list {
            // let candidates = coords.get_mut(&a).unwrap();
            if candidates.is_empty() { continue; }
            let mut nxt = 0;
            for (i, c) in candidates.iter().enumerate() {
                if c.gcd() < candidates[nxt].gcd() { nxt = i; }
            }
            
            count += 1;
            // println!("Destroyed {} at {:?}", count, candidates[nxt] + center);
            if count == target { return candidates[nxt] + center; }
            candidates.remove(nxt);
        } 
    }
}

// =========== TEST ==============

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(4, 6), 2);
        assert_eq!(gcd(16, 12), 4);
        assert_eq!(gcd(8, 25), 1);
        assert_eq!(gcd(6, 6), 6);
        assert_eq!(gcd(1, 7), 1);
    }

    #[test]
    fn test_task_A() {
        let map = [".#..#", ".....", "#####", "....#", "...##"];
        assert!(task_A(&map) == (8, Coord::new(4, 3)));

        let map = [
            "......#.#.",
            "#..#.#....",
            "..#######.",
            ".#.#.###..",
            ".#..#.....",
            "..#....#.#",
            "#..#....#.",
            ".##.#..###",
            "##...#..#.",
            ".#....####",
        ];
        assert!(task_A(&map) == (33, Coord::new(8, 5)));

        let map = [
            "#.#...#.#.",
            ".###....#.",
            ".#....#...",
            "##.#.#.#.#",
            "....#.#.#.",
            ".##..###.#",
            "..#...##..",
            "..##....##",
            "......#...",
            ".####.###.",
        ];
        assert!(task_A(&map) == (35, Coord::new(2, 1)));

        let map = [
            ".#..#..###",
            "####.###.#",
            "....###.#.",
            "..###.##.#",
            "##.##.#.#.",
            "....###..#",
            "..#.#..#.#",
            "#..#.#.###",
            ".##...##.#",
            ".....#.#..",
        ];
        assert!(task_A(&map) == (41, Coord::new(3, 6)));
    }

    #[test]
    fn test_cmp() {
        let coords: Vec<Coord> = vec![
            Coord { x: 0, y: -2 },
            Coord { x: 1, y: -3 },
            Coord { x: 3, y: -1 },
            Coord { x: 2, y: 0 },
            Coord { x: 3, y: 1 },
            Coord { x: 1, y: 3 },
            Coord { x: 0, y: 2 },
            Coord { x: -1, y: 3},
            Coord { x: -3, y: 1 },
            Coord { x: -2, y: 0 },
            Coord { x: -3, y: -1 },
            Coord { x: -1, y: -3 },
        ];
        for (i, a) in coords.iter().enumerate() {
            for (j, b) in coords.iter().enumerate() {
                assert_eq!(a.cmp(b), i.cmp(&j), "{} {}", i, j);
            }
        }
    }

    #[test]
    fn test_task_B_small() {
        let map = [
            ".#....#####...#..",
            "##...##.#####..##",
            "##...#...#.#####.",
            "..#.....X...###..",
            "..#.#.....#....##",
        ];
        let center = Coord {x: 8, y: 3};
        assert!(task_B(&map, center, 1) == Coord {x: 8, y: 1});
        assert!(task_B(&map, center, 17) == Coord {x: 10, y: 4});
        assert!(task_B(&map, center, 30) == Coord {x: 7, y: 0});
        assert!(task_B(&map, center, 31) == Coord {x: 8, y: 0});
        assert!(task_B(&map, center, 31) == Coord {x: 8, y: 0});
        assert!(task_B(&map, center, 34) == Coord {x: 16, y: 1});
        assert!(task_B(&map, center, 36) == Coord {x: 14, y: 3});
    }

    #[test]
    fn test_task_B_large() {
        let map = [
            ".#..##.###...#######",
            "##.############..##.",
            ".#.######.########.#",
            ".###.#######.####.#.",
            "#####.##.#.##.###.##",
            "..#####..#.#########",
            "####################",
            "#.####....###.#.#.##",
            "##.#################",
            "#####.##.###..####..",
            "..######..##.#######",
            "####.##.####...##..#",
            ".#####..#.######.###",
            "##...#.##########...",
            "#.##########.#######",
            ".####.#.###.###.#.##",
            "....##.##.###..#####",
            ".#.#.###########.###",
            "#.#.#.#####.####.###",
            "###.##.####.##.#..##",
        ];
        let center = Coord {x: 11, y: 13};
        assert!(task_B(&map, center, 1) == Coord {x: 11, y: 12});
        assert!(task_B(&map, center, 2) == Coord {x: 12, y: 1});
        assert!(task_B(&map, center, 3) == Coord {x: 12, y: 2});
        assert!(task_B(&map, center, 10) == Coord {x: 12, y: 8});
        assert!(task_B(&map, center, 20) == Coord {x: 16, y: 0});
        assert!(task_B(&map, center, 50) == Coord {x: 16, y: 9});
        assert!(task_B(&map, center, 100) == Coord {x: 10, y: 16});
        assert!(task_B(&map, center, 199) == Coord {x: 9, y: 6});
        assert!(task_B(&map, center, 200) == Coord {x: 8, y: 2});
        assert!(task_B(&map, center, 201) == Coord {x: 10, y: 9});
        assert!(task_B(&map, center, 299) == Coord {x: 11, y: 1});
    }
}
