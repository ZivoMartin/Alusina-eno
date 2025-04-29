use std::{fs::File, io::Read};

use anyhow::{Context, Result, bail, ensure};

#[derive(Debug, Default)]
pub enum FileClass {
    #[default]
    Elf32,
    Elf64,
}

#[derive(Debug, Default)]
pub enum Endianness {
    #[default]
    Little,
    Big,
}

#[derive(Debug, Default)]
pub enum Abi {
    #[default]
    SystemV,
    HPUX,
    NetBSD,
    Linux,
    Solaris,
    AIX,
    IRIX,
    FreeBSD,
    Tru64,
    Novell,
    OpenBSD,
    OpenVMS,
    NonStop,
    AROS,
    Fuchsia,
    Fuchsia64,
    Linux64,
}

#[derive(Debug, Default)]
pub enum FileType {
    #[default]
    Exec, // Executable file
    Rel,  // Contains code and data, but not executable
    Dyn,  // Shared libs
    Core, // Generated when program crashes, contains general memory state
    Os,   // Os file
    Hios, // Architecture specific file content, specific usage
    Loos, // Architecture specific file content, specific usage
    None, // None type, file does not contain any information
}

impl TryFrom<u16> for FileType {
    type Error = anyhow::Error;

    fn try_from(value: u16) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            0x0001 => Self::Exec,
            0x0002 => Self::Rel,
            0x0003 => Self::Dyn,
            0x0004 => Self::Core,
            0x0005 => Self::Os,
            0x0006 => Self::Hios,
            0x0007 => Self::Loos,
            0xFFFF => Self::None,
            _ => bail!("Unknown file type: {value}"),
        })
    }
}

impl TryFrom<u8> for Abi {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            0x00 => Self::SystemV,
            0x01 => Self::HPUX,
            0x02 => Self::NetBSD,
            0x03 => Self::Linux,
            0x04 => Self::Solaris,
            0x06 => Self::AIX,
            0x07 => Self::IRIX,
            0x08 => Self::FreeBSD,
            0x09 => Self::Tru64,
            0x0A => Self::Novell,
            0x0B => Self::OpenBSD,
            0x0C => Self::OpenVMS,
            0x0D => Self::NonStop,
            0x0E => Self::AROS,
            0x0F => Self::Fuchsia,
            0x10 => Self::Fuchsia64,
            0x11 => Self::Linux64,
            _ => bail!("Invalid ABI: {value}"),
        })
    }
}

impl TryFrom<u8> for FileClass {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            0x01 => Ok(FileClass::Elf32),
            0x02 => Ok(FileClass::Elf64),
            _ => bail!("Invalid file class: {value}"),
        }
    }
}

impl TryFrom<u8> for Endianness {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            0x01 => Ok(Endianness::Little),
            0x02 => Ok(Endianness::Big),
            _ => bail!("Invalid endianness: {value}"),
        }
    }
}

#[derive(Default, Debug)]
pub struct ElfHeader {
    pub class: FileClass,
    pub endianness: Endianness,
    pub abi: Abi,
    pub file_type: FileType,
    pub start: u64,
    pub phoff: u64,
    pub shoff: u64,
}

impl ElfHeader {
    pub fn extract_from_file_bytes(bytes: &Vec<u8>) -> Result<Self> {
        let mut h = ElfHeader::default();

        ensure!(bytes[..4] == [0x7F, 0x45, 0x4C, 0x46], "Not an ELF file");
        ensure!(
            u16::from_le_bytes([bytes[0x12], bytes[0x13]]) == 0xF3,
            "Not a RISC-V ELF file"
        );
        h.class = FileClass::try_from(bytes[5])?;
        h.endianness = Endianness::try_from(bytes[6])?;
        h.abi = Abi::try_from(bytes[8])?;
        h.file_type = FileType::try_from(u16::from_le_bytes([bytes[0x10], bytes[0x11]]))?;
        h.start = u64::from_le_bytes(
            bytes[0x18..0x20]
                .try_into()
                .context("Failed to parse starting point")?,
        );

        h.phoff = u64::from_le_bytes(
            bytes[0x20..0x28]
                .try_into()
                .context("Failed to parse phoff")?,
        );
        h.shoff = u64::from_le_bytes(
            bytes[0x28..0x30]
                .try_into()
                .context("Failed to parse shoff")?,
        );

        Ok(h)
    }
}
