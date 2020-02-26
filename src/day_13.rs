#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(deprecated)]    // external deprecations in crossterm crate

use std::{thread, time};
use std::io::Write;

use crate::intcode::Intcode;
use crate::intcode::MachineState;
use crate::intcode::long;

use crossterm::{
    execute, queue, terminal, cursor, style::Color, style
};

const EMPTY: long = 0;
const WALL: long = 1;
const BLOCK: long = 2;
const PADDLE: long = 3;
const BALL: long = 4;


pub fn solve() {
    let mut code = Intcode::parse_file("inputs/day_13.txt");
    println!("Task A: {}", task_A(&code));
    code[0] = 2;
    task_B(&code);
}


struct Game {
    grid: Vec<Vec<long>>,
    ball_x: long,
    paddle_x: long,
    score: long,
    stdout: std::io::Stdout,
}

impl Game {
    fn new(width: usize, height: usize) -> Game {
        Game {
            grid: vec![vec![0; width]; height],
            ball_x: 0,
            paddle_x: 0,
            score: 0,
            stdout: std::io::stdout(),
        }
    }

    fn set(&mut self, i: long, j: long, value: long) {
        self.grid[i as usize][j as usize] = value;
    }

    fn show(&mut self) {
        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                let x = match self.grid[i][j] {
                    EMPTY  => style::style('.').with(Color::DarkGrey),
                    WALL   => style::style('#').with(Color::White),
                    BLOCK  => style::style('%').with(Color::Yellow),
                    PADDLE => style::style('=').with(Color::Blue),
                    BALL   => style::style('o').with(Color::Red),
                    _ => panic!("...at ({}, {})", i, j),
                };

                queue!(self.stdout,
                       cursor::MoveTo(2 * j as u16, i as u16),
                       style::PrintStyledContent(x)).unwrap();
            }
        }
        queue!(self.stdout, 
               cursor::MoveTo(0, self.grid.len() as u16 + 1),
               style::Print(self.score)).unwrap();
        self.stdout.flush().unwrap();
    }


    fn update(&mut self, output: &[long]) {
        assert!(output.len() % 3 == 0);
    
        let mut pointer: usize = 0;
        while pointer < output.len() {
            let x = output[pointer];
            let y = output[pointer + 1];
            let value = output[pointer + 2];
            pointer += 3;
    
            if x == -1 {
                assert_eq!(y, 0);
                self.score = value;
                continue;
            }

            match value {
                PADDLE => { self.paddle_x = x; }
                BALL   => { self.ball_x = x; }
                _ => {}
            }
    
            self.set(y, x, value);
        }
    }
}


fn task_A(code: &[long]) -> long {
    let mut intcode = Intcode::blackbox(code.to_vec());
    let mut game = Game::new(37, 24);
    
    intcode.run();
    game.update(&intcode.output);
    
    let mut count: long = 0;

    for i in 0..game.grid.len() {
        for j in 0..game.grid[i].len() {
            if game.grid[i][j] == BLOCK { count += 1; }
        }
    }
    game.show();
    count
}


fn task_B(code: &[long]) {
    let mut intcode = Intcode::blackbox(code.to_vec());
    let mut game = Game::new(37, 24);

    execute!(game.stdout, terminal::Clear(terminal::ClearType::All), cursor::Hide).unwrap();
        
    loop {
        let state = intcode.run();
        game.update(&intcode.output);
        intcode.input.push((game.ball_x - game.paddle_x).signum());
        thread::sleep(time::Duration::from_millis(20));
        game.show();

        if state == MachineState::Halt { break }
    }
}