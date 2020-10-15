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

#[allow(dead_code)]
// Condition codes set after arithmetic and logical instructions. Jump instructions look at these.
pub enum ConditionCode {
    // Negative result.
    ZF,
    // Signed result.
    SF,
    // Overflow.
    OF
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
pub struct Mem {
    pub main: [i64; 10000],
    pub registers: [i64; 15],
    pub condition: ConditionCode,
    pub counter: i64,
    pub stat: Status
}
