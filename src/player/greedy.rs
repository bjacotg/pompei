use super::Player;

pub struct Greedy<Eval> {
    pub eval: Eval,
}

impl<Eval: Fn(&crate::game::board::Board) -> i64> Player for Greedy<Eval> {
    fn play(&self, board: &crate::game::board::Board) -> crate::game::turn::Turn {
        let possible_moves = board.possible_move();

        possible_moves
            .iter()
            .max_by_key(|turn| (self.eval)(&board.action(turn).unwrap()))
            .unwrap()
            .clone()
    }
}

pub fn elevation(board: &crate::game::board::Board) -> i64 {
    let player = board.current_player().other_player();
    let meeples = board.get_player_meeple(player);
    (board.first_floor.intersection(meeples).len()
        + board.second_floor.intersection(meeples).len() * 2
        + board.third_floor.intersection(meeples).len() * 3) as i64
}
