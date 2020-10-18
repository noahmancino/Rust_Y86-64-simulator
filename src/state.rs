/*
    This file defines the structure of the systems memory state.
 */


#[allow(dead_code)]
pub struct RegisterFile {
    rax: i64,
    rcx: i64,
    rdx: i64,
    rbx: i64,
    rsp: i64,
    rbp: i64,
    rsi: i64,
    rdi: i64,
    r8: i64,
    r9: i64,
    r10: i64,
    r11: i64,
    r12: i64,
    r13: i64,
    r14: i64
}

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
    pub registers: [i64; 15],
    // Program counter.
    pub PC: i64,
    pub stat: Status,
    // Condition flags for jmp instructions.
    pub OF: bool,
    pub SF: bool,
    pub ZF: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            main: [0; 5000],
            registers: [0; 15],
            PC: 0,
            stat: Status::AOK,
            OF: false,
            SF: false,
            ZF: false,
        }
    }

    // TODO: Deal with indexes >4090
    /*
        Reading ten bytes at a time is convenient because that is the size of the largest
        instruction.
    */
    pub fn read_mem(&self, index: u16) -> Vec<i8> {
        let mut x: Vec<i8> = vec![0; 10];
        x.copy_from_slice(&(self.main[index as usize .. (index as usize + 10)]));
        x
    }


}