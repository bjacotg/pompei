#![allow(dead_code)]

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use game::prelude::Position;
use ratatui::{DefaultTerminal, widgets::ListState};

mod game;
mod player;

mod ui;

fn main() {
    let mut terminal = ratatui::init();

    let (_player1, _player2) = player_selection_menu(&mut terminal);
    let mut game = game::Game::new();
    let mut selected_tile = Position::new(0, 0);
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

fn player_selection_menu(terminal: &mut DefaultTerminal) -> (usize, usize) {
    let mut list_state = ListState::default().with_selected(Some(0));
    let player1_selection = loop {
        terminal
            .draw(|frame| {
                let area = frame.area();
                frame.render_stateful_widget(
                    ui::menu_widget::MenuWidget { player1: None },
                    area,
                    &mut list_state,
                );
            })
            .expect("failed to draw frame");
        let Some(message) = handle_event() else {
            continue;
        };

        match message {
            Message::Up => {
                list_state.select_previous();
            }
            Message::Down => {
                list_state.select_next();
            }
            Message::Select => {
                break list_state.selected().unwrap();
            }
            _ => {}
        }
    };

    let mut list_state = ListState::default().with_selected(Some(0));
    loop {
        terminal
            .draw(|frame| {
                let area = frame.area();
                frame.render_stateful_widget(
                    ui::menu_widget::MenuWidget {
                        player1: Some(player1_selection),
                    },
                    area,
                    &mut list_state,
                );
            })
            .expect("failed to draw frame");
        let Some(message) = handle_event() else {
            continue;
        };

        match message {
            Message::Up => {
                list_state.select_previous();
            }
            Message::Down => {
                list_state.select_next();
            }
            Message::Select => {
                return (player1_selection, list_state.selected().unwrap());
            }
            _ => {}
        }
    }
}
