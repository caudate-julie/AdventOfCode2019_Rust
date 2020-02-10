

pub fn run(intcode: &mut Vec<i32>, mut pointer: usize) {
    loop {
        let opcode = intcode[pointer];
        match opcode {
            99 => return,
            1 => {
                let op1 = intcode[pointer + 1] as usize;
                let op2 = intcode[pointer + 2] as usize;
                let res = intcode[pointer + 3] as usize;
                intcode[res] = intcode[op1] + intcode[op2];
                pointer += 4;
            }
            2 => {
                let op1 = intcode[pointer + 1] as usize;
                let op2 = intcode[pointer + 2] as usize;
                let res = intcode[pointer + 3] as usize;
                intcode[res] = intcode[op1] * intcode[op2];
                pointer += 4;
            }
            _ => panic!("{}", opcode),
        }
    }
}
