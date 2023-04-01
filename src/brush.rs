use anyhow::{Ok, Result};
use std::io::{StdoutLock, Write};

use crate::io::flush_output;

#[derive(Default)]
pub struct Brush(u16, u16);

impl Brush {
    pub fn set_brush(&mut self, x: u16, y: u16) {
        *self = Self(x, y);
        print!("\x1b[{y};{x}H");
    }

    pub fn move_brush<T: Into<u16>>(&self, x: T, y: T) {
        let x = x.into();
        let y = y.into();
        print!(
            "\x1b[{};{}H",
            self.1.wrapping_add(y),
            self.0.wrapping_add(x)
        );
    }

    pub fn hide_cursor() {
        print!("\x1b[?25l");
    }

    pub fn show_cursor() {
        print!("\x1b[?25h");
    }

    pub fn paint_box(&mut self, lock: &mut StdoutLock<'static>, size: (u16, u16)) -> Result<()> {
        // Paint the top left corner
        write!(lock, "┏")?;

        // For some reason rust says that this is an i32 without the u16
        self.move_brush(1u16, 0);
        // Paint the top bars
        write!(lock, "{}", "━".repeat((size.0 - 1).into()))?;

        self.move_brush(size.0, 0);
        write!(lock, "┓")?;

        for i in 1..=size.1 {
            self.move_brush(0, i);
            write!(lock, "┃")?;
            self.move_brush(size.0, i);
            write!(lock, "┃")?;
        }

        self.move_brush(0, size.1);
        write!(lock, "┗")?;

        self.move_brush(1, size.1);
        write!(lock, "{}", "━".repeat((size.0 - 1).into()))?;

        write!(lock, "┛")?;

        flush_output()?;

        Ok(())
    }
}
