mod board_widget;

use board_widget::BoardWidget;
use ratatui::widgets::{Block, Borders};

use crate::game::prelude::Position;

pub fn draw(frame: &mut ratatui::Frame, game: &crate::game::Game, selection: Position) {
    let board_game_block = Block::default()
        .title(game.next_action())
        .borders(Borders::ALL);
    let board_area = board_game_block.inner(frame.area());

    frame.render_widget(board_game_block, frame.area());
    frame.render_widget(BoardWidget(game, selection), board_area);
}
