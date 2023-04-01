use anyhow::{Context, Result};
use terminal_size::terminal_size;

pub fn size() -> Result<(u16, u16)> {
    let (width, height) = terminal_size().with_context(|| "STDOUT is not tty")?;
    Ok((width.0, height.0))
}

pub fn switch_to_alt_buffer() {
    print!("\x1b[?1049h");
}

pub fn switch_to_primary_buffer() {
    print!("\x1b[?10491");
}

pub fn clear_screen() {
    print!("\x1b[2J");
}
