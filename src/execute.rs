use crate::state::*;
use crate::instructions::*;

pub fn op_test(mut sys_state: State) -> State {
    sys_state.read_mem(0);
    let data = sys_state.mem_bus;
    let op_code = (data[0] & 0xf);
    sys_state = op(sys_state, 1, 0, op_code);

    return sys_state
}