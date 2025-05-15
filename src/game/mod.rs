pub mod board;
pub mod error;
pub mod prelude;
pub mod turn;

use std::collections::HashSet;

use prelude::*;
use turn::Turn;

use self::turn::PartialTurn;

#[derive(Debug)]
pub struct Game {
    board: board::Board,
    current_turn: PartialTurn,
    selectable: HashSet<Position>,
}

impl Game {
    pub fn new() -> Self {
        let selectable = board::Board::new()
            .get_tiles()
            .map(|(position, _)| position)
            .collect();
        Self {
            board: board::Board::new(),
            current_turn: PartialTurn::NothingSetup,
            selectable,
        }
    }
    fn reset_selectable(&mut self) {
        let possible_moves = self.board.possible_move();
        self.selectable = match self.current_turn {
            PartialTurn::Nothing => possible_moves
                .iter()
                .map(|turn| match turn {
                    Turn::Setup(_, _) => panic!("Not possible"),
                    Turn::MoveBuild { start, .. } => *start,
                    Turn::FinalMove { start, .. } => *start,
                })
                .collect(),
            PartialTurn::Selection(s) => possible_moves
                .iter()
                .filter_map(|turn| match turn {
                    Turn::Setup(_, _) => panic!("Not possible"),
                    Turn::MoveBuild { start, end, .. } if *start == s => Some(*end),
                    Turn::FinalMove { start, end } if *start == s => Some(*end),
                    _ => None,
                })
                .collect(),
            PartialTurn::Move(s, e) => {
                if matches!(
                    self.board.get_tile(e).construction,
                    Construction::ThirdLevel
                ) {
                    HashSet::new()
                } else {
                    possible_moves
                        .iter()
                        .filter_map(|turn| match turn {
                            Turn::MoveBuild { start, end, build } if s == *start && e == *end => {
                                Some(*build)
                            }
                            _ => None,
                        })
                        .collect()
                }
            }
            PartialTurn::NothingSetup => self
                .board
                .get_tiles()
                .filter(|&(_, tile)| tile.player.is_none())
                .map(|(position, _)| position)
                .collect(),
            PartialTurn::PartialSetup(first) => self
                .board
                .get_tiles()
                .filter(|&(position, tile)| position != first && tile.player.is_none())
                .map(|(position, _)| position)
                .collect(),
        };
    }

    pub fn selectable(&self) -> &HashSet<Position> {
        &self.selectable
    }

    pub fn cancel(&mut self) {
        self.current_turn = match self.current_turn {
            PartialTurn::NothingSetup | PartialTurn::PartialSetup(_) => PartialTurn::NothingSetup,
            _ => PartialTurn::Nothing,
        };
        self.reset_selectable();
    }

    pub fn register_selection(&mut self, selection: Position) {
        match self.current_turn {
            PartialTurn::Nothing => {
                self.current_turn = PartialTurn::Selection(selection);
            }
            PartialTurn::Selection(start) => {
                self.current_turn = PartialTurn::Move(start, selection);
            }
            PartialTurn::Move(start, end) => {
                self.board = self
                    .board
                    .action(&turn::Turn::MoveBuild {
                        start,
                        end,
                        build: selection,
                    })
                    .unwrap();
                self.current_turn = PartialTurn::Nothing;
            }
            PartialTurn::NothingSetup => {
                self.current_turn = PartialTurn::PartialSetup(selection);
            }
            PartialTurn::PartialSetup(first) => {
                self.board = self.board.place_worker(first, selection).unwrap();
                self.current_turn = match self.board.current_player() {
                    Player::Player1 => PartialTurn::Nothing,
                    Player::Player2 => PartialTurn::NothingSetup,
                };
            }
        };
        self.reset_selectable();
    }

    pub fn selected(&self) -> Vec<Position> {
        match self.current_turn {
            PartialTurn::Nothing => vec![],
            PartialTurn::Selection(start) => vec![start],
            PartialTurn::Move(start, end) => vec![start, end],
            PartialTurn::NothingSetup => vec![],
            PartialTurn::PartialSetup(first) => vec![first],
        }
    }

    pub fn winner(&self) -> Option<Player> {
        match self.current_turn {
            PartialTurn::Move(_, end)
                if matches!(
                    self.board.get_tile(end).construction,
                    Construction::ThirdLevel
                ) =>
            {
                Some(self.board.next_player)
            }
            PartialTurn::Nothing if self.selectable().is_empty() => {
                Some(self.board.next_player.other_player())
            }
            _ => None,
        }
    }

    pub fn next_action(&self) -> String {
        let action = match self.current_turn {
            crate::game::turn::PartialTurn::Nothing => "Pick a worker!",
            crate::game::turn::PartialTurn::Selection(_) => "Move your worker!",
            crate::game::turn::PartialTurn::Move(_, _) => "Build!",
            crate::game::turn::PartialTurn::NothingSetup => "Place your first worker!",
            crate::game::turn::PartialTurn::PartialSetup(_) => "Place your second worker!",
        };
        format!("{}: {action}", self.board.next_player)
    }

    pub fn board(&self) -> &board::Board {
        &self.board
    }

    pub fn play(&mut self, turn: Turn) {
        self.board = self.board.action(&turn).unwrap();
        self.current_turn = if self
            .board
            .get_tiles()
            .any(|(_, t)| t.player == Some(self.board.next_player))
        {
            crate::game::turn::PartialTurn::Nothing
        } else {
            crate::game::turn::PartialTurn::NothingSetup
        };
        self.reset_selectable();
    }
}
