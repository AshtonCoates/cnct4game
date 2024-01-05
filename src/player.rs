
use crate::board::Board;
use crate::PlayerNumber;

const INFINITY: f64 = std::f64::INFINITY;
const NEG_INFINITY: f64 = std::f64::NEG_INFINITY;

pub enum Player {
    Human,
    Bot(Board, usize), // data for board info and max depth is included directly in the enumeration
}

impl Player { // using the minimax algorithm with alpha-beta pruning



    fn make_move(&self) -> usize {
        0
    }

    fn eval_board(board:Vec<Vec<i32>>) -> i32 {
        0
    }

    fn simulate_move(board: Vec<Vec<i32>>, col:usize) -> Vec<Vec<i32>> {
        vec![vec![0;3];3]
    }

}