use anyhow::Result;
use colored::Colorize;
use std::io::{StdoutLock, Write};

use crate::Brush;

pub struct Game;

impl Game {
    pub fn draw_box(
        lock: &mut StdoutLock<'static>,
        brush: &mut Brush,
        size: (u16, u16),
    ) -> Result<()> {
        let title = "snake.rs";
        let credits = " by cameron";

        let starting_pos = (size.0 / 2) - ((title.len() + credits.len()) / 2) as u16;
        brush.set_brush(starting_pos, 2);
        write!(lock, "{}{}", title.green().bold(), credits.white())?;

        // Set brush at starting position for box
        brush.set_brush(10, 4);

        brush.paint_box(lock, (size.0 - 20, size.1 - 8))?;

        Ok(())
    }
}
