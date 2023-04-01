use anyhow::Result;
use std::io::stdout;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

mod brush;
use brush::Brush;

mod game;
use game::*;

mod terminal;

mod io;

fn main() -> Result<()> {
    // Initial setup
    terminal::switch_to_alt_buffer();
    terminal::clear_screen();
    Brush::hide_cursor();

    let (width, height) = terminal::size()?;

    let mut cursor = Brush::default();

    // TODO: add back
    //let mut rng = thread_rng();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {
        let mut std_lock = stdout().lock();
        
        Game::draw_box(&mut std_lock, &mut cursor, (width, height))?;
        
        std::thread::sleep(std::time::Duration::from_millis(30));
        terminal::clear_screen();
    }

    terminal::clear_screen();
    cursor.set_brush(0, 0);
    Brush::show_cursor();
    terminal::switch_to_primary_buffer();

    Ok(())
}
