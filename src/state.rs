/*
    This module defines the structure of the systems memory state.
 */

// "Processor" status code
#[allow(dead_code)]
pub enum Status {
    // Default status
    AOK,
    // Status after halt instruction
    HLT,
    // Status after attempting to access invalid memory
    ADR,
    // Invalid instruction.
    INS
}

#[allow(dead_code)]
pub struct State {
    /*
        Main memory is private as a nod to the fact that operations on main memory involve I/O.
        Note: In the Toy system, everything is big endian.
     */
    main: [i8; 5000],
    pub mem_bus: [i8; 10],
    pub registers: [i64; 15],
    pub program_counter: usize,
    pub status: Status,
    // Condition flags for jmp instructions.
    pub overflow_flag: bool,
    pub sign_flag: bool,
    pub zero_flag: bool,

}

impl State {
    pub fn new() -> Self {
        Self {
            main: [0; 5000],
            mem_bus: [0; 10],
            registers: [0; 15],
            program_counter: 0,
            status: Status::AOK,
            overflow_flag: false,
            sign_flag: false,
            zero_flag: false,
        }
    }

    // TODO: Deal with indexes >4090
    /*
        Reading ten bytes at a time is convenient because that is the size of the largest
        instruction.
    */
    pub fn read_mem(&mut self, index: usize) {
        for x in 0..9 {
            self.mem_bus[x] = self.main[x + index];
        }
    }

    pub fn write_mem(&mut self, index: usize, val: i8) {
        self.main[index] = val;
    }


}