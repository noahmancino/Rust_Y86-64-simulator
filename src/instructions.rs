use crate::state::*;
/*
    Argument free instructions. NOP is a valid instruction, but not worth actually writing a
    function for.
 */

// Changes system state to halt. This will stop the system from executing new instructions.
fn halt(mut sys_state: State) {
    sys_state.stat = Status::HLT;
    sys_state.PC += 1;
}

/*
    Binary operator instructions. These instructions compute a binary operation on the data in
    two registers, src and dest, and store the result in dest. These instructions also set
    condition codes.
 */
pub fn op(mut sys_state: State, src: i8, dest: i8, op_code: i8) -> State {
    let src_val = sys_state.registers[src as usize];
    let dest_val = sys_state.registers[dest as usize];
    let val_op = match op_code {
        0 => dest_val.overflowing_add(src_val),
        1 => dest_val.overflowing_sub(src_val),
        2 => (src_val & dest_val, false),
        3 => (src_val ^ dest_val, false),
        _ => {
            sys_state.stat = Status::INS;
            // Garbage value because we need to return something.
            (0, false)
        }
    };
    match sys_state.stat {
        Status::AOK => {
            sys_state.registers[dest as usize] = val_op.0;
            sys_state.OF = val_op.1;
            sys_state.ZF = val_op.0 == 0;
            sys_state.SF = val_op.0 < 0;
        }
        _ => ()
    };
    sys_state.PC += 2;
    sys_state
}

/*
    Jump instructions. These instructions are given a dest (absolute address), and set the program
    counter to the dest if combinations of flags fulfill a condition.
 */

