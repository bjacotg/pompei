use super::Player;
use rand::{prelude::*, rng};

pub struct RandomPlayer;

impl Player for RandomPlayer {
    fn play(&self, board: &crate::game::board::Board) -> crate::game::turn::Turn {
        let possible_moves = board.possible_move();

        let turn = possible_moves.choose(&mut rng()).unwrap().clone();
        turn
    }
}
