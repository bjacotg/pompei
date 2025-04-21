use std::fmt::Display;

use itertools::iproduct;

use super::error;

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub construction: Construction,
    pub player: Option<Player>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn get_neighbors(self) -> Vec<Position> {
        iproduct!(0..5, 0..5)
            .map(|(row, col)| Position::new(row, col))
            .filter(|&pos| Position::are_neighbors(pos, self))
            .collect()
    }

    pub fn are_neighbors(p1: Position, p2: Position) -> bool {
        p1 != p2
            && (p1.row as i32 - p2.row as i32).abs() <= 1
            && (p1.col as i32 - p2.col as i32).abs() <= 1
    }

    pub fn up(&mut self) {
        self.row = (self.row + 4) % 5;
    }

    pub fn down(&mut self) {
        self.row = (self.row + 1) % 5;
    }

    pub fn left(&mut self) {
        self.col = (self.col + 4) % 5;
    }

    pub fn right(&mut self) {
        self.col = (self.col + 1) % 5;
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            construction: Construction::GroundLevel,
            player: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Construction {
    GroundLevel,
    FirstLevel,
    SecondLevel,
    ThirdLevel,
    Dome,
}

impl Construction {
    pub fn build(self) -> error::Result<Construction> {
        use Construction::*;
        match self {
            GroundLevel => Ok(FirstLevel),
            FirstLevel => Ok(SecondLevel),
            SecondLevel => Ok(ThirdLevel),
            ThirdLevel => Ok(Dome),
            Dome => Err(error::GameError::InvalidMove),
        }
    }

    pub fn can_move(self, next: Construction) -> bool {
        match (self, next) {
            (_, Construction::Dome) => false,
            (Construction::SecondLevel, Construction::ThirdLevel) => true,
            (_, Construction::ThirdLevel) => false,
            (Construction::GroundLevel, Construction::SecondLevel) => false,
            _ => true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Player {
    Player1,
    Player2,
}

impl Player {
    pub fn other_player(&self) -> Player {
        match self {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::Player1 => write!(f, "Player 1"),
            Player::Player2 => write!(f, "Player 2"),
        }
    }
}
