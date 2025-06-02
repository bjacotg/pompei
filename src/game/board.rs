use super::*;

#[derive(Clone, Debug)]
pub struct Board {
    player1_meeple: PositionSet,
    player2_meeple: PositionSet,
    first_floor: PositionSet,
    second_floor: PositionSet,
    third_floor: PositionSet,
    dome: PositionSet,
    pub next_player: Player,
}

impl Board {
    pub fn new() -> Self {
        Self {
            player1_meeple: PositionSet::new(),
            player2_meeple: PositionSet::new(),
            first_floor: PositionSet::new(),
            second_floor: PositionSet::new(),
            third_floor: PositionSet::new(),
            dome: PositionSet::new(),
            next_player: Player::Player1,
        }
    }

    fn get_construction(&self, position: Position) -> Construction {
        if self.first_floor.contains(position) {
            Construction::FirstLevel
        } else if self.second_floor.contains(position) {
            Construction::SecondLevel
        } else if self.third_floor.contains(position) {
            Construction::ThirdLevel
        } else if self.dome.contains(position) {
            Construction::Dome
        } else {
            Construction::GroundLevel
        }
    }

    pub fn get_tile(&self, position: Position) -> Tile {
        let player = if self.player1_meeple.contains(position) {
            Some(Player::Player1)
        } else if self.player2_meeple.contains(position) {
            Some(Player::Player2)
        } else {
            None
        };

        Tile {
            construction: self.get_construction(position),
            player,
        }
    }

    fn get_player_meeple_mut(&mut self, player: Player) -> &mut PositionSet {
        match player {
            Player::Player1 => &mut self.player1_meeple,
            Player::Player2 => &mut self.player2_meeple,
        }
    }

    fn get_player_meeple(&self, player: Player) -> PositionSet {
        match player {
            Player::Player1 => self.player1_meeple,
            Player::Player2 => self.player2_meeple,
        }
    }

    fn get_meeple(&self) -> PositionSet {
        self.player1_meeple.union(self.player2_meeple)
    }

    pub fn place_worker(&self, p1: Position, p2: Position) -> error::Result<Self> {
        let mut new_board = self.clone();
        let other_player_meeple = new_board.get_player_meeple(self.next_player.other_player());
        if other_player_meeple.contains(p1) || other_player_meeple.contains(p2) {
            return Err(error::GameError::InvalidMove);
        }
        let player_meeple = new_board.get_player_meeple_mut(self.next_player);
        if !player_meeple.is_empty() {
            return Err(error::GameError::InvalidMove);
        }
        *player_meeple = [p1, p2].into();
        new_board.next_player = self.next_player.other_player();
        Ok(new_board)
    }

    pub fn action(&self, turn: &turn::Turn) -> error::Result<Self> {
        let (start, end, build) = match turn {
            Turn::Setup(p1, p2) => {
                if self.get_player_meeple(self.next_player).is_empty() {
                    return self.place_worker(*p1, *p2);
                } else {
                    return Err(error::GameError::InvalidMove);
                }
            }
            Turn::MoveBuild { start, end, build } => (*start, *end, Some(*build)),
            Turn::FinalMove { start, end } => (*start, *end, None),
        };
        if !Position::are_neighbors(start, end) {
            return Err(error::GameError::InvalidMove);
        }

        if !self.get_player_meeple(self.next_player).contains(start) {
            return Err(error::GameError::InvalidMove);
        }

        if self.get_meeple().contains(end) || self.dome.contains(end) {
            return Err(error::GameError::InvalidMove);
        }
        if !self.third_floor.contains(end) && build.is_none() {
            return Err(error::GameError::InvalidMove);
        }

        let mut new_board = self.clone();
        let player_meeple = new_board.get_player_meeple_mut(self.next_player);
        player_meeple.remove(start);
        player_meeple.add(end);

        if let Some(construction_position) = build {
            if !Position::are_neighbors(end, construction_position) {
                return Err(error::GameError::InvalidMove);
            }
            if new_board.get_meeple().contains(construction_position) {
                return Err(error::GameError::InvalidMove);
            }
            new_board.build(construction_position)?;
        }
        new_board.next_player = self.next_player.other_player();
        Ok(new_board)
    }

    fn build(&mut self, position: Position) -> error::Result<()> {
        if self.dome.contains(position) {
            return Err(error::GameError::InvalidMove);
        } else if self.third_floor.contains(position) {
            self.third_floor.remove(position);
            self.dome.add(position);
        } else if self.second_floor.contains(position) {
            self.second_floor.remove(position);
            self.third_floor.add(position);
        } else if self.first_floor.contains(position) {
            self.first_floor.remove(position);
            self.second_floor.add(position);
        } else {
            self.first_floor.add(position);
        }
        Ok(())
    }

    pub fn get_tiles(&self) -> impl Iterator<Item = (Position, Tile)> {
        ALL_POSITIONS
            .into_iter()
            .map(|pos| (pos, self.get_tile(pos)))
    }

    pub fn possible_move(&self) -> Vec<turn::Turn> {
        let mut acc = vec![];
        if !self.get_player_meeple(self.next_player).is_empty() {
            for orig_pos in self.get_player_meeple(self.next_player) {
                for possible_move in orig_pos.get_neighbors().difference(self.get_meeple()) {
                    if !self
                        .get_construction(orig_pos)
                        .can_move(self.get_construction(possible_move))
                    {
                        continue;
                    }
                    if self.third_floor.contains(possible_move) {
                        acc.push(turn::Turn::FinalMove {
                            start: orig_pos,
                            end: possible_move,
                        });
                    } else {
                        for possible_build in possible_move
                            .get_neighbors()
                            .difference(self.dome)
                            .difference(self.get_meeple())
                            .union([orig_pos].into())
                        {
                            acc.push(turn::Turn::MoveBuild {
                                start: orig_pos,
                                end: possible_move,
                                build: possible_build,
                            });
                        }
                    }
                }
            }
        } else {
            let empty_spot =
                ALL_POSITIONS.difference(self.get_player_meeple(self.next_player.other_player()));
            for pos1 in empty_spot {
                for pos2 in empty_spot {
                    acc.push(turn::Turn::Setup(pos1, pos2));
                }
            }
        }
        acc
    }

    pub fn setup_done(&self) -> bool {
        self.get_meeple().len() == 4
    }

    pub fn current_player(&self) -> Player {
        self.next_player
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let board = Board::new();
        for (_, tile) in board.get_tiles() {
            assert_eq!(tile.construction, Construction::GroundLevel);
            assert!(tile.player.is_none())
        }
    }

    #[test]
    fn place_worker_empty() {
        let p1 = Position::new(1, 2);
        let p2 = Position::new(3, 2);
        let board = Board::new().place_worker(p1, p2).unwrap();
        assert_eq!(board.get_tile(p1).player, Some(Player::Player1));
        assert_eq!(board.get_tile(p2).player, Some(Player::Player1));
    }
    #[test]
    fn place_worker_non_empty() {
        let p1 = Position::new(1, 2);
        let p2 = Position::new(3, 2);
        let board = Board::new().place_worker(p1, p2).unwrap();

        assert_eq!(
            board.place_worker(p1, p2).err(),
            Some(error::GameError::InvalidMove)
        );
    }

    #[test]
    fn action() {
        let mut board = Board::new();
        board.player1_meeple.add(Position::new(1, 2));

        let new_board = board
            .action(&turn::Turn::MoveBuild {
                start: Position::new(1, 2),
                end: Position::new(2, 2),
                build: Position::new(2, 3),
            })
            .unwrap();
        assert_eq!(new_board.get_tile(Position::new(1, 2)).player, None);
        assert_eq!(
            new_board.get_tile(Position::new(2, 2)).player,
            Some(Player::Player1)
        );
        assert_eq!(
            new_board.get_tile(Position::new(2, 3)).construction,
            Construction::FirstLevel
        );
    }
    #[test]
    fn possible_move() {
        let occupied = Position::new(2, 2);
        let mut board = Board::new();
        board.player1_meeple.add(occupied);
        let possible_moves = board.possible_move();
        assert_eq!(possible_moves.len(), 8 * 8);
        for possible_move in possible_moves {
            assert!(board.action(&possible_move).is_ok());
        }
    }

    #[test]
    fn are_neighbors() {
        assert!(Position::are_neighbors(
            Position::new(1, 2),
            Position::new(2, 3)
        ));
        assert!(!Position::are_neighbors(
            Position::new(1, 2),
            Position::new(3, 3)
        ));
    }
    #[test]
    fn get_neighbors() {
        let occupied = Position::new(1, 1);
        let neighbors = occupied.get_neighbors();

        assert_eq!(neighbors.len(), 8);
        for neighbor in neighbors {
            assert!(Position::are_neighbors(occupied, neighbor));
        }
    }
}
