mod state;
mod instructions;

// Fetches, decodes, and executes the next instruction in memory.
fn execute_next(mut sys_state: state::State) {

}

// Sets up default memory state.
fn initial_state(mut sys_state: state::State) {

}
/*
    Initializes default system state, loads the program into memory and executes a single
    instruction whenever the controller tells it to. Passes the system state to the controller at
    when the program is loaded into memory and at each step thereafter.
 */

fn main() {
    println!("Hello, world!");
}
