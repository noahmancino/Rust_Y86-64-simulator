use crate::state::*;
use crate::instructions::*;

pub fn op_test(mut sys_state: State) -> State {
    let data: Vec<i8> = sys_state.read_mem(0);
    let op_code = data[0];
    sys_state = op(sys_state, 1, 0, op_code);
    return sys_state
}