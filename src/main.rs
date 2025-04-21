#![allow(dead_code)]

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use game::prelude::Position;

mod game;

mod ui;

fn main() {
    let mut game = game::Game::new();
    let mut selected_tile = Position::new(0, 0);
    let mut terminal = ratatui::init();
    let winner = loop {
        terminal
            .draw(|frame| ui::draw(frame, &game, selected_tile))
            .expect("failed to draw frame");
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

enum Message {
    Select,
    Up,
    Down,
    Left,
    Right,
    Quit,
    Reset,
}

fn handle_event() -> Option<Message> {
    let Ok(Event::Key(KeyEvent { code, .. })) = event::read() else {
        return None;
    };

    match code {
        KeyCode::Char('q') => Some(Message::Quit),
        KeyCode::Char('h') | KeyCode::Left => Some(Message::Left),
        KeyCode::Char('j') | KeyCode::Down => Some(Message::Down),
        KeyCode::Char('k') | KeyCode::Up => Some(Message::Up),
        KeyCode::Char('l') | KeyCode::Right => Some(Message::Right),
        KeyCode::Enter | KeyCode::Char(' ') => Some(Message::Select),
        KeyCode::Esc => Some(Message::Reset),
        _ => None,
    }
}

fn select(game: &mut game::Game, position: Position) {
    if !game.selectable().contains(&position) {
        return;
    }
    game.register_selection(position);
}
