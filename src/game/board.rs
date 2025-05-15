use super::*;

#[derive(Clone, Debug)]
pub struct Board {
    tiles: [[Tile; 5]; 5],
    pub next_player: Player,
}

impl Board {
    pub fn new() -> Self {
        Self {
            tiles: [[Tile::default(); 5]; 5],
            next_player: Player::Player1,
        }
    }

    pub fn get_tile(&self, position: Position) -> &Tile {
        &self.tiles[position.row][position.col]
    }

    fn get_tile_mut(&mut self, position: Position) -> &mut Tile {
        &mut self.tiles[position.row][position.col]
    }

    pub fn place_worker(&self, p1: Position, p2: Position) -> error::Result<Self> {
        let mut new_board = self.clone();
        let tile = new_board.get_tile_mut(p1);
        match tile.player {
            Some(_) => return Err(error::GameError::InvalidMove),
            None => {
                tile.player = Some(self.next_player);
            }
        }
        let tile = new_board.get_tile_mut(p2);
        match tile.player {
            Some(_) => return Err(error::GameError::InvalidMove),
            None => {
                tile.player = Some(self.next_player);
            }
        }
        new_board.next_player = self.next_player.other_player();
        Ok(new_board)
    }

    pub fn action(&self, turn: &turn::Turn) -> error::Result<Self> {
        let (start, end, build) = match turn {
            Turn::Setup(position, position1) => {
                if self
                    .get_tiles()
                    .any(|(_, &t)| t.player == Some(self.next_player))
                {
                    return Err(error::GameError::InvalidMove);
                } else {
                    let mut new_board = self.clone();
                    new_board.get_tile_mut(*position).player = Some(self.next_player);
                    new_board.get_tile_mut(*position1).player = Some(self.next_player);
                    new_board.next_player = self.next_player.other_player();
                    return Ok(new_board);
                }
            }
            Turn::MoveBuild { start, end, build } => (*start, *end, Some(*build)),
            Turn::FinalMove { start, end } => (*start, *end, None),
        };
        if !Position::are_neighbors(start, end) {
            return Err(error::GameError::InvalidMove);
        }
        if self.get_tile(start).player != Some(self.next_player) {
            return Err(error::GameError::InvalidMove);
        }
        let move_tile = self.get_tile(end);
        if move_tile.player.is_some() || move_tile.construction == Construction::Dome {
            return Err(error::GameError::InvalidMove);
        }
        if move_tile.construction != Construction::ThirdLevel && build.is_none() {
            return Err(error::GameError::InvalidMove);
        }
        let mut new_board = self.clone();
        new_board.get_tile_mut(start).player = None;
        new_board.get_tile_mut(end).player = Some(self.next_player);
        if let Some(construction_position) = build {
            if !Position::are_neighbors(end, construction_position) {
                return Err(error::GameError::InvalidMove);
            }
            let construction_tile = new_board.get_tile_mut(construction_position);
            if construction_tile.player.is_some() {
                return Err(error::GameError::InvalidMove);
            }
            construction_tile.construction = construction_tile.construction.build()?;
        }
        new_board.next_player = self.next_player.other_player();
        Ok(new_board)
    }

    pub fn get_tiles(&self) -> impl Iterator<Item = (Position, &Tile)> {
        self.tiles.iter().enumerate().flat_map(|(row, tiles_row)| {
            tiles_row
                .iter()
                .enumerate()
                .map(move |(col, tile)| (Position::new(row, col), tile))
        })
    }

    pub fn possible_move(&self) -> Vec<turn::Turn> {
        let mut acc = vec![];
        if self
            .get_tiles()
            .any(|(_, tile)| tile.player == Some(self.next_player))
        {
            for (orig_pos, orig_tile) in self.get_tiles() {
                if orig_tile.player != Some(self.next_player) {
                    continue;
                }
                for possible_move in orig_pos.get_neighbors() {
                    let move_tile = self.get_tile(possible_move);
                    if !orig_tile.construction.can_move(move_tile.construction) {
                        continue;
                    }
                    if move_tile.player.is_some() {
                        continue;
                    }
                    if move_tile.construction == Construction::ThirdLevel {
                        acc.push(turn::Turn::FinalMove {
                            start: orig_pos,
                            end: possible_move,
                        });
                    } else {
                        for possible_build in possible_move.get_neighbors() {
                            let build_tile = self.get_tile(possible_build);
                            if build_tile.construction == Construction::Dome {
                                continue;
                            }
                            if build_tile.player.is_some() && possible_build != orig_pos {
                                continue;
                            }

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
            let empty_spot = self
                .get_tiles()
                .filter_map(|(pos, tile)| {
                    if tile.player.is_none() {
                        Some(pos)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            for pos1 in &empty_spot {
                for pos2 in &empty_spot {
                    acc.push(turn::Turn::Setup(*pos1, *pos2));
                }
            }
        }
        acc
    }

    pub fn setup_done(&self) -> bool {
        self.get_tiles()
            .filter(|(_, tile)| tile.player.is_some())
            .count()
            == 4
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
        for row in board.tiles.iter() {
            for tile in row.iter() {
                assert_eq!(tile.construction, Construction::GroundLevel);
                assert!(tile.player.is_none())
            }
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
        board.get_tile_mut(Position::new(1, 2)).player = Some(Player::Player1);

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
        board.get_tile_mut(occupied).player = Some(Player::Player1);
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
