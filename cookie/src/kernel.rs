use crate::header::ElfHeader;
use anyhow::Result;
use std::{fs::OpenOptions, io::Read};

pub struct Kernel;

impl Kernel {
    pub fn process(path: String) -> Result<()> {
        let mut f = OpenOptions::new().read(true).create(false).open(path)?;
        let mut bytes = Vec::new();
        f.read_to_end(&mut bytes)?;
        let header = ElfHeader::extract_from_file_bytes(&bytes)?;
        let ip = header.start;
        println!("{header:?}");
        Ok(())
    }
}
