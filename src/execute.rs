use crate::state::*;
use crate::instructions::*;

pub fn instruction_cycle(mut sys_state: State) {
    sys_state.read_mem(sys_state.program_counter);
    let instruction_specifier = sys_state.mem_bus[0];
    let instruction_type = instruction_specifier & 0xf0;
    let instruction_function = instruction_specifier & 0xf;
    match instruction_type {
        0 =>
    }
}

fn movq_family(mut sys_state: State, op_fn: i8) {
    match op_fn {

    }
}