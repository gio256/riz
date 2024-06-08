/// Note that these opcodes are not one-to-one with riscv instructions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Opcode {
    // arithmetic
    Add,
    Sub,
    Slt,
    Sltu,

    // logic
    And,
    Or,
    Xor,

    // memory load ops
    Lw,
    Lb,
    Lh,
    Lbu,
    Lhu,

    // memory store ops
    Sw,
    Sb,
    Sh,

    // jumps
    Jal,
    Jalr,

    // branching
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,
}
