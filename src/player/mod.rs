mod random;

use crate::game::{board::Board, turn::Turn};

trait Player {
    fn play(&self, board: &Board) -> Turn;
}
