
#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
    Exit
}

#[derive(Debug)]
pub enum ExitCause {
    Instruction,
    EndOfProgram
}

pub struct Interpreter {
    prg: Vec<Instruction>,
    acc: i64,
    ptr: usize
}

pub fn parse(inp: impl Iterator<Item=String>) -> Vec<Instruction> {
    inp.map(|line| {
        use Instruction::*;
        let (code, operand) = line.split_at(3);
        let operand = &operand[1..];
        match code {
            "acc" => Acc(operand.parse().unwrap()),
            "jmp" => Jmp(operand.parse().unwrap()),
            "nop" => Nop(operand.parse().unwrap()),
            _ => Exit
        }
    }).collect()
}

impl Interpreter {

    pub fn new() -> Interpreter {
        Interpreter {
            prg: Vec::new(),
            acc: 0,
            ptr: 0
        }
    }

    pub fn load_program(&mut self, program: &Vec<Instruction>) {
        self.prg.clear();
        self.prg.extend((*program).clone().into_iter());
        self.acc = 0;
        self.ptr = 0;
    }

    pub fn get_instruction_mut(&mut self, ptr: usize) -> Option<&mut Instruction> {
        self.prg.get_mut(ptr)
    }

    pub fn get_accumulator(&self) -> i64 {
        self.acc
    }

    pub fn get_pointer(&self) -> usize {
        self.ptr
    }

    pub fn run_step(&mut self) -> Option<ExitCause> {

        use Instruction::*;
        if let Some(inst) = self.prg.get(self.ptr) {

            let mut next_ptr = self.ptr + 1;

            // println!("[{}] {:?}", self.ptr, inst);

            match inst {
                Acc(val) => self.acc += *val as i64,
                Jmp(off) => next_ptr = (self.ptr as isize + *off as isize) as usize,
                Exit => return Some(ExitCause::Instruction),
                _ => {}
            }

            self.prg[self.ptr] = Exit; // Temporary to avoid infinite loops
            self.ptr = next_ptr;

            None

        } else {
            Some(ExitCause::EndOfProgram)
        }

    }

    pub fn run(&mut self) -> Option<ExitCause> {
        loop {
            if let Some(cause) = self.run_step() {
                return Some(cause);
            }
        }
    }

}