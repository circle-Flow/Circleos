use std::io::{stdout, Write};
use crossterm::{
    cursor, event::{self, Event, KeyCode},
    terminal, ExecutableCommand, QueueableCommand,
};
use rand::Rng;
use std::time::{Duration, Instant};

/// A simple terminal-based “engine” that moves a dot around using WASD.
/// This is a placeholder to demonstrate how CircleOSD apps could run.

fn main() -> crossterm::Result<()> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(cursor::Hide)?;

    let (mut x, mut y) = (10u16, 10u16);
    let mut rng = rand::thread_rng();
    let start = Instant::now();

    loop {
        stdout.queue(cursor::MoveTo(x, y))?;
        print!("🟢");
        stdout.flush()?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('w') => y = y.saturating_sub(1),
                    KeyCode::Char('s') => y += 1,
                    KeyCode::Char('a') => x = x.saturating_sub(1),
                    KeyCode::Char('d') => x += 1,
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }

        // Simulate random “enemy” spawn
        if rng.gen_range(0..100) > 98 {
            stdout.queue(cursor::MoveTo(rng.gen_range(0..50), rng.gen_range(0..20)))?;
            print!("❌");
        }

        if start.elapsed().as_secs() > 60 {
            break;
        }
    }

    stdout.execute(cursor::Show)?;
    terminal::disable_raw_mode()?;
    println!("\nGame ended. Thanks for playing!");
    Ok(())
}
