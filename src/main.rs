use std::io;

const BOARD_HEIGHT: usize = 6;
const BOARD_LENGTH: usize = 7;

fn main() {
    let mut board: [[i32; BOARD_LENGTH]; BOARD_HEIGHT] = [[0; BOARD_LENGTH]; BOARD_HEIGHT];
    let mut current_player = 1;
    let winner = loop {
        // print the board
        show_board(board);

        // take turn
        let pos = get_user_input();
        board = place_marker(board, pos-1, current_player);

        if check_win(board, current_player) { break current_player; }
        if check_stalemate(board) { break 0; }

        // set other player
        if current_player == 1 {current_player = 2;} else {current_player = 1;}

    };
    println!("The winner is {}!", winner);
}

// function to show the board
fn show_board(board:[[i32; BOARD_LENGTH]; BOARD_HEIGHT]) {
    for row in board {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
    println!();
}

// function to place a marker, if position was not valid the same board will be returned back
fn place_marker(mut board: [[i32; BOARD_LENGTH]; BOARD_HEIGHT], pos:usize, player:i32) -> [[i32; BOARD_LENGTH]; BOARD_HEIGHT] {

    for row in board.iter_mut().rev() {
        if row[pos] == 0 {
            row[pos] = player;
            break;
        }
    }
    board
}

fn get_user_input() -> usize {
    loop {
        println!("Choose a column to play in!");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse() {
            Ok(position) if position < BOARD_LENGTH+1 && position > 0 => return position,
            _ => println!("Invalid input. Please enter a valid column number.")
        }
    }
}

fn check_stalemate(board:[[i32; BOARD_LENGTH]; BOARD_HEIGHT]) -> bool {

    // if the board contains no zeros, there is a stalemate

    for row in board {
        for col in row {
            if col == 0 { return false; }
        }
    }
    true
}

fn check_win(board:[[i32; BOARD_LENGTH]; BOARD_HEIGHT], player:i32) -> bool {
    
    // iterate through cells, at each one check if it matches the desired player and if so, look around it for solutions

    let min_row: usize = if BOARD_HEIGHT >= 4 { BOARD_HEIGHT - 4 } else { 0 }; // the last row we should search for vertical/diagonal solutions
    let min_col: usize = if BOARD_LENGTH >= 4 { 3 } else { 0 }; // the first column to be checked for down-left diagonal solutions
    let max_col: usize = if BOARD_LENGTH >= 4 { BOARD_LENGTH - 4 } else { 0 }; // the last column we should search for horizontal/down-right diagonal solutions

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

fn check_vert(board:[[i32; BOARD_LENGTH]; BOARD_HEIGHT], pos:(usize, usize)) -> bool {
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

fn check_hor(board:[[i32;BOARD_LENGTH]; BOARD_HEIGHT], pos:(usize, usize)) -> bool {
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

fn check_left_diag(board:[[i32; BOARD_LENGTH]; BOARD_HEIGHT], pos:(usize, usize)) -> bool {
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

fn check_right_diag(board:[[i32; BOARD_LENGTH]; BOARD_HEIGHT], pos:(usize, usize)) -> bool {
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
