use crate::instructions::combine_bytes;
mod state;
mod instructions;
mod execute;
mod assemble;
use std::env::*;

// Fetches, decodes, and executes the next instruction in memory.
/*
fn execute_next(mut sys_state: state::State) {

}
*/



/*
    Initializes default system state, loads the program into memory and executes a single
    instructions.
 */

fn main() {
    for lines in assemble::tokenize("./example") {
        for line in lines {
            println!("token: {}", line);
        }
    }
    return;
    let mut sys_state: state::State  = state::State::new();
    sys_state.registers[0] = 66;
    sys_state.registers[1] = 1;
    sys_state.registers[4] = 200;
    sys_state.write_mem(2, 0x61);
    sys_state.write_mem(3, 0x10);
    sys_state.write_mem(0, 0xa0);
    sys_state.write_mem(1, 0x00);
    sys_state.write_mem(4, 0x74);
    sys_state.write_mem(12, 2);
    sys_state.write_mem(13, 0xb0);
    for x in 0..150 {
        println!("x: {}", x);
        println!("register[0]: {}", sys_state.registers[0]);
        match sys_state.status {
            state::Status::AOK => {
                if x % 10 == 0 {
                    sys_state.read_mem(sys_state.registers[4] as usize);
                    println!("stack pointer {}", sys_state.registers[4]);
                    println!("combined bytes {}\n\n\n", combine_bytes(&sys_state))
                }
                sys_state = execute::instruction_cycle(sys_state);
                true
            }
            _ => {
                println!("hi!");
                false
            }

        };
    }
}
