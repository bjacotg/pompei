use super::*;

#[derive(Debug)]
pub enum PartialTurn {
    Nothing,
    Selection(Position),
    Move(Position, Position),
    NothingSetup,
    PartialSetup(Position),
}

// #[derive(Debug, Clone)]
// pub struct Turn {
//     pub start: Position,
//     pub end: Position,
//     pub build: Option<Position>,
// }

#[derive(Debug, Clone)]
pub enum Turn {
    Setup(Position, Position),
    MoveBuild {
        start: Position,
        end: Position,
        build: Position,
    },
    FinalMove {
        start: Position,
        end: Position,
    },
}
