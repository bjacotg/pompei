mod greedy;
mod random;

use crate::game::{board::Board, turn::Turn};

pub trait Player {
    fn play(&self, board: &Board) -> Turn;
}

pub const PLAYER_TYPE: [&str; 3] = ["Human", "Random", "Greedy hill climber"];
pub type PlayerOrHuman = Option<Box<dyn Player>>;

pub fn get_player_from_selection(selection: usize) -> PlayerOrHuman {
    match selection {
        0 => None,
        1 => Some(Box::new(random::RandomPlayer)),
        2 => Some(Box::new(greedy::Greedy {
            eval: greedy::elevation,
        })),
        _ => panic!("Not possible"),
    }
}
