use std::io;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments{
    /// Number of rows in the board
    #[arg(short, long, default_value_t = 6)]
    rows: usize,

    /// Number of columns in the board
    #[arg(short, long, default_value_t = 7)]
    cols: usize,
}

fn main() {

    let args = Arguments::parse();

    let board_height: usize = args.rows;
    let board_length: usize = args.cols;

    let mut board = vec![vec![0;board_length];board_height];
    let mut current_player = 1;
    let winner = loop {
        // print the board
        show_board(&board);

        // take turn
        board = loop {
            let pos:usize = get_user_input(current_player, board_length);
            let board_clone = board.clone();
            let temp_board = place_marker(board_clone, pos-1, current_player);
            if temp_board == board { println!("That column is full! Please enter a valid column number"); } else { break temp_board; }
        };

        if check_win(&board, current_player, board_length, board_height) { break current_player; }
        if check_stalemate(&board) { break 0; }

        // set other player
        current_player = 3 - current_player;

    };
    if winner == 0 {
        println!("Stalemate!");
    } else {
        println!("The winner is {}!", winner);
    }
}

// function to show the board
fn show_board(board:&Vec<Vec<i32>>) {
    for row in board {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
    println!();
}

// function to place a marker, if position was not valid the same board will be returned back
fn place_marker(mut board: Vec<Vec<i32>>, pos:usize, player:i32) -> Vec<Vec<i32>> {

    for row in board.iter_mut().rev() {
        if row[pos] == 0 {
            row[pos] = player;
            break;
        }
    }
    board
}

fn get_user_input(player:i32, board_length:usize) -> usize {
    loop {
        println!("Player {}, choose a column to play in!", player);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse() {
            Ok(position) if position < board_length+1 && position > 0 => return position,
            _ => println!("Invalid input. Please enter a valid column number.")
        }
    }
}

fn check_stalemate(board:&Vec<Vec<i32>>) -> bool {

    // if the board contains no zeros, there is a stalemate

    for row in board {
        for col in row {
            if col == &0 { return false; }
        }
    }
    true
}

fn check_win(board: &Vec<Vec<i32>>, player:i32, board_length:usize, board_height:usize) -> bool {

    // iterate through cells, at each one check if it matches the desired player and if so, look around it for solutions

    let min_row: usize = if board_height >= 4 { board_height - 4 } else { 0 }; // the last row we should search for vertical/diagonal solutions
    let min_col: usize = if board_length >= 4 { 3 } else { 0 }; // the first column to be checked for down-left diagonal solutions
    let max_col: usize = if board_length >= 4 { board_length - 4 } else { 0 }; // the last column we should search for horizontal/down-right diagonal solutions

    for (row_num, row) in board.iter().enumerate() {
        for (col_num, cell) in row.iter().enumerate() {
            let pos: (usize, usize) = (row_num, col_num); // (row, column)
            if *cell == player {
                if row_num <= min_row {
                    if check_vert(board, pos) { return true; }
                }
                if col_num <= max_col {
                    if check_hor(board, pos) { return true; }
                } 
                if (row_num <= min_row) && (col_num <= max_col) {
                    if check_right_diag(board, pos) { return true; }
                }
                if (row_num <= min_row) && col_num >= min_col {
                    if check_left_diag(board, pos) { return true; }
                }
            }
        }
    }
    false // no solutions found
}

fn check_vert(board:&Vec<Vec<i32>>, pos:(usize, usize)) -> bool {
    let pos0 = board[pos.0][pos.1];
    let pos1 = board[pos.0 + 1][pos.1];
    let pos2 = board[pos.0 + 2][pos.1];
    let pos3 = board[pos.0 + 3][pos.1];
    if (pos0 == pos1) && (pos0 == pos2) && (pos0 == pos3) {
        show_board(board);
        true
    } else {
        false
    }
}

fn check_hor(board: &Vec<Vec<i32>>, pos:(usize, usize)) -> bool {
    let pos0 = board[pos.0][pos.1];
    let pos1 = board[pos.0][pos.1 + 1];
    let pos2 = board[pos.0][pos.1 + 2];
    let pos3 = board[pos.0][pos.1 + 3];
    if (pos0 == pos1) && (pos0 == pos2) && (pos0 == pos3) {
        show_board(board);
        true
    } else {
        false
    }
}

fn check_left_diag(board: &Vec<Vec<i32>>, pos:(usize, usize)) -> bool {
    let pos0 = board[pos.0][pos.1];
    let pos1 = board[pos.0 + 1][pos.1 - 1];
    let pos2 = board[pos.0 + 2][pos.1 - 2];
    let pos3 = board[pos.0 + 3][pos.1 - 3];
    if (pos0 == pos1) && (pos0 == pos2) && (pos0 == pos3) {
        show_board(board);
        true
    } else {
        false
    }
}

fn check_right_diag(board: &Vec<Vec<i32>>, pos:(usize, usize)) -> bool {
    let pos0 = board[pos.0][pos.1];
    let pos1 = board[pos.0 + 1][pos.1 + 1];
    let pos2 = board[pos.0 + 2][pos.1 + 2];
    let pos3 = board[pos.0 + 3][pos.1 + 3];
    if (pos0 == pos1) && (pos0 == pos2) && (pos0 == pos3) {
        show_board(board);
        true
    } else {
        false
    }
}
