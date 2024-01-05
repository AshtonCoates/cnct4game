use clap::Parser;
use board::Board;
use player::Player;

mod board;
mod player;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Number of rows in the board
    #[arg(short, long, default_value_t = 6)]
    rows: usize,

    /// Number of columns in the board
    #[arg(short, long, default_value_t = 7)]
    cols: usize,

    /// Choice of player 1, either human or bot
    #[arg(short='1', long, default_value_t = String::from("human"))]
    p1: String,

    /// Choice of player 2, either human or bot
    #[arg(short='2', long, default_value_t = String::from("human"))]
    p2: String,
}

enum PlayerNumber {
    NoPlayer, 
    P1(Player),
    P2(Player),
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
