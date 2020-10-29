use crate::state::*;

fn extract_byte(byte: i8, val: i64) -> i8 {
    let mask = 0xff;
    let extracted = (val >> (8 * byte)) & mask;
    extracted as i8
}

fn combine_bytes(sys_state: &State) -> i64 {
    let mut combined = 0;
    for x in 0..7 {
        combined = combined << 8;
        combined |= sys_state.mem_bus[x] as i64
    }
    combined
}

/*
    Argument free instructions. NOP is a valid instruction, but not worth actually writing a
    function for.
 */

// Changes system state to halt. This will stop the system from executing new instructions.
pub fn halt(mut sys_state: State) -> State {
    sys_state.status = Status::HLT;
    sys_state
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
            sys_state.status = Status::INS;
            // Garbage value because we need to return something.
            (0, false)
        }
    };
    match sys_state.status {
        Status::AOK => {
            sys_state.registers[dest as usize] = val_op.0;
            sys_state.overflow_flag = val_op.1;
            sys_state.zero_flag = val_op.0 == 0;
            sys_state.sign_flag = val_op.0 < 0;
        }
        _ => ()
    };
    sys_state.program_counter += 2;
    sys_state
}

/*
    Jump instructions. These instructions will jump to an address or do nothing depending on the
    state of the system's condition codes.
 */
pub fn jxx(mut sys_state: State, dest: usize, op_code: i8) -> State {
    let will_branch = match op_code {
        // jmp
        0 => true,
        // jle
        1 => sys_state.zero_flag || (sys_state.sign_flag ^ sys_state.overflow_flag),
        // jl
        2 => sys_state.sign_flag ^ sys_state.overflow_flag,
        // je
        3 => sys_state.zero_flag,
        // jne
        4 => !sys_state.zero_flag,
        // jge
        5 => sys_state.zero_flag || !(sys_state.sign_flag ^ sys_state.overflow_flag),
        // jg
        6 => !(sys_state.sign_flag ^ sys_state.overflow_flag),
        //
        _ => {
            sys_state.status = Status::INS;
            // 'false' is just garbage.
            false
        }
    };

    match sys_state.status {
        Status::AOK => {
            if will_branch {
                sys_state.program_counter = dest;
            }
            else {
                sys_state.program_counter += 9;
            }
        }
        _ => ()
    }
    sys_state
}

/*
    Moves an immediate value to a register.
 */
pub fn irmovq(mut sys_state: State, immediate: i64, dest: i8) -> State {
    sys_state.registers[dest as usize] = immediate;
    sys_state.program_counter += 10;
    sys_state
}

/*
    Moves a value from register to memory.
 */

pub fn rrmovq(mut sys_state: State, src: i8, dest: i8) {
    sys_state.registers[dest] = sys_state.registers[src];
    sys_state.program_counter += 2;
}

pub fn rmmovq(mut sys_state: State, src: i8, dest_reg: i8) -> State {
    let dest = sys_state.registers[dest_reg] as usize;
    let mut byte: i8;
    for x in 7..0 {
        byte = extract_byte(x, sys_state.registers[src as usize]);
        sys_state.write_mem(dest, byte)
    }
    sys_state.program_counter += 10;
    sys_state
}

pub fn mrmovq(mut sys_state: State, src_reg: i8, dest: i8) -> State {
    let src = sys_state.registers[src_reg] as usize;
    sys_state.read_mem(src);
    sys_state.registers[dest as usize] = combine_bytes(&sys_state);
    sys_state.program_counter += 10;
    sys_state
}

pub fn cmovxx(mut sys_state: State, src: i8, dest: i8, op_code:i8) -> State {
    let will_move = match op_code {
        1 => sys_state.zero_flag || (sys_state.sign_flag ^ sys_state.overflow_flag),
        2 => (sys_state.sign_flag ^ sys_state.overflow_flag),
        3 => sys_state.zero_flag,
        4 => !(sys_state.zero_flag),
        5 => sys_state.zero_flag || !(sys_state.sign_flag ^ sys_state.overflow_flag),
        6 => !(sys_state.sign_flag ^ sys_state.overflow_flag),
        _ => {
            sys_state.status = Status::INS;
            false
        }
    };

    match sys_state.status {
        Status::AOK => {
            if will_move {
                sys_state.registers[dest as usize] = sys_state.registers[src as usize];
            }
            sys_state.program_counter += 2;
        }
        _ => ()
    };

    sys_state
}

pub fn pushq(mut sys_state: State, src: i8) -> State {
    sys_state.registers[4] -= 8;
    // register 4 is %rsp
    let address = sys_state.registers[4] as usize;
    let mut write: i8;
    for x in 7..0 {
        write = extract_byte(x, sys_state.registers[src as usize]);
        // Now I'm starting to regret my choice of endianess...
        sys_state.write_mem(address + 7 - (x as usize), write)
    }
    sys_state.program_counter += 2;
    sys_state
}

pub fn popq(mut sys_state: State, dest: i8) -> State {
    sys_state.read_mem(sys_state.registers[4] as usize);
    let popped = combine_bytes(&sys_state);
    sys_state.registers[dest as usize] = popped;
    sys_state.registers[4] += 8;
    sys_state.program_counter += 2;
    sys_state
}

pub fn call(mut sys_state: State, dest: usize) -> State {
    sys_state.registers[4] -= 8;
    let address = sys_state.registers[4] as usize;
    let mut write: i8;
    for x in 7..0 {
        write = extract_byte(x, sys_state.program_counter as i64);
        sys_state.write_mem(address + 7 - (x as usize), write);

    }
    sys_state.program_counter = dest;
    sys_state
}

pub fn ret(mut sys_state: State) -> State {
    sys_state.read_mem(sys_state.registers[4] as usize);
    let address = combine_bytes(&sys_state) as usize;
    sys_state.registers[4] += 8;
    sys_state.program_counter = address;
    sys_state
}