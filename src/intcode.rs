#[derive(Clone)]
pub struct Intcode {
    pub code: Vec<i32>,
    pub input: Vec<i32>,
    pub output: Vec<i32>,
    pointer: usize,
}

impl Intcode {
    pub fn blackbox(code: Vec<i32>) -> Intcode {
        Intcode {
            code,
            input: Vec::new(),
            output: Vec::new(),
            pointer: 0,
        }
    }

    pub fn machine(code: Vec<i32>, input: Vec<i32>, output: Vec<i32>) -> Intcode {
        Intcode { code, input, output, pointer: 0 }
    }

    fn operand(&self, offset: usize, operand: i32) -> i32 {
        let mode = (operand / 10_i32.pow((offset + 1) as u32)) % 10;
        let op = self.code[self.pointer + offset];
        match mode {
            0 => self.code[op as usize],
            1 => op,
            _ => panic!("wrong mode: {}", mode)
        }
    }

    pub fn run(&mut self) {
        loop {
            let inst = self.code[self.pointer];
            let opcode = inst % 100;
            match opcode {
                99 => return,
                1 => {   // addition
                    let op1 = self.operand(1, inst);
                    let op2 = self.operand(2, inst);
                    let res = self.code[self.pointer + 3] as usize;
                    self.code[res] = op1 + op2;
                    self.pointer += 4;
                }
                2 => {   // multiplication
                    let op1 = self.operand(1, inst);
                    let op2 = self.operand(2, inst);
                    let res = self.code[self.pointer + 3] as usize;
                    self.code[res] = op1 * op2;
                    self.pointer += 4;
                }
                _ => panic!("wrong opcode: {}", opcode),
            }
        }
    }    
}
