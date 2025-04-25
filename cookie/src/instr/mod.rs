mod decode;

use crate::registers::{RD, RS};
pub type Imm = u32;
pub type Shamt = i32;
pub type Pred = u8;
pub type Succ = u8;
pub type Zimm = u32;
pub type Csr = u32;

pub enum Instr {
    Lui(Imm, RD),
    AUIPC(Imm, RD),
    Jal(Imm, RD),
    Jalr(Imm, RS, RD),
    Beq(Imm, RS, RS),
    Bne(Imm, RS, RS),
    Blt(Imm, RS, RS),
    Bge(Imm, RS, RS),
    Bltu(Imm, RS, RS),
    Bgeu(Imm, RS, RS),
    Lb(Imm, RS, RD),
    Lh(Imm, RS, RD),
    Lw(Imm, RS, RD),
    Lbu(Imm, RS, RD),
    Lhu(Imm, RS, RD),
    Sb(Imm, RS, RS, Imm),
    Sh(Imm, RS, RS, Imm),
    Sw(Imm, RS, RS, Imm),
    Addi(Imm, RS, RD),
    Slti(Imm, RS, RD),
    Sltiu(Imm, RS, RD),
    Xori(Imm, RS, RD),
    Ori(Imm, RS, RD),
    Andi(Imm, RS, RD),
    Slli(Shamt, RS, RD),
    Srli(Shamt, RS, RD),
    Srai(Shamt, RS, RD),
    Add(RS, RS, RD),
    Sub(RS, RS, RD),
    Sll(Shamt, RS, RD),
    Slt(Shamt, RS, RD),
    Sltu(Shamt, RS, RD),
    Xor(RS, RS, RD),
    Srl(RS, RS, RD),
    Sra(RS, RS, RD),
    Or(RS, RS, RD),
    And(RS, RS, RD),
    Fence(Pred, Succ),
    FenceI,
    Ecall,
    Ebreak,
    Csrrw(Csr, RS, RD),
    Csrrs(Csr, RS, RD),
    Csrrc(Csr, RS, RD),
    Cssrwi(Csr, Zimm, RD),
    Cssrsi(Csr, Zimm, RD),
    Cssrci(Csr, Zimm, RD),
}
