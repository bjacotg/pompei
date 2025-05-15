mod random;

use crate::game::{board::Board, turn::Turn};

pub trait Player {
    fn play(&self, board: &Board) -> Turn;
}

pub const PLAYER_TYPE: [&str; 2] = ["Human", "Random"];
pub type PlayerOrHuman = Option<Box<dyn Player>>;

pub fn get_player_from_selection(selection: usize) -> PlayerOrHuman {
    match selection {
        0 => None,
        1 => Some(Box::new(random::RandomPlayer)),
        _ => panic!("Not possible"),
    }
}
