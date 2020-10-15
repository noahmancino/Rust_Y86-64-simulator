#[path = "model/memory.rs"] mod memory;
#[path = "model/instructions.rs"] mod instructions;
#[allow(non_snake_case)]

fn main() {
    let _code = memory::ConditionCode::ZF;
    println!("Hello, world!");
}
