use std::io::Read;

#[derive(Clone)]
#[derive(PartialEq)]
pub enum MachineState {
    Waiting,
    Halt,
}

#[derive(Clone)]
pub struct Intcode {
    pub code: Vec<i32>,
    pub input: Vec<i32>,
    pub output: Vec<i32>,
    pointer: usize,
    relative_base: i32,
}

impl Intcode {
    pub fn parse_string(data: &str) -> Vec<i32> {
        data.split(",").map(|x| x.trim().parse().unwrap()).collect()
    }

    pub fn parse_file(filename: &str) -> Vec<i32> {
        let mut file = std::fs::File::open(filename).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        Intcode::parse_string(&data)
    }

    pub fn blackbox(code: Vec<i32>) -> Intcode {
        Intcode {
            code,
            input: Vec::new(),
            output: Vec::new(),
            pointer: 0,
            relative_base: 0,
        }
    }

    pub fn machine(code: Vec<i32>, input: Vec<i32>) -> Intcode {
        Intcode { code, input, output: Vec::new(), pointer: 0, relative_base: 0 }
    }

    fn rvalue(&self, offset: usize) -> i32 {
        let inst = self.code[self.pointer];
        let mode = (inst / 10_i32.pow((offset + 1) as u32)) % 10;
        let op = self.code[self.pointer + offset];
        let addr = match mode {
            0 => op,
            1 => return op,
            2 => op + self.relative_base,
            _ => panic!("wrong mode: {}", mode)
        } as usize;
        if addr >= self.code.len() { 0 } else { self.code[addr] }
    }

    fn lvalue(&mut self, offset: usize) -> &mut i32 {
        let inst = self.code[self.pointer];
        let mode = (inst / 10_i32.pow((offset + 1) as u32)) % 10;
        let op = self.code[self.pointer + offset];
        let addr = match mode {
            0 => op,
            2 => op + self.relative_base,
            _ => panic!("wrong mode: {}", mode)
        } as usize;
        if addr >= self.code.len() {
            self.code.resize(addr + 1, 0);
        }
        &mut self.code[addr]
    }

    pub fn run(&mut self) -> MachineState {
        loop {
            let inst = self.code[self.pointer];
            let opcode = inst % 100;
            match opcode {
                99 => {
                    return MachineState::Halt;
                }
                1 => {   // addition
                    let op1 = self.rvalue(1);
                    let op2 = self.rvalue(2);
                    let dst = self.lvalue(3);
                    *dst = op1 + op2;
                    self.pointer += 4;
                }
                2 => {   // multiplication
                    let op1 = self.rvalue(1);
                    let op2 = self.rvalue(2);
                    let dst = self.lvalue(3);
                    *dst = op1 * op2;
                    self.pointer += 4;
                }
                3 => {   // input
                    if self.input.is_empty() {
                        return MachineState::Waiting;
                    }
                    let inp = self.input.remove(0);
                    let dst = self.lvalue(1);
                    *dst = inp;
                    self.pointer += 2;
                }
                4 => {   // output
                    let op1 = self.rvalue(1);
                    self.output.push(op1);
                    self.pointer += 2;
                }
                5 => {   // jump if true
                    let op1 = self.rvalue(1);
                    let op2 = self.rvalue(2) as usize;
                    if op1 != 0 { self.pointer = op2; }
                    else        { self.pointer += 3; }
                }
                6 => {   // jump if false
                    let op1 = self.rvalue(1);
                    let op2 = self.rvalue(2) as usize;
                    if op1 == 0 { self.pointer = op2; }
                    else        { self.pointer += 3; }
                }
                7 => {   // less than
                    let op1 = self.rvalue(1);
                    let op2 = self.rvalue(2);
                    let dst = self.lvalue(3);
                    *dst = if op1 < op2 { 1 } else { 0 };
                    self.pointer += 4;
                }
                8 => {   // equals
                    let op1 = self.rvalue(1);
                    let op2 = self.rvalue(2);
                    let dst = self.lvalue(3);
                    *dst = if op1 == op2 { 1 } else { 0 };
                    self.pointer += 4;
                }
                9 => {  // adjust relative base
                    let op1 = self.rvalue(1);
                    self.relative_base += op1;
                    self.pointer += 2;
                }
                _ => panic!("wrong opcode: {}", opcode),
            }
        }
    }    
}

//  ================ TESTS ==================

#[cfg(test)]
mod tests
{
    use super::*;

    fn compare_codes(initial: &[i32], expected: &[i32]) {
        let mut intcode = Intcode::blackbox(initial.to_owned());
        intcode.run();
        assert!(&intcode.code[..] == expected);
    }

    #[test]
    fn test_day2() {
        compare_codes(&[1,0,0,0,99], &[2,0,0,0,99]);
        compare_codes(&[2,3,0,3,99], &[2,3,0,6,99]);
        compare_codes(&[2,4,4,5,99,0], &[2,4,4,5,99,9801]);
        compare_codes(&[1,1,1,4,99,5,6,0,99], &[30,1,1,4,2,5,6,0,99]);
    }

    fn compare_output(code: &[i32], input: &[i32], expected: &[i32]) {
        let mut intcode = Intcode::machine(code.to_owned(), input.to_owned());
        intcode.run();
        assert!(&intcode.output[..] == expected);
    }

    #[test]
    fn test_day5() {
        // position mode, equals 8
        compare_output(&[3,9,8,9,10,9,4,9,99,-1,8], &[8], &[1]);
        compare_output(&[3,9,8,9,10,9,4,9,99,-1,8], &[7], &[0]);

        // position mode, less than 8
        compare_output(&[3,9,7,9,10,9,4,9,99,-1,8], &[8], &[0]);
        compare_output(&[3,9,7,9,10,9,4,9,99,-1,8], &[7], &[1]);

        // immediate mode, equals 8
        compare_output(&[3,3,1108,-1,8,3,4,3,99], &[8], &[1]);
        compare_output(&[3,3,1108,-1,8,3,4,3,99], &[7], &[0]);

        // immediate mode, less than 8
        compare_output(&[3,3,1107,-1,8,3,4,3,99], &[8], &[0]);
        compare_output(&[3,3,1107,-1,8,3,4,3,99], &[7], &[1]);

        // position mode, jump test
        compare_output(&[3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], &[0], &[0]);
        compare_output(&[3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], &[42], &[1]);

        // immediate mode, jump test
        compare_output(&[3,3,1105,-1,9,1101,0,0,12,4,12,99,1], &[0], &[0]);
        compare_output(&[3,3,1105,-1,9,1101,0,0,12,4,12,99,1], &[42], &[1]);

        // large test: x < 8 => 999, x == 8 => 1000, x > 8 => 1001
        compare_output(
            &[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
              1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
              999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 
            &[4],
            &[999],
        );
        compare_output(
            &[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
              1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
              999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 
            &[8],
            &[1000],
        );
        compare_output(
            &[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
              1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
              999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 
            &[42],
            &[1001],
        );
    }

    #[test]
    fn test_quine_day_9() {
        compare_output(&[109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99],
                       &[],
                       &[109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]);
    }
}
