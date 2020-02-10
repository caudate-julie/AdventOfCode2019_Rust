#![allow(non_snake_case)]
#![allow(dead_code)]

pub fn solve() {
    let lo = 171309;
    let hi = 643603;

    let (a, b) = task_AB(lo, hi);
    println!("Task A: {}", a);
    println!("Task B: {}", b);
}

fn to_digits(mut a: i32) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    while a > 0 {
        v.push(a % 10);
        a /= 10;
    }
    v.reverse();
    v
}

fn non_decreasing(v: &[i32]) -> bool {
    for i in 1..v.len() {
        if v[i - 1] > v[i] { return false; }
    }
    true
}

fn dubs(v: &[i32]) -> bool {
    for i in 1..v.len() {
        if v[i - 1] == v[i] { return true; }
    }
    false
}

fn exact_dubs(v: &[i32]) -> bool {
    for i in 1..v.len() {
        if v[i - 1] == v[i] { 
            let left = i == 1 || v[i - 2] != v[i];
            let right = i == v.len() - 1 || v[i + 1] != v[i];
            if left && right {
                return true;
            }
        }
    }
    false
}


fn task_AB(lo: i32, hi: i32) -> (i32, i32) {
    let mut countA = 0;
    let mut countB = 0;
    for i in lo..hi+1 {
        let v = to_digits(i);
        assert!(v.len() == 6);
        if non_decreasing(&v) && dubs(&v) { countA += 1; }
        if non_decreasing(&v) && exact_dubs(&v) { countB += 1; }
    }
    (countA, countB)
}

// ============== TESTS ======================

#[test]
fn test_non_decreasing() {
    assert!(non_decreasing(&to_digits(123455)));
    assert!(!non_decreasing(&to_digits(123454)));
}

#[test]
fn test_dubs() {
    assert!(dubs(&to_digits(111111)));
    assert!(dubs(&to_digits(113355)));
    assert!(!dubs(&to_digits(121212)));
}

#[test]
fn test_exact_dubs() {
    assert!(exact_dubs(&to_digits(112233)));
    assert!(!exact_dubs(&to_digits(121212)));
    assert!(!exact_dubs(&to_digits(111444)));
    assert!(exact_dubs(&to_digits(111144)));
}
