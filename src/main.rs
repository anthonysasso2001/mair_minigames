use std::{env, io};

use mair_minigames::input_validation::{check, InValType};

pub mod minesweeper;
// use terminal_minigames::game_obj::GameList;

/// Main menu, triggers submodules for each game
fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);

    let mut menu_input = String::new();
    print!("Please pick a game: ");
    // TODO: input validation loop
    io::stdin()
        .read_line(&mut menu_input)
        .expect("failed to read input.");

    //TODO: menu select parser (match case)

    println!("{menu_input}");

    let int_check = check(&menu_input, InValType::Menu);

    if !int_check {
        println!("sorry incorrect or poorly formatted input, try again")
    } else {
        minesweeper::start_minesweeper()
    }
}
