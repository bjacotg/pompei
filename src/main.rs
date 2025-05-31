#![allow(dead_code)]
#![feature(isolate_most_least_significant_one)]

use control::{Message, handle_event};
use game::prelude::Position;

mod control;
mod game;
mod player;
mod ui;

fn main() {
    let mut terminal = ratatui::init();

    let (player1, player2) = ui::menu_widget::player_selection_menu(&mut terminal);
    let mut game = game::Game::new();
    let mut selected_tile = Position::new(0, 0);
    let winner = loop {
        terminal
            .draw(|frame| ui::draw(frame, &game, selected_tile))
            .expect("failed to draw frame");
        let current_player = match game.board().current_player() {
            game::prelude::Player::Player1 => player1.as_ref(),
            game::prelude::Player::Player2 => player2.as_ref(),
        };

        if let Some(player) = current_player {
            let turn = player.play(game.board());
            game.play(turn);
        } else {
            let Some(message) = handle_event() else {
                continue;
            };
            match message {
                Message::Select => select(&mut game, selected_tile),
                Message::Up => selected_tile.up(),
                Message::Down => selected_tile.down(),
                Message::Left => selected_tile.left(),
                Message::Right => selected_tile.right(),
                Message::Quit => break None,
                Message::Reset => game.cancel(),
            };
        }
        if let Some(winner) = game.winner() {
            break Some(winner);
        }
    };
    ratatui::restore();
    match winner {
        Some(winner) => println!("{winner} won!"),
        None => println!("Game interrupted"),
    }
}

fn select(game: &mut game::Game, position: Position) {
    if !game.selectable().contains(position) {
        return;
    }
    game.register_selection(position);
}
