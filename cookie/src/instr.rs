use crate::registers::{RD, RS};
use anyhow::{Result, bail};

pub type Imm = u32;
pub type Shamt = i32;
pub type Pred = u8;
pub type Succ = u8;
pub type Zimm = u32;
pub type Csr = u32;

pub enum Instr {
    Lui(Imm, RD),
    AUIPC(Imm, RD),
    JAL,
    JALR(Imm, RS, RD),
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,
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

macro_rules! rd {
    ($instr:expr) => {
        RD::from(((($instr >> 7) & 0x1F) as u8))
    };
}

macro_rules! rs1 {
    ($instr:expr) => {
        RS::from(((($instr >> 15) & 0x1F) as u8))
    };
}

macro_rules! rs2 {
    ($instr:expr) => {
        RS::from(((($instr >> 20) & 0x1F) as u8))
    };
}

macro_rules! imm_right {
    ($instr:expr) => {
        (($instr >> 7) & 0x1F) as Imm
    };
}

macro_rules! imm_left {
    ($instr:expr) => {
        (($instr as i32) >> 20) as Imm
    };
}

macro_rules! left_flag {
    ($instr:expr) => {
        (($instr >> 30) & 0x1) as u8
    };
}

macro_rules! flag {
    ($instr:expr) => {
        (($instr >> 12) & 0x7) as u8
    };
}

macro_rules! shamt {
    ($instr:expr) => {
        (($instr >> 20) & 0x1F) as i32
    };
}

macro_rules! pred {
    ($instr:expr) => {
        ((($instr >> 24) & 0xF) as u8)
    };
}
macro_rules! succ {
    ($instr:expr) => {
        ((($instr >> 20) & 0xF) as u8)
    };
}

macro_rules! zimm {
    ($instr:expr) => {
        (($instr >> 15) & 0x1F) as Zimm
    };
}

macro_rules! csr {
    ($instr:expr) => {
        (($instr >> 20) & 0xFFF) as Csr
    };
}

impl Instr {
    pub fn decode(instr: u32) -> Result<Self> {
        Ok(match instr & 0x7F {
            0b0110111 => Self::Lui(imm_left!(instr), rd!(instr)),
            0b0010111 => Self::AUIPC(imm_left!(instr), rd!(instr)),
            0b1101111 => todo!("JAL"),
            0b1100111 => Self::JALR(imm_left!(instr), rs1!(instr), rd!(instr)),
            0b1100011 => match flag!(instr) {
                0b000 => Self::Beq,
                0b001 => Self::Bne,
                0b100 => Self::Blt,
                0b101 => Self::Bge,
                0b110 => Self::Bltu,
                0b111 => Self::Bgeu,
                _ => bail!("Invalid branch funct3: {}", flag!(instr)),
            },
            0b0000011 => match flag!(instr) {
                0b000 => Self::Lb(imm_left!(instr), rs1!(instr), rd!(instr)),
                0b001 => Self::Lh(imm_left!(instr), rs1!(instr), rd!(instr)),
                0b010 => Self::Lw(imm_left!(instr), rs1!(instr), rd!(instr)),
                0b100 => Self::Lbu(imm_left!(instr), rs1!(instr), rd!(instr)),
                0b101 => Self::Lhu(imm_left!(instr), rs1!(instr), rd!(instr)),
                _ => bail!("Invalid flag for loop: {}", flag!(instr)),
            },
            0b0100011 => match flag!(instr) {
                0b000 => Self::Sb(
                    imm_left!(instr),
                    rs2!(instr),
                    rs1!(instr),
                    imm_right!(instr),
                ),
                0b001 => Self::Sh(
                    imm_left!(instr),
                    rs2!(instr),
                    rs1!(instr),
                    imm_right!(instr),
                ),
                0b010 => Self::Sw(
                    imm_left!(instr),
                    rs2!(instr),
                    rs1!(instr),
                    imm_right!(instr),
                ),
                _ => bail!("Invalid flag for shift operation: {}", flag!(instr)),
            },
            0b0010011 => match flag!(instr) {
                0b000 => Self::Addi(imm_left!(instr), rs1!(instr), rd!(instr)),
                0b010 => Self::Slti(imm_left!(instr), rs1!(instr), rd!(instr)),
                0b011 => Self::Sltiu(imm_left!(instr), rs1!(instr), rd!(instr)),
                0b100 => Self::Xori(imm_left!(instr), rs1!(instr), rd!(instr)),
                0b110 => Self::Ori(imm_left!(instr), rs1!(instr), rd!(instr)),
                0b111 => Self::Andi(imm_left!(instr), rs1!(instr), rd!(instr)),
                0b001 => Self::Slli(shamt!(instr), rs1!(instr), rd!(instr)),
                0b101 => match left_flag!(instr) {
                    0b0 => Self::Srli(shamt!(instr), rs1!(instr), rd!(instr)),
                    0b1 => Self::Srai(shamt!(instr), rs1!(instr), rd!(instr)),
                    _ => bail!(
                        "Invalid flag for srl immediate operation: {}",
                        left_flag!(instr)
                    ),
                },
                _ => bail!("Invalid flag for immediate operation: {}", flag!(instr)),
            },
            0b0110011 => match flag!(instr) {
                0b000 => match left_flag!(instr) {
                    0b0 => Self::Add(rs2!(instr), rs1!(instr), rd!(instr)),
                    0b1 => Self::Sub(rs2!(instr), rs1!(instr), rd!(instr)),
                    _ => bail!("Invalid flag for add/sub operation"),
                },
                0b001 => Self::Sll(shamt!(instr), rs1!(instr), rd!(instr)),
                0b010 => Self::Slt(shamt!(instr), rs1!(instr), rd!(instr)),
                0b011 => Self::Sltu(shamt!(instr), rs1!(instr), rd!(instr)),
                0b100 => Self::Xor(rs2!(instr), rs1!(instr), rd!(instr)),
                0b101 => match left_flag!(instr) {
                    0b0 => Self::Srl(rs2!(instr), rs1!(instr), rd!(instr)),
                    0b1 => Self::Sra(rs2!(instr), rs1!(instr), rd!(instr)),
                    _ => bail!("Invalid flag for srl operation: {}", left_flag!(instr)),
                },
                0b110 => Self::Or(rs2!(instr), rs1!(instr), rd!(instr)),
                0b111 => Self::And(rs2!(instr), rs1!(instr), rd!(instr)),
                _ => bail!("Invalid flag for operation: {}", flag!(instr)),
            },
            0b0001111 => match flag!(instr) {
                0b0 => Self::Fence(pred!(instr), succ!(instr)),
                0b1 => Self::FenceI,
                _ => bail!("Invalid flag for Fence instruction: {}", flag!(instr)),
            },
            0b1110011 => match flag!(instr) {
                0b000 => match left_flag!(instr) {
                    0b0 => Self::Ecall,
                    0b1 => Self::Ebreak,
                    _ => bail!(
                        "Invalid flag for break/call operation: {}",
                        left_flag!(instr)
                    ),
                },
                0b001 => Self::Csrrw(csr!(instr), rs1!(instr), rd!(instr)),
                0b010 => Self::Csrrs(csr!(instr), rs1!(instr), rd!(instr)),
                0b011 => Self::Csrrc(csr!(instr), rs1!(instr), rd!(instr)),
                0b101 => Self::Cssrwi(csr!(instr), zimm!(instr), rd!(instr)),
                0b110 => Self::Cssrsi(csr!(instr), zimm!(instr), rd!(instr)),
                0b111 => Self::Cssrci(csr!(instr), zimm!(instr), rd!(instr)),
                _ => bail!("Invalid flag for Css instruction: {}", flag!(instr)),
            },
            _ => bail!("Unrecognized opcode: 0x{:02x}", instr & 0x7F),
        })
    }
}
