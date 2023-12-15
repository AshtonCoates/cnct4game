use std::io;

const BOARD_HEIGHT: usize = 6;
const BOARD_LENGTH: usize = 7;

fn main() {
    let mut board: [[i32; BOARD_LENGTH]; BOARD_HEIGHT] = [[0; BOARD_LENGTH]; BOARD_HEIGHT];
    let mut current_player = 1;
    loop {
        // print the board
        show_board(board);

        // take turn
        let pos = get_user_input();
        board = place_marker(board, pos-1, current_player);

        if check_win(board, current_player) { break; }

        // set other player
        if current_player == 1 {current_player = 2;} else {current_player = 1;}

    }
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

fn check_win(board:[[i32; BOARD_LENGTH]; BOARD_HEIGHT], player:i32) -> bool {
    false
}
