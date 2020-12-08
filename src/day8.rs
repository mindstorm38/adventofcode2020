
mod interpret;
use interpret::{Instruction, Interpreter, ExitCause};

const INPUT: &str = include_str!("res/day8");

fn main() {

    let program = interpret::parse(INPUT.lines().map(String::from));
    let mut interpreter = Interpreter::new();

    interpreter.load_program(&program);
    interpreter.run();
    println!("Acc: {}", interpreter.get_accumulator());

    let mut fix_index = 0usize;

    loop {

        interpreter.load_program(&program);

        if let Some(inst) = interpreter.get_instruction_mut(fix_index) {
            use Instruction::*;
            match *inst {
                Jmp(val) => *inst = Nop(val),
                Nop(val) => *inst = Jmp(val),
                _ => {}
            }
        } else {
            panic!("Can no longer switch instruction.");
        }

        fix_index += 1;

        if let Some(ExitCause::EndOfProgram) = interpreter.run() {
            break;
        }

    }

    println!("End of Program, switched the instruction at {}, accumulator: {}", fix_index, interpreter.get_accumulator());

}