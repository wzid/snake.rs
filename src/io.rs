use anyhow::Result;
use std::io::{self, Write};

pub fn flush_output() -> Result<()> {
    io::stdout().flush()?;
    Ok(())
}
