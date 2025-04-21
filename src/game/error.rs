use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum GameError {
    #[error("invalid move")]
    InvalidMove,
}

pub type Result<T> = std::result::Result<T, GameError>;
