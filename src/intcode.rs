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

//  ================ TESTS ==================

#[cfg(test)]
mod tests
{
    use super::*;

    fn compare_codes(initial: &[i32], expected: &[i32]) {
        let mut intcode = Intcode::blackbox(initial.to_owned());
        intcode.run();
        assert!(&intcode.code[..] == expected)
    }

    #[test]
    fn test_day2() {
        compare_codes(&[1,0,0,0,99], &[2,0,0,0,99]);
        compare_codes(&[2,3,0,3,99], &[2,3,0,6,99]);
        compare_codes(&[2,4,4,5,99,0], &[2,4,4,5,99,9801]);
        compare_codes(&[1,1,1,4,99,5,6,0,99], &[30,1,1,4,2,5,6,0,99]);
    }
}
