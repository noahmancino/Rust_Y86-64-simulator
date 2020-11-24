use crate::state::*;
use crate::instructions::*;

// TODO: separate some concerns.
pub fn instruction_cycle(mut sys_state: State) -> State {
    sys_state.read_mem(sys_state.program_counter);
    let instruction_type = (sys_state.mem_bus[0] as u8 & 0xf0) >> 4;
    let instruction_function = sys_state.mem_bus[0] as u8 & 0xf;
    println!("program counter: {}", sys_state.program_counter);
    println!("instruction type: {}", instruction_type);
    println!("instruction function: {}", instruction_function);
    sys_state = match instruction_type {
        0 => halt(sys_state),
        1 => {
            sys_state.program_counter += 1;
            sys_state
        }
        2 => {
            let reg_a = (sys_state.mem_bus[1] as u8 & 0xf0) >> 4;
            let reg_b = sys_state.mem_bus[1] as u8 & 0xf;
            rrmovq(sys_state, reg_a, reg_b)
        }
        3 => {
            let reg_b = sys_state.mem_bus[1] as u8 & 0xf;
            sys_state.read_mem(sys_state.program_counter + 2);
            let immediate = combine_bytes(&sys_state);
            irmovq(sys_state, immediate, reg_b)

        }
        4 => {
            let reg_a = (sys_state.mem_bus[1] as u8 & 0xf0) >> 4;
            let reg_b = sys_state.mem_bus[1] as u8 & 0xf;
            sys_state.read_mem(sys_state.program_counter + 2);
            let displacement = combine_bytes(&sys_state) as usize;
            rmmovq(sys_state, reg_a, reg_b, displacement)
        }
        5 => {
            let reg_a = (sys_state.mem_bus[1] as u8 & 0xf0) >> 4;
            let reg_b = sys_state.mem_bus[1] as u8 & 0xf;
            sys_state.read_mem(sys_state.program_counter + 2);
            let displacement = combine_bytes(&sys_state) as usize;
            mrmovq(sys_state, reg_a, reg_b, displacement)
        }
        6 => {
            let reg_a = (sys_state.mem_bus[1] as u8 & 0xf0) >> 4;
            let reg_b = sys_state.mem_bus[1] as u8 & 0xf;
            op(sys_state, reg_a, reg_b, instruction_function)
        }
        7 => {
            sys_state.read_mem(sys_state.program_counter + 1);
            let destination = combine_bytes(&sys_state) as usize;
            //println!("destintation {}", destination);
            jxx(sys_state, destination, instruction_function)
        }
        8 => {
            sys_state.read_mem(sys_state.program_counter + 1);
            let destination = combine_bytes(&sys_state) as usize;
            call(sys_state, destination)

        }
        9 => ret(sys_state),
        10 => {
            let reg_a = (sys_state.mem_bus[1] as u8 & 0xf0) >> 4;
            pushq(sys_state, reg_a)
        }
        11 => {
            let reg_a = (sys_state.mem_bus[1] as u8 & 0xf0) >> 4;
            popq(sys_state, reg_a)
        }
        _ => {
            sys_state.status = Status::INS;
            sys_state
        }
    };
    sys_state
}

