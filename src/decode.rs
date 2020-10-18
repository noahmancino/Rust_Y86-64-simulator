use crate::state::*;
use crate::instructions::*;

pub fn op_test(mut sys_state: State) {
    let data: Vec<i8> = sys_state.read_mem(0);
    let op_code = data[0];
    op(sys_state, 1, 0, op_code);
}