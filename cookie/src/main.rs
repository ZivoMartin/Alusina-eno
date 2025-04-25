use anyhow::{Result, bail};
use kernel::Kernel;
use std::env::args;
mod instr;
mod kernel;
mod registers;

fn main() -> Result<()> {
    match args().nth(1) {
        Some(path) => Kernel::process(path),
        None => bail!("Please provide a path for an executable"),
    }
}
