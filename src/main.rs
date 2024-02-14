use std::{env, io};

use mair_minigames::input_validation::{check, InValType};

pub mod minesweeper;
// use terminal_minigames::game_obj::GameList;

/// Main menu, triggers submodules for each game
// fn main() {
//     let args: Vec<String> = env::args().collect();
//     dbg!(args);

//     let mut menu_input = String::new();
//     print!("Please pick a game: ");
//     // TODO: input validation loop
//     io::stdin()
//         .read_line(&mut menu_input)
//         .expect("failed to read input.");

//     //TODO: menu select parser (match case)

//     println!("{menu_input}");

//     let int_check = check(&menu_input, InValType::Menu);

//     if !int_check {
//         println!("sorry incorrect or poorly formatted input, try again")
//     } else {
//         minesweeper::start_minesweeper()
//     }
// }
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::io::{stdout, Result};
fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // TODO: main loop
    loop {
        // Draw events
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                    .white()
                    .on_blue(),
                area,
            );
        })?;

        // Handle events
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
