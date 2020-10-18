mod state;
mod instructions;
mod decode;
mod assemble;


// Fetches, decodes, and executes the next instruction in memory.
/*
fn execute_next(mut sys_state: state::State) {

}
*/


// Sets up default memory state.
/*
fn initial_state(mut sys_state: state::State) {
}
*/

/*
    Initializes default system state, loads the program into memory and executes a single
    instructions.
 */

fn main() {
    let mut sys_state: state::State  = state::State::new();
    sys_state.registers[0] = 10;
    sys_state.registers[1] = 15;
    sys_state = decode::op_test(sys_state);
    println!("Register 0: {}", sys_state.registers[0]);
}
