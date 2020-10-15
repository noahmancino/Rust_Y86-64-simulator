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
    pub main: [i64; 1000],
    pub registers: [i64; 15],
    // Program counter.
    pub PC: i64,
    pub stat: Status,
    // Condition flags for jmp instructions.
    pub OF: bool,
    pub SF: bool,
    pub ZF: bool,
}
