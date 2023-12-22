use clap::Parser;
use crossterm::{
    cursor,
    event::{self, KeyCode, KeyEvent, KeyEventKind},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::stdout;
use std::{thread, time};

const SNOWFLAKE1: char = '❄';
const SNOWFLAKE2: char = '❅';
const SNOWFLAKE3: char = '❆';
const SNOWFLAKE4: char = '❉';
const SNOWFLAKE5: char = '❊';
const SNOWFLAKE6: char = '❋';

const SF_CANDIDATES: [char; 6] = [
    SNOWFLAKE1, SNOWFLAKE2, SNOWFLAKE3, SNOWFLAKE4, SNOWFLAKE5, SNOWFLAKE6,
];

const TREE1: &str = "    *    ";
const TREE2: &str = "   ***   ";
const TREE3: &str = "  *****  ";
const TREE4: &str = " ******* ";
const STOC1: &str = "   | |   ";
const STOC2: &str = "   | |   ";

/// Merry Christmas!
#[derive(Parser, Debug)]
#[command(name = "rsnow", author, version, about, long_about = None)]
struct Args {
    /// Speed of snowfall
    #[arg(short, long, default_value_t = 1, value_parser=clap::value_parser!(u16).range(1..=5))]
    speed: u16,

    /// Quantity of snowflakes
    #[arg(short, long, default_value_t = 20, value_parser=clap::value_parser!(u8).range(0..=100))]
    quantity: u8,

    /// Bool, enabled rainbow color [default is false]
    #[arg(short, long, default_value_t = false)]
    rainbow: bool,

    /// Bool, enabled random shape [default is false]
    #[arg(short = 't', long = "randomtype", default_value_t = false)]
    random: bool,

    /// Bool, remove the Christmas tree [default is false]
    #[arg(short, long, default_value_t = false)]
    notree: bool,
}

struct Snowflake {
    x: u16,
    y: u16,
    color: Color,
    shape: char,
}

fn get_snowflake(random: bool, rainbow: bool, width: u16) -> Snowflake {
    // Default color is white
    let mut color = Color::White;
    // Generate random color if rainbow mode is enabled
    if rainbow {
        let rd_r = rand::random::<u8>();
        let rd_g = rand::random::<u8>();
        let rd_b = rand::random::<u8>();
        color = Color::Rgb {
            r: rd_r,
            g: rd_g,
            b: rd_b,
        }
    }

    // Default shape
    let mut shape = SNOWFLAKE1;
    // Generate random shape if random mode is enabled
    if random {
        shape = SF_CANDIDATES[rand::random::<usize>() % 6];
    }

    // Return snowflake
    Snowflake {
        x: rand::random::<u16>() % width,
        y: 0,
        color,
        shape,
    }
}

fn main() {
    // CLI
    let args = Args::parse();
    let speed = args.speed;
    let quantity = args.quantity;
    let notree = args.notree;
    let rainbow = args.rainbow;
    let random = args.random;

    // Setup
    terminal::enable_raw_mode().unwrap();

    // Clear the terminal
    execute!(stdout(), terminal::Clear(ClearType::All), cursor::Hide).unwrap();

    // Initialize snowflakes
    let mut snowflakes: Vec<Snowflake> = Vec::new();

    loop {
        // Get terminal size
        let size = terminal::size().unwrap();
        let width = size.0;
        let height = size.1;

        // Generate new snowflake
        let new_snowflake = get_snowflake(random, rainbow, width);
        snowflakes.push(new_snowflake);

        // Render tree if enabled
        if !notree {
            let tree_pos = width / 2 - 6;
            execute!(
                stdout(),
                cursor::MoveTo(tree_pos, height - 6),
                SetForegroundColor(Color::Green),
                Print(TREE1),
                cursor::MoveTo(tree_pos, height - 5),
                Print(TREE2),
                cursor::MoveTo(tree_pos, height - 4),
                Print(TREE3),
                cursor::MoveTo(tree_pos, height - 3),
                Print(TREE4),
                cursor::MoveTo(tree_pos, height - 2),
                SetForegroundColor(Color::Red),
                Print(STOC1),
                cursor::MoveTo(tree_pos, height - 1),
                Print(STOC2),
            )
            .unwrap();
        }

        // Render snowflakes
        for flake in &snowflakes {
            execute!(
                stdout(),
                cursor::MoveTo(flake.x, flake.y),
                SetForegroundColor(flake.color),
                Print(flake.shape),
            )
            .unwrap();
        }

        // Move snowflakes down
        snowflakes = snowflakes
            .into_iter()
            .filter(|flake| flake.y < height - 1)
            .map(|mut flake| {
                flake.y += speed; // speed
                flake
            })
            .collect();

        // Add new snowflakes
        for _ in 1..quantity {
            // for quantity
            let new_snowflake = get_snowflake(random, rainbow, width);
            snowflakes.push(new_snowflake);
        }

        // Refresh the terminal
        execute!(stdout(), cursor::MoveTo(0, 0)).unwrap();

        // Sleep to control the falling speed
        thread::sleep(time::Duration::from_millis(100));

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
