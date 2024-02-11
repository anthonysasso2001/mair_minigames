#[doc(inline)]
use std::io::stdin;
use std::vec;

/// value for item (combines mine flag, click flag, and surrounding count)
/// if 0-8 not mine and show on click
/// if 9 this item is a mine
/// if positive it has been clicked
/// if negative it has not yet been clicked

/// State for one minesweeper cell, used in State to build board object
///
/// # Attributes
/// * `value` - value for item (combines mine flag, click flag, and surrounding count)
///     if 0-8 not mine and show on click
///     if 9 this item is a mine
///     if positive it has been clicked
///     if negative it has not yet been clicked
#[derive(Debug, PartialEq, Clone)]
struct BoardItem {
    value: i8,
}

/// State for one minesweeper game, to keep track of board and size etc.
///
/// # Attributes
/// * `score` - score for this game positive values only.
/// * `length` - y dimension for this board, used in initialization.
/// * `width` - x dimension for this board, used in initialization.
/// * `mine_ratio` - number of spaces per mine, used in initialization.
/// * `board` - vector of board, stores state for game through 2d array of BoardItems.
#[derive(Debug, PartialEq)]
struct State {
    score: u64,
    length: usize,
    width: usize,
    mine_ratio: u32,
    board: Vec<Vec<BoardItem>>,
}

impl State {
    fn new(length: usize, width: usize, mine_ratio: u32) -> State {
        let new_board = vec![vec![BoardItem { value: 0 }; width]; length];
        // let nb = Vec<BoardItem>::
        State {
            score: 0,
            length,
            width,
            mine_ratio,
            board: new_board,
        }
    }
}

/// init for start_minesweeper
pub fn start_minesweeper() {
    // Bootstrap minesweeper game
    println!("Welcome to minesweeper, enter the number of spaces/mine");
    // 8*8, 10 mines = 6.4/mine
    println!("Examples:");
    println!("Beginner - 6 spaces/mine");
    // 16*16, 40 mines = 6.4/mine (reducing to 5 due to scaling)
    println!("Intermediate - 5 spaces/mine");
    // 30*16, 99 mines = 4.8 (reducing to 4 due to scaling)
    println!("Expert - 4 spaces/mine");
    // let game_state=State::new(length, width)
    let mut in_scale = String::new();

    print!(": ");
    stdin()
        .read_line(&mut in_scale)
        .expect("Did not enter a correct string");
}
