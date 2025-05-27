use std::fmt::Display;

use super::error;

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub construction: Construction,
    pub player: Option<Player>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position(u32);

// Sentinel of impossible position.
const SENTINEL: u32 = 0b11_100000_100000_100000_100000_100000;
fn bad_position(p: u32) -> bool {
    p == 0 || p & SENTINEL != 0
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Self(1 << (row * 6 + col))
    }

    pub fn row(self) -> usize {
        (self.0.ilog2() / 6) as usize
    }

    pub fn col(self) -> usize {
        (self.0.ilog2() % 6) as usize
    }

    pub fn get_neighbors(self) -> Vec<Position> {
        [
            self.0 << 1,
            self.0 >> 1,
            self.0 << 6,
            self.0 >> 6,
            self.0 << 5,
            self.0 << 7,
            self.0 >> 5,
            self.0 >> 7,
        ]
        .iter()
        .filter(|&&bm| !bad_position(bm))
        .map(|&bm| Self(bm))
        .collect()
    }

    pub fn are_neighbors(p1: Position, p2: Position) -> bool {
        let p1 = p1.0.ilog2();
        let p2 = p2.0.ilog2();
        let diff = p1.abs_diff(p2);
        matches!(diff, 1 | 5 | 6 | 7)
    }

    pub fn up(&mut self) {
        let new = self.0 >> 6;
        self.0 = if bad_position(new) { self.0 << 24 } else { new };
    }

    pub fn down(&mut self) {
        let new = self.0 << 6;
        self.0 = if bad_position(new) { self.0 >> 24 } else { new };
    }

    pub fn left(&mut self) {
        let new = self.0 >> 1;
        self.0 = if bad_position(new) { self.0 << 4 } else { new };
    }

    pub fn right(&mut self) {
        let new = self.0 << 1;
        self.0 = if bad_position(new) { self.0 >> 4 } else { new };
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

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[test]
    fn get_neighbord() {
        assert_that!(
            Position::new(0, 0).get_neighbors(),
            {
                &Position::new(0, 1),
                &Position::new(1, 0),
                &Position::new(1, 1)
            }
        );

        assert_that!(
            Position::new(0, 1).get_neighbors(),
            {
                &Position::new(0, 0),
                &Position::new(1, 0),
                &Position::new(1, 1),
                &Position::new(0, 2),
                &Position::new(1, 2)
            }
        );

        assert_that!(
            Position::new(1, 1).get_neighbors(),
            {
                &Position::new(0, 0),
                &Position::new(1, 0),
                &Position::new(2, 0),
                &Position::new(0, 1),
                &Position::new(2, 1),
                &Position::new(0, 2),
                &Position::new(1, 2),
                &Position::new(2, 2)
            }
        );
    }

    #[test]
    fn are_neighbors() {
        assert!(Position::are_neighbors(
            Position::new(1, 1),
            Position::new(1, 2)
        ));

        assert!(Position::are_neighbors(
            Position::new(1, 1),
            Position::new(2, 2)
        ));

        assert!(!Position::are_neighbors(
            Position::new(2, 2),
            Position::new(2, 2)
        ));
        assert!(!Position::are_neighbors(
            Position::new(2, 2),
            Position::new(4, 2)
        ));
    }

    #[test]
    fn neighbors() {
        for r1 in 0..5 {
            for c1 in 0..5 {
                let pos1 = Position::new(r1, c1);
                let n = pos1.get_neighbors();
                for r2 in 0..5 {
                    for c2 in 0..5 {
                        let pos2 = Position::new(r2, c2);
                        let is_neigh = Position::are_neighbors(pos1, pos2);
                        assert_eq!(is_neigh, Position::are_neighbors(pos2, pos1));
                        assert_eq!(is_neigh, n.contains(&pos2));
                    }
                }
            }
        }
    }

    #[test]
    fn moves() {
        for r1 in 0..5 {
            for c1 in 0..5 {
                let pos1 = Position::new(r1, c1);
                let mut p = pos1;
                p.up();
                p.down();
                assert_eq!(pos1, p);
                p.left();
                p.right();
                assert_eq!(pos1, p);
                p.up();
                p.left();
                p.down();
                p.right();
                assert_eq!(pos1, p);
            }
        }
    }
}
