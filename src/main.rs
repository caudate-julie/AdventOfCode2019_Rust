#![allow(non_snake_case)]

mod day_1;

fn main() {
    let xs = day_1::read_data();

    // dbg!(&xs);
    println!("Task A: {}", day_1::task_A(&xs));
    println!("Task B: {}", day_1::task_B(&xs));
}
