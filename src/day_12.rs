#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::coords::Coord;

#[derive(Clone)]
#[derive(Eq, PartialEq)]
struct Moon {
    position: Coord,
    velocity: Coord,
}

impl Coord {
    fn energy(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Moon {
    fn still(position: Coord) -> Moon {
        Moon { position, velocity: Coord::new(0, 0, 0) }
    }

    fn energy(&self) -> i32 {
        self.position.energy() * self.velocity.energy()
    }

    fn gravitate(&mut self, other: &Moon) {
        self.velocity.x += (other.position.x - self.position.x).signum();
        self.velocity.y += (other.position.y - self.position.y).signum();
        self.velocity.z += (other.position.z - self.position.z).signum();
    }

    fn fly(&mut self) {
        self.position = self.position + self.velocity;
    }
}

pub fn solve() {
    let io = Moon::still(Coord { x: 9, y: 13, z: -8 });
    let europa = Moon::still(Coord { x: -3, y: 16, z: -17 });
    let ganymede = Moon::still(Coord { x: -4, y: 11, z: -10 });
    let callisto = Moon::still(Coord { x: 0, y: -2, z: -2 });
    let moons = vec![io, europa, ganymede, callisto];

    println!("Task A: {}", task_A(moons.clone(), 1000));
    println!("Task B: {}", task_B(moons));
}

fn task_A(mut moons: Vec<Moon>, steps: i32) -> i32 {
    let n = moons.len();
    for _ in 0..steps {
        for i in 0..n {
            for j in i+1..n {
                let (left, right) = moons.split_at_mut(j);
                left[i].gravitate(&right[0]);
                right[0].gravitate(&left[i]);
            }

            moons[i].fly();
        }
    }
    moons.iter().map(|x| x.energy()).sum()
}

fn cycle(positions_init: Vec<i32>, velocities_init: Vec<i32>) -> i64 {
    let mut positions = positions_init.clone();
    let mut velocities = velocities_init.clone();
    let n = positions.len();
    assert_eq!(velocities.len(), n);

    let mut count: i64 = 0;
    loop {
        for i in 0..n {
            for j in i+1..n {
                velocities[i] += (positions[j] - positions[i]).signum();
                velocities[j] += (positions[i] - positions[j]).signum();
            }

            positions[i] += velocities[i];
        }

        count += 1;
        if positions == positions_init && velocities == velocities_init {
            break count
        }
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 { b } else { gcd(b % a, a) }
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn task_B(moons: Vec<Moon>) -> i64 {
    let cycle_x = cycle(
        moons.iter().map(|p| p.position.x).collect(),
        moons.iter().map(|p| p.velocity.x).collect(),
    );
    let cycle_y = cycle(
        moons.iter().map(|p| p.position.y).collect(),
        moons.iter().map(|p| p.velocity.y).collect(),
    );
    let cycle_z = cycle(
        moons.iter().map(|p| p.position.z).collect(),
        moons.iter().map(|p| p.velocity.z).collect(),
    );

    lcm(lcm(cycle_x, cycle_y), cycle_z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy() {
        let moons = vec![
            Moon::still(Coord{ x: -1, y: 0, z: 2 }),
            Moon::still(Coord{ x: 2, y: -10, z: -7 }),
            Moon::still(Coord{ x: 4, y: -8, z: 8 }),
            Moon::still(Coord{ x: 3, y: 5, z: -1 }),
        ];
        assert_eq!(task_A(moons.clone(), 10), 179);
        assert_eq!(task_B(moons.clone()), 2772);

        let moons = vec![
            Moon::still(Coord{ x: -8, y: -10, z: 0}),
            Moon::still(Coord{ x: 5, y: 5, z: 10 }),
            Moon::still(Coord{ x: 2, y: -7, z: 3 }),
            Moon::still(Coord{ x: 9, y: -8, z: -3 }),
        ];
        assert_eq!(task_A(moons.clone(), 100), 1940);
        assert_eq!(task_B(moons.clone()), 4686774924);
    }
}
