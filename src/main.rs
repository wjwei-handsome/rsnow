use crossterm::{
    cursor,
    event::{self, KeyCode, KeyEvent, KeyEventKind},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::stdout;
use std::{thread, time};

const SNOWFLAKE: char = '❄';

struct Snowflake {
    x: u16,
    y: u16,
}

fn main() {
    // Setup
    terminal::enable_raw_mode().unwrap();
    // Get terminal size
    let size = terminal::size().unwrap();
    let width = size.0;
    let height = size.1;

    // Clear the terminal
    execute!(stdout(), terminal::Clear(ClearType::All), cursor::Hide).unwrap();

    // Initialize snowflakes
    let mut snowflakes: Vec<Snowflake> = Vec::new();

    loop {
        // Generate new snowflake
        let new_snowflake = Snowflake {
            x: rand::random::<u16>() % width,
            y: 0,
        };
        snowflakes.push(new_snowflake);

        // Render snowflakes
        for flake in &snowflakes {
            execute!(
                stdout(),
                cursor::MoveTo(flake.x, flake.y),
                SetForegroundColor(Color::White),
                Print(SNOWFLAKE),
            )
            .unwrap();
        }

        // Move snowflakes down
        snowflakes = snowflakes
            .into_iter()
            .filter(|flake| flake.y < height - 1)
            .map(|mut flake| {
                flake.y += 1;
                flake
            })
            .collect();

        // Refresh the terminal
        execute!(stdout(), cursor::MoveTo(0, 0)).unwrap();

        // Sleep to control the falling speed
        thread::sleep(time::Duration::from_millis(10));

        // Check for exit
        if event::poll(time::Duration::from_millis(0)).unwrap() {
            if let event::Event::Key(KeyEvent {
                code,
                modifiers: _,
                state: _,
                kind,
            }) = event::read().unwrap()
            {
                if code == KeyCode::Char('q') && kind == KeyEventKind::Press {
                    break;
                }
            }
        }

        // Clear the terminal
        execute!(stdout(), terminal::Clear(ClearType::All),).unwrap();
    }

    // Clean up
    execute!(stdout(), cursor::Show, terminal::Clear(ClearType::All)).unwrap();
    terminal::disable_raw_mode().unwrap();
}