mod state;
mod instructions;
mod execute;
mod assemble;


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
    let mut sys_state: state::State  = state::State::new();
    sys_state.registers[0] = 66;
    sys_state.registers[1] = 1;
    sys_state.write_mem(0, 0x1);
    sys_state = execute::op_test(sys_state);
    sys_state = instructions::halt(sys_state);
    println!("Register 0: {}", sys_state.registers[0]);
}
