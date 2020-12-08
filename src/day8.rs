
mod interpret;

use std::time::Instant;
use interpret::{Instruction, Interpreter, ExitCause};

const INPUT: &str = include_str!("res/day8");

fn main() {

    let whole_now = Instant::now();

    let now = Instant::now();
    let program = interpret::parse(INPUT.lines().map(String::from));
    println!("Parsing duration: {}ns", now.elapsed().as_secs_f64());

    let now = Instant::now();
    let mut interpreter = Interpreter::new();
    println!("New duration: {}ns", now.elapsed().as_nanos());

    let now = Instant::now();
    interpreter.load_program(&program);
    interpreter.run();
    println!("Run: {}ns", now.elapsed().as_nanos());
    println!("Acc: {}", interpreter.get_accumulator());

    let now = Instant::now();
    let mut fix_index = 0usize;

    loop {

        interpreter.load_program(&program);

        loop {

            if let Some(inst) = interpreter.get_instruction_mut(fix_index) {

                fix_index += 1;

                use Instruction::*;
                match *inst {
                    Jmp(val) => {
                        *inst = Nop(val);
                        break;
                    },
                    Nop(val) => {
                        *inst = Jmp(val);
                        break;
                    },
                    _ => {}
                }

            } else {
                panic!("Can no longer swap instruction.");
            }

        }

        if let Some(ExitCause::EndOfProgram) = interpreter.run() {
            break;
        }

    }

    println!("Bug resolution: {}ns", now.elapsed().as_nanos());
    println!("End of Program, switched the instruction at {}, accumulator: {}", fix_index, interpreter.get_accumulator());
    println!("Whole run: {}ns", whole_now.elapsed().as_nanos());

}