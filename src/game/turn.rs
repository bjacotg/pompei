use super::*;

#[derive(Debug)]
pub enum PartialTurn {
    Nothing,
    Selection(Position),
    Move(Position, Position),
    NothingSetup,
    PartialSetup(Position),
}

#[derive(Debug)]
pub struct Turn {
    pub start: Position,
    pub end: Position,
    pub build: Option<Position>,
}
