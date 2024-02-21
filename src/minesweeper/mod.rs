#[doc(inline)]
// use std::io::stdin;
use rand::Rng;
use std::{fmt::Debug, vec};
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
struct MinesweeperState {
    score: u64,
    mine_ratio: u32,
    board: Vec<Vec<BoardItem>>,
}

#[derive(Debug)]
enum MinesweeperValues {
    Empty = 0,
    Mine = 9,
    // Flagged = -9,
}

impl MinesweeperState {
    fn new(length: usize, width: usize, mine_ratio: u32) -> MinesweeperState {
        // BoardItem { value: 0 }; width * length
        let mut new_board = vec![
            vec![
                BoardItem {
                    value: MinesweeperValues::Empty as i8
                };
                width
            ];
            length
        ];

        let mut mine_count = length * width / mine_ratio as usize;

        while mine_count > 0 {
            let temp_len = rand::thread_rng().gen_range(0..length);
            let temp_wid = rand::thread_rng().gen_range(0..width);

            let check_cell: &mut BoardItem = &mut new_board[temp_wid][temp_len];

            if check_cell.value == 0 {
                check_cell.value = MinesweeperValues::Mine as i8;
                mine_count -= 1;
            }
        }

        // /*
        for row in 0..new_board.len() {
            for column in 0..new_board[row].len() {
                if MinesweeperValues::Mine as i8 != new_board[row][column].value {
                    // because max(0, row - 1) and min(row + 1, length) breaks with usize
                    let x_min = if row > 0 { row - 1 } else { 0 };
                    let x_max = if row < length - 1 {
                        row + 1
                    } else {
                        length - 1
                    };

                    for x in x_min..x_max + 1 {
                        let y_min = if column > 0 { column - 1 } else { 0 };
                        let y_max = if column < width - 1 {
                            column + 1
                        } else {
                            width - 1
                        };

                        for y in y_min..y_max + 1 {
                            if x != row || y != column {
                                if MinesweeperValues::Mine as i8 == new_board[x][y].value {
                                    new_board[row][column].value += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        // */
        MinesweeperState {
            score: 0,
            mine_ratio,
            board: new_board,
        }
    }
}

/// init for start_minesweeper
pub fn start_minesweeper() {
    let length = 16;
    let width = 16;
    let mine_ratio = 4;
    let minesweeper = MinesweeperState::new(length, width, mine_ratio);

    println!(
        "\nlength: {0}, width: {1}, ratio: {2}, mines: {3}\n",
        length,
        width,
        minesweeper.mine_ratio,
        length * width / minesweeper.mine_ratio as usize
    );

    for row in minesweeper.board.into_iter() {
        for cell in row.into_iter() {
            // incase I need to use format: format!("\x1b[31m{val}\x1b[37m", val = cell.value)
            let print_val: String = match cell.value {
                1..=8 => format!("\x1b[33m{val}\x1b[37m", val = cell.value),
                9 => "\x1b[31m@\x1b[37m".to_string(),
                -9 => "\x1b[31mF\x1b[37m".to_string(), // for flagged options
                _ => "?".to_string(),
            };

            print!("[{0}]", print_val);
        }
        print!("\n");
    }
}
