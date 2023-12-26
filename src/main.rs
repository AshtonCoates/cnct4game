use std::io;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Number of rows in the board
    #[arg(short, long, default_value_t = 6)]
    rows: usize,

    /// Number of columns in the board
    #[arg(short, long, default_value_t = 7)]
    cols: usize,

    /* 
    /// Choice of player 1, either Human or Bot
    #[arg(short, long, default_value_t = Player::Human)]
    p1: Player,

    /// Choice of player 2, either Human or Bot
    #[arg(short, long, default_value_t = Player::Human)]
    p2: Player,
    */
}

struct Board {
    height : usize,
    length : usize,
    player : i32, // current player
    state  : Vec<Vec<i32>>,
}

impl Board {

    fn new(height:usize, length:usize, state:Vec<Vec<i32>>) -> Board {
        Board {
            height : height,
            length : length,
            player : 1,
            state : state,
        }
    }

    // show the board
    fn show(&self) {
        print!("{}[2J", 27 as char);
        for row in &self.state {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }
        println!();
    }

    // function to place a marker, if position was not valid the same board will be returned back
    fn place_marker(&mut self, col:usize,) {

        for row in self.state.iter_mut().rev() {
            if row[col] == 0 {
                row[col] = self.player;
                break;
            }
        }
    }

    fn check_full(&self, col:usize) -> bool {
        for row in &self.state {
            if row[col] == 0 {
                return false;
            }
        }
        true
    }

    fn get_user_input(&self) -> usize {

        loop {
            println!("Player {}, choose a column to play in!", self.player);
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            match input.trim().parse() {
                Ok(position) if position < self.length+1 && position > 0 => return position,
                _ => println!("Invalid input. Please enter a valid column number.")
            }
        }
    }

    fn check_stalemate(&self) -> bool {

        // if the board contains no zeros, there is a stalemate

        for row in &self.state {
            for col in row {
                if col == &0 { return false; }
            }
        }
        true
    }

    fn check_win(&self) -> bool {

        // iterate through cells, at each one check if it matches the desired player and if so, look around it for solutions

        let min_row: usize = self.height - 4; // the last row we should search for vertical/diagonal solutions
        let min_col: usize = 3; // the first column to be checked for down-left diagonal solutions
        let max_col: usize = self.length; // the last column we should search for horizontal/down-right diagonal solutions

        for (row_num, row) in self.state.iter().enumerate() {
            for (col_num, cell) in row.iter().enumerate() {
                let pos: (usize, usize) = (row_num, col_num); // (row, column)
                if *cell == self.player {
                    if row_num <= min_row {
                        if self.check_vert(pos) { return true; }
                    }
                    if col_num <= max_col {
                        if self.check_hor(pos) { return true; }
                    } 
                    if (row_num <= min_row) && (col_num <= max_col) {
                        if self.check_right_diag(pos) { return true; }
                    }
                    if (row_num <= min_row) && col_num >= min_col {
                        if self.check_left_diag(pos) { return true; }
                    }
                }
            }
        }
        false // no solutions found
    }

    // the 4 functions below check for different patterns of wins

    fn check_vert(&self, pos:(usize, usize)) -> bool {
        let pos0 = self.state[pos.0][pos.1];
        let pos1 = self.state[pos.0 + 1][pos.1];
        let pos2 = self.state[pos.0 + 2][pos.1];
        let pos3 = self.state[pos.0 + 3][pos.1];
        if (pos0 == pos1) && (pos0 == pos2) && (pos0 == pos3) {
            self.show();
            true
        } else {
            false
        }
    }

    fn check_hor(&self, pos:(usize, usize)) -> bool {
        let pos0 = self.state[pos.0][pos.1];
        let pos1 = self.state[pos.0][pos.1 + 1];
        let pos2 = self.state[pos.0][pos.1 + 2];
        let pos3 = self.state[pos.0][pos.1 + 3];
        if (pos0 == pos1) && (pos0 == pos2) && (pos0 == pos3) {
            self.show();
            true
        } else {
            false
        }
    }

    fn check_left_diag(&self, pos:(usize, usize)) -> bool {
        let pos0 = self.state[pos.0][pos.1];
        let pos1 = self.state[pos.0 + 1][pos.1 - 1];
        let pos2 = self.state[pos.0 + 2][pos.1 - 2];
        let pos3 = self.state[pos.0 + 3][pos.1 - 3];
        if (pos0 == pos1) && (pos0 == pos2) && (pos0 == pos3) {
            self.show();
            true
        } else {
            false
        }
    }

    fn check_right_diag(&self, pos:(usize, usize)) -> bool {
        let pos0 = self.state[pos.0][pos.1];
        let pos1 = self.state[pos.0 + 1][pos.1 + 1];
        let pos2 = self.state[pos.0 + 2][pos.1 + 2];
        let pos3 = self.state[pos.0 + 3][pos.1 + 3];
        if (pos0 == pos1) && (pos0 == pos2) && (pos0 == pos3) {
            self.show();
            true
        } else {
            false
        }
    }

}

#[derive(Debug)]
enum Player {
    Human,
    Bot,
}


fn main() {

    // get board size args
    let args = Arguments::parse();

    let mut board = Board::new(args.rows, args.cols, vec![vec![0; args.cols]; args.rows]);

    if board.height < 4 || board.length < 4 {
        println!("Board height and length must be at least 4! Please try again.");
        return;
    }

    let winner: i32 = loop {

        board.show();
        
        let mut col = board.get_user_input() - 1;
        while board.check_full(col) {
            println!("That column was full, please choose a valid column!");
            col = board.get_user_input() - 1;
        }

        board.place_marker(col);
        if board.check_win() {
            break board.player;
        } else if board.check_stalemate() {
            break 0;
        }
        board.player = 3 - board.player;

    };
    if winner == 0 {
        println!("Stalemate!");
    } else {
        println!("The winner is {}!", winner);
    }
}
