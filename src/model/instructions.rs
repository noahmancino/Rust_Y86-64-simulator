mod state;
/*
    Argument free instructions. NOP is a valid instruction, but not worth actually writing a
    function for.
 */

// Changes system state to halt. This will stop the system from executing new instructions.
fn halt(mut sys_state: state::State) {
    sys_state.stat = state::Status::HLT;
}

/*
    Binary operator instructions. These instructions compute a binary operation on the data in
    two registers, src and dest, and store the result in dest. These instructions also set
    condition codes.
 */
fn op(mut sys_state: state::State, src: i32, dest: i32) {
    sys_state.registers[dest] = sys_state.registers[src] - sys_state.registers[dest];
}

/*
    Jump instructions. These instructions are given a dest (absolute address), and set the program
    counter to the dest if combinations of flags fulfill a condition.
 */

