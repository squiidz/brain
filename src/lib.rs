mod instruction;
pub mod compiler;
pub mod machine;
pub mod bytecode;

pub use bytecode::{ByteCode, ByteCodeType};
pub use compiler::Compiler;
pub use machine::Machine;

pub mod prelude {
    pub use compiler::Compiler;
    pub use machine::Machine;
    pub use bytecode::ByteCode;
    use std::io::{Read, Write};

    pub fn execute<R: Read, W: Write>(code: &str, r: R, w: W) -> Result<(), String> {
        let mut comp = Compiler::new(code);
        let instruction = comp.compile();
        let mut machine = Machine::new(&instruction, r, w);
        machine.execute()
    }
}
